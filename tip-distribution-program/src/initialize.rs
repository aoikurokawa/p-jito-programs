use jito_tip_core::{create_account, loader::load_system_program};
use jito_tip_distribution_core::{config::Config, load_mut_unchecked, Transmutable};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
};

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

    load_system_program(system_program_info)?;

    let rent = Rent::get()?;
    let space = Config::LEN;
    let seeds = Config::seeds();
    let seeds: Vec<&[u8]> = seeds.iter().map(|seed| seed.as_slice()).collect();
    let (_merkle_root_upload_config_pubkey, merkle_root_upload_config_bump) =
        find_program_address(&seeds, program_id);

    let bindings = [merkle_root_upload_config_bump];
    let seeds = [Seed::from(Config::SEED), Seed::from(&bindings)];
    let signers = [Signer::from(&seeds)];

    create_account(
        initializer_info,
        config_info,
        system_program_info,
        program_id,
        &rent,
        space as u64,
        &signers,
    )?;

    let cfg = unsafe { load_mut_unchecked::<Config>(config_info.borrow_mut_data_unchecked())? };

    cfg.authority = authority;
    cfg.expired_funds_account = expired_funds_account;
    cfg.num_epochs_valid = num_epochs_valid;
    cfg.max_validator_commission_bps = max_validator_commission_bps;
    cfg.bump = bump;

    cfg.validate()?;

    Ok(())
}
