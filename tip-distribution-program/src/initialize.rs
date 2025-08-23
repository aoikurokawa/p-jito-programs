use jito_tip_distribution_core::{config::Config, load_mut_unchecked, Transmutable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
};
use pinocchio_system::instructions::CreateAccount;

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
    let [config, _system_program, initializer] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let rent = Rent::get()?;

    let space = Config::LEN;
    let required_lamports = rent.minimum_balance(space);

    let (_config_pubkey, config_bump) = find_program_address(&[Config::SEED], program_id);

    let bindings = [config_bump];
    let seeds = [Seed::from(Config::SEED), Seed::from(&bindings)];
    let signers = [Signer::from(&seeds)];
    CreateAccount {
        from: initializer,
        to: config,
        lamports: required_lamports,
        space: space as u64,
        owner: program_id,
    }
    .invoke_signed(&signers)?;

    let cfg = unsafe { load_mut_unchecked::<Config>(config.borrow_mut_data_unchecked())? };

    cfg.authority = authority;
    cfg.expired_funds_account = expired_funds_account;
    cfg.num_epochs_valid = num_epochs_valid;
    cfg.max_validator_commission_bps = max_validator_commission_bps;
    cfg.bump = bump;

    cfg.validate()?;

    Ok(())
}
