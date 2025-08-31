use jito_tip_core::{
    create_account,
    loader::{load_signer, load_system_program},
};
use jito_tip_distribution_core::{
    config::Config, load_mut_unchecked, load_unchecked,
    tip_distribution_account::TipDistributionAccount, Transmutable,
};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{clock::Clock, rent::Rent, Sysvar},
};
use pinocchio_log::log;

/// Initialize a new [TipDistributionAccount] associated with the given validator vote key
/// and current epoch.
pub fn process_initialize_tip_distribution_account(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    merkle_root_upload_authority: Pubkey,
    validator_commission_bps: u16,
    bump: u8,
) -> Result<(), ProgramError> {
    let [config_info, tip_distribution_account_info, validator_vote_account_info, signer_info, system_program_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    load_signer(signer_info, true)?;
    load_system_program(system_program_info)?;

    let cfg = unsafe {
        Config::load(program_id, config_info, false)?;
        load_unchecked::<Config>(&config_info.borrow_data_unchecked()[8..])?
    };

    if validator_commission_bps > cfg.max_validator_commission_bps {
        log!(
            "Validator commission BPS {} should be less than {}",
            validator_commission_bps,
            cfg.max_validator_commission_bps
        );
        return Err(TipDistributionError::MaxValidatorCommissionFeeBpsExceeded.into());
    }

    // let validator_vote_state = VoteState::deserialize(&ctx.accounts.validator_vote_account)?;
    // if &validator_vote_state.node_pubkey != ctx.accounts.signer.key {
    //     return Err(Unauthorized.into());
    // }

    let current_epoch = Clock::get()?.epoch;
    let rent = Rent::get()?;
    let space = 8usize
        .checked_add(TipDistributionAccount::LEN)
        .ok_or(TipDistributionError::ArithmeticError)?;

    let (tip_distribution_account_pubkey, tip_distribution_account_bump) =
        TipDistributionAccount::find_program_address(
            program_id,
            validator_vote_account_info.key(),
            current_epoch,
        );

    if tip_distribution_account_pubkey.ne(tip_distribution_account_info.key()) {
        log!("TipDistributionAccount account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    let tip_distribution_account_bump_slice = [tip_distribution_account_bump];

    // Create the seeds for the PDA
    let current_epoch_bytes = current_epoch.to_le_bytes();
    let tip_distribution_account_seeds = [
        Seed::from(b"TIP_DISTRIBUTION_ACCOUNT".as_slice()),
        Seed::from(validator_vote_account_info.key().as_ref()),
        Seed::from(current_epoch_bytes.as_slice()),
        Seed::from(tip_distribution_account_bump_slice.as_slice()),
    ];

    let signers = [Signer::from(tip_distribution_account_seeds.as_slice())];

    log!(
        "Initializing TipDistributionAccount at address {}",
        tip_distribution_account_info.key()
    );

    create_account(
        signer_info,
        tip_distribution_account_info,
        system_program_info,
        program_id,
        &rent,
        space as u64,
        &signers,
    )?;

    let tip_distribution_account = unsafe {
        let tip_distribution_account_data =
            tip_distribution_account_info.borrow_mut_data_unchecked();
        tip_distribution_account_data[0..8].copy_from_slice(TipDistributionAccount::DISCRIMINATOR);
        load_mut_unchecked::<TipDistributionAccount>(&mut tip_distribution_account_data[8..])?
    };

    tip_distribution_account.validator_vote_account = *validator_vote_account_info.key();
    tip_distribution_account.epoch_created_at = current_epoch;
    tip_distribution_account.validator_commission_bps = validator_commission_bps;
    tip_distribution_account.merkle_root_upload_authority = merkle_root_upload_authority;
    tip_distribution_account.merkle_root = None;
    tip_distribution_account.expires_at = current_epoch;
    tip_distribution_account.bump = bump;

    tip_distribution_account.validate()?;

    Ok(())
}
