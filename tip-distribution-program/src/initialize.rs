use jito_tip_core::{
    create_account,
    loader::{load_signer, load_system_program},
    transmutable::Transmutable,
};
use jito_tip_distribution_core::{config::Config, load_mut_unchecked};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
};
use pinocchio_log::log;

/// Initialize a singleton instance of the [Config] account.
pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority: Pubkey,
    expired_funds_account: Pubkey,
    num_epochs_valid: u64,
    max_validator_commission_bps: u16,
    bump: u8,
) -> Result<(), ProgramError> {
    let [config_info, system_program_info, initializer_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(initializer_info, true)?;
    load_system_program(system_program_info)?;

    let rent = Rent::get()?;
    let space = 8usize
        .checked_add(Config::LEN)
        .ok_or(TipDistributionError::ArithmeticError)?;

    let (config_pubkey, config_bump, mut config_seeds) = Config::find_program_address(program_id);
    config_seeds.push(vec![config_bump]);
    if config_pubkey.ne(config_info.key()) {
        log!("Config account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    let seeds: Vec<Seed> = config_seeds
        .iter()
        .map(|seed| Seed::from(seed.as_slice()))
        .collect();
    let signers = [Signer::from(seeds.as_slice())];

    log!("Initializing Config at address {}", config_info.key());
    create_account(
        initializer_info,
        config_info,
        system_program_info,
        program_id,
        &rent,
        space as u64,
        &signers,
    )?;

    let cfg = unsafe {
        let config_data = config_info.borrow_mut_data_unchecked();
        config_data[0..8].copy_from_slice(Config::DISCRIMINATOR);
        load_mut_unchecked::<Config>(&mut config_data[8..])?
    };

    cfg.authority = authority;
    cfg.expired_funds_account = expired_funds_account;
    cfg.num_epochs_valid = num_epochs_valid;
    cfg.max_validator_commission_bps = max_validator_commission_bps;
    cfg.bump = bump;

    cfg.validate()?;

    Ok(())
}
