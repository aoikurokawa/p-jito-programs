use jito_tip_core::{close_program_account, loader::load_signer};
use jito_tip_distribution_core::{
    config::Config, load_mut_unchecked, tip_distribution_account::TipDistributionAccount,
};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{clock::Clock, Sysvar},
};

/// Anyone can invoke this only after the [TipDistributionAccount] has expired.
/// This instruction will send any unclaimed funds to the designated `expired_funds_account`
/// before closing and returning the rent exempt funds to the validator.
pub fn process_close_tip_distribution_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let [config_info, expired_funds_account_info, tip_distribution_account_info, validator_vote_account_info, signer] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let current_epoch = Clock::get()?.epoch;

    unsafe {
        Config::load(program_id, config_info, false)?;
    }
    let config = unsafe { load_mut_unchecked::<Config>(config_info.borrow_mut_data_unchecked())? };

    unsafe {
        TipDistributionAccount::load(
            program_id,
            tip_distribution_account_info,
            validator_vote_account_info.key(),
            current_epoch,
            false,
        )?;
    }
    let tip_distribution_account = unsafe {
        load_mut_unchecked::<TipDistributionAccount>(
            tip_distribution_account_info.borrow_mut_data_unchecked(),
        )?
    };

    load_signer(signer, false)?;

    if config
        .expired_funds_account
        .ne(expired_funds_account_info.key())
    {
        return Err(TipDistributionError::Unauthorized.into());
    }

    if Clock::get()?.epoch <= tip_distribution_account.expires_at {
        return Err(TipDistributionError::PrematureCloseTipDistributionAccount.into());
    }

    let _expired_amount = TipDistributionAccount::claim_expired(
        tip_distribution_account_info,
        expired_funds_account_info,
    )?;

    tip_distribution_account.validate()?;

    unsafe {
        close_program_account(program_id, tip_distribution_account_info, signer)?;
    }

    Ok(())
}
