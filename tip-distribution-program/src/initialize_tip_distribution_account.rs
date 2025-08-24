use jito_tip_distribution_core::{
    config::Config, load_mut_unchecked, load_unchecked,
    tip_distribution_account::TipDistributionAccount, Transmutable,
};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{clock::Clock, rent::Rent, Sysvar},
};
use pinocchio_system::instructions::CreateAccount;

/// Initialize a new [TipDistributionAccount] associated with the given validator vote key
/// and current epoch.
pub fn process_initialize_tip_distribution_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    merkle_root_upload_authority: Pubkey,
    validator_commission_bps: u16,
    bump: u8,
) -> Result<(), ProgramError> {
    let [config, tip_distribution_account, validator_vote_account, signer, _system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let cfg = unsafe { load_unchecked::<Config>(config.borrow_data_unchecked())? };

    if validator_commission_bps > cfg.max_validator_commission_bps {
        return Err(TipDistributionError::MaxValidatorCommissionFeeBpsExceeded.into());
    }

    // let validator_vote_state = VoteState::deserialize(&ctx.accounts.validator_vote_account)?;
    // if &validator_vote_state.node_pubkey != ctx.accounts.signer.key {
    //     return Err(Unauthorized.into());
    // }

    let current_epoch = Clock::get()?.epoch;
    let rent = Rent::get()?;

    let space = TipDistributionAccount::LEN;
    let required_lamports = rent.minimum_balance(space);

    let (_config_pubkey, tip_distribution_account_bump) =
        find_program_address(&[TipDistributionAccount::SEED], program_id);

    let bindings = [tip_distribution_account_bump];
    let seeds = [
        Seed::from(TipDistributionAccount::SEED),
        Seed::from(&bindings),
    ];
    let signers = [Signer::from(&seeds)];
    CreateAccount {
        from: signer,
        to: config,
        lamports: required_lamports,
        space: space as u64,
        owner: program_id,
    }
    .invoke_signed(&signers)?;

    let tip_distribution_account = unsafe {
        load_mut_unchecked::<TipDistributionAccount>(
            tip_distribution_account.borrow_mut_data_unchecked(),
        )?
    };

    tip_distribution_account.validator_vote_account = *validator_vote_account.key();
    tip_distribution_account.epoch_created_at = current_epoch;
    tip_distribution_account.validator_commission_bps = validator_commission_bps;
    tip_distribution_account.merkle_root_upload_authority = merkle_root_upload_authority;
    tip_distribution_account.merkle_root = None;
    tip_distribution_account.expires_at = current_epoch;
    tip_distribution_account.bump = bump;

    tip_distribution_account.validate()?;

    Ok(())
}
