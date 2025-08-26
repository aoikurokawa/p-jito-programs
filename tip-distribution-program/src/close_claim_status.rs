use jito_tip_core::close_program_account;
use jito_tip_distribution_core::{claim_status::ClaimStatus, config::Config, load_unchecked};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{clock::Clock, Sysvar},
};

/// Anyone can invoke this only after the [TipDistributionAccount] has expired.
/// This instruction will return any rent back to `claimant` and close the account
pub fn process_close_claim_status(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let [config_info, claim_status_info, claim_status_payer_info, claimant_info, tip_distribution_account_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    unsafe {
        Config::load(program_id, config_info, false)?;
    }

    let claim_status = unsafe {
        ClaimStatus::load(
            program_id,
            claim_status_info,
            *claimant_info.key(),
            *tip_distribution_account_info.key(),
            false,
        )?;

        load_unchecked::<ClaimStatus>(&claim_status_info.borrow_data_unchecked()[8..])?
    };

    if claim_status
        .claim_status_payer
        .ne(claim_status_payer_info.key())
    {
        return Err(ProgramError::InvalidAccountData);
    }

    // can only claim after claim_status has expired to prevent draining.
    if Clock::get()?.epoch <= claim_status.expires_at {
        return Err(TipDistributionError::PrematureCloseClaimStatus.into());
    }

    unsafe {
        close_program_account(program_id, claim_status_info, claim_status_payer_info)?;
    }

    Ok(())
}
