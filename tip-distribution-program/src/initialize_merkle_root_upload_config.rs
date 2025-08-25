use jito_tip_core::{
    create_account,
    loader::{load_signer, load_system_program},
};
use jito_tip_distribution_core::{
    config::Config, load_mut_unchecked, load_unchecked,
    merkle_root_upload_config::MerkleRootUploadConfig, Transmutable,
};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
};

pub fn process_initialize_merkle_root_upload_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority: Pubkey,
    original_authority: Pubkey,
) -> Result<(), ProgramError> {
    let [config_info, merkle_root_upload_config_info, authority_info, payer_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    unsafe {
        Config::load(program_id, config_info, true)?;
    }
    let config = unsafe { load_unchecked::<Config>(config_info.borrow_data_unchecked())? };

    // Call the authorize function
    if config.authority.ne(authority_info.key()) {
        return Err(TipDistributionError::Unauthorized.into());
    }

    load_signer(authority_info, true)?;
    load_signer(payer_info, true)?;
    load_system_program(system_program_info)?;

    let rent = Rent::get()?;
    let space = MerkleRootUploadConfig::LEN;
    let seeds = MerkleRootUploadConfig::seeds();
    let seeds: Vec<&[u8]> = seeds.iter().map(|seed| seed.as_slice()).collect();
    let (_merkle_root_upload_config_pubkey, merkle_root_upload_config_bump) =
        find_program_address(&seeds, program_id);

    let bindings = [merkle_root_upload_config_bump];
    let seeds = [Seed::from(Config::SEED), Seed::from(&bindings)];
    let signers = [Signer::from(&seeds)];

    create_account(
        payer_info,
        merkle_root_upload_config_info,
        system_program_info,
        program_id,
        &rent,
        space as u64,
        &signers,
    )?;

    let merkle_root_upload_config = unsafe {
        load_mut_unchecked::<MerkleRootUploadConfig>(config_info.borrow_mut_data_unchecked())?
    };

    // Set the bump and override authority
    merkle_root_upload_config.override_authority = authority;
    merkle_root_upload_config.original_upload_authority = original_authority;
    merkle_root_upload_config.bump = merkle_root_upload_config_bump;

    Ok(())
}
