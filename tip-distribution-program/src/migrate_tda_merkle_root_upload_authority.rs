use jito_tip_distribution_core::{
    load_mut_unchecked, load_unchecked, merkle_root_upload_config::MerkleRootUploadConfig,
    tip_distribution_account::TipDistributionAccount,
};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

pub fn process_migrate_tda_merkle_root_upload_authority(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let [tip_distribution_account_info, merkle_root_upload_config_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let tip_distribution_account = unsafe {
        load_mut_unchecked::<TipDistributionAccount>(
            &mut tip_distribution_account_info.borrow_mut_data_unchecked()[8..],
        )?
    };

    let merkle_root_upload_config = unsafe {
        MerkleRootUploadConfig::load(program_id, merkle_root_upload_config_info, false)?;
        load_unchecked::<MerkleRootUploadConfig>(
            &merkle_root_upload_config_info.borrow_data_unchecked()[8..],
        )?
    };

    // Validate TDA has no MerkleRoot uploaded to it
    if tip_distribution_account.merkle_root.is_some() {
        return Err(TipDistributionError::InvalidTdaForMigration.into());
    }

    // Validate the TDA key is the acceptable original authority (i.e. the original Jito Lab's authority)
    if tip_distribution_account.merkle_root_upload_authority
        != merkle_root_upload_config.original_upload_authority
    {
        return Err(TipDistributionError::InvalidTdaForMigration.into());
    }

    // Change the TDA's root upload authority
    tip_distribution_account.merkle_root_upload_authority =
        merkle_root_upload_config.override_authority;

    Ok(())
}
