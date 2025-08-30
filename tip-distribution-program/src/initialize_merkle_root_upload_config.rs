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
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
};
use pinocchio_log::log;

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

    let config = unsafe {
        Config::load(program_id, config_info, false)?;
        load_unchecked::<Config>(&config_info.borrow_data_unchecked()[8..])?
    };

    // Call the authorize function
    if config.authority.ne(authority_info.key()) {
        return Err(TipDistributionError::Unauthorized.into());
    }

    load_signer(authority_info, true)?;
    load_signer(payer_info, true)?;
    load_system_program(system_program_info)?;

    let rent = Rent::get()?;
    let space = 8usize
        .checked_add(MerkleRootUploadConfig::LEN)
        .ok_or(TipDistributionError::ArithmeticError)?;

    let (
        merkle_root_upload_config_pubkey,
        merkle_root_upload_config_bump,
        mut merkle_root_upload_config_seed,
    ) = MerkleRootUploadConfig::find_program_address(program_id);
    merkle_root_upload_config_seed.push([merkle_root_upload_config_bump]);

    if merkle_root_upload_config_pubkey.ne(merkle_root_upload_config_info.key()) {
        log!("MerkleRootUploadConfig account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    let seeds: Vec<Seed> = merkle_root_upload_config_seed
        .iter()
        .map(|seed| Seed::from(seed.as_slice()))
        .collect();

    let signers = [Signer::from(seeds.as_slice())];

    log!(
        "Initializing MerkleRootUploadConfig at address {}",
        merkle_root_upload_config_info.key()
    );
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
        let merkle_root_upload_config_data =
            merkle_root_upload_config_info.borrow_mut_data_unchecked();
        merkle_root_upload_config_data[0..8].copy_from_slice(MerkleRootUploadConfig::DISCRIMINATOR);
        load_mut_unchecked::<MerkleRootUploadConfig>(&mut merkle_root_upload_config_data[8..])?
    };

    // Set the bump and override authority
    merkle_root_upload_config.override_authority = authority;
    merkle_root_upload_config.original_upload_authority = original_authority;
    merkle_root_upload_config.bump = merkle_root_upload_config_bump;

    Ok(())
}
