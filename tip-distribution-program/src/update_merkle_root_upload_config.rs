use jito_tip_core::loader::{load_signer, load_system_program};
use jito_tip_distribution_core::{
    config::Config, load_mut_unchecked, load_unchecked,
    merkle_root_upload_config::MerkleRootUploadConfig,
};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

/// Update merkle_root_upload_config fields. Only the [MerkleRootUploadConfig] authority can invoke this.
pub fn process_update_merkle_root_upload_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority: Pubkey,
    original_authority: Pubkey,
) -> Result<(), ProgramError> {
    let [config_info, merkle_root_upload_config_info, authority_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    unsafe {
        Config::load(program_id, config_info, false)?;
    }
    let config = unsafe { load_unchecked::<Config>(config_info.borrow_data_unchecked())? };

    // Call the authorize function
    if config.authority.ne(authority_info.key()) {
        return Err(TipDistributionError::Unauthorized.into());
    }

    load_signer(merkle_root_upload_config_info, false)?;
    load_system_program(system_program_info)?;

    unsafe {
        MerkleRootUploadConfig::load(program_id, config_info, true)?;
    }
    let merkle_root_upload_config = unsafe {
        load_mut_unchecked::<MerkleRootUploadConfig>(
            merkle_root_upload_config_info.borrow_mut_data_unchecked(),
        )?
    };

    // Update override authority
    merkle_root_upload_config.override_authority = authority;
    merkle_root_upload_config.original_upload_authority = original_authority;

    Ok(())
}
