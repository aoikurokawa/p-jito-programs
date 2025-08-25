use jito_tip_core::loader::load_signer;
use jito_tip_distribution_core::{config::Config, load_mut_unchecked};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{account_info::AccountInfo, program_error::ProgramError, pubkey::Pubkey};

/// Update config fields. Only the [Config] authority can invoke this.
pub fn process_update_config(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    authority: Pubkey,
    expired_funds_account: Pubkey,
    num_epochs_valid: u64,
    max_validator_commission_bps: u16,
) -> Result<(), ProgramError> {
    let [config_info, authority_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    unsafe {
        Config::load(program_id, config_info, true)?;
    }

    load_signer(authority_info, false)?;

    let config = unsafe { load_mut_unchecked::<Config>(config_info.borrow_mut_data_unchecked())? };

    if config.authority.ne(authority_info.key()) {
        return Err(TipDistributionError::Unauthorized.into());
    }

    config.authority = authority;
    config.expired_funds_account = expired_funds_account;
    config.num_epochs_valid = num_epochs_valid;
    config.max_validator_commission_bps = max_validator_commission_bps;

    config.validate()?;

    Ok(())
}
