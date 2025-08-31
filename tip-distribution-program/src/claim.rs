use jito_tip_core::{create_account, loader::load_signer};
use jito_tip_distribution_core::{
    claim_status::ClaimStatus, config::Config, load_mut_unchecked,
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

use crate::verify;

/// Claims tokens from the [TipDistributionAccount].
#[allow(dead_code)]
pub fn process_claim(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    bump: u8,
    amount: u64,
    proof: [[u8; 32]; 32],
) -> Result<(), ProgramError> {
    let [config_info, tip_distribution_account_info, merkle_root_upload_authority_info, claim_status_info, claimant_info, payer_info, system_program_info, validator_vote_account_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let current_epoch = Clock::get()?.epoch;

    unsafe {
        Config::load(program_id, config_info, false)?;
    }

    let tip_distribution_account = unsafe {
        TipDistributionAccount::load(
            program_id,
            tip_distribution_account_info,
            validator_vote_account_info.key(),
            current_epoch,
            false,
        )?;
        load_mut_unchecked::<TipDistributionAccount>(
            &mut tip_distribution_account_info.borrow_mut_data_unchecked()[8..],
        )?
    };

    if tip_distribution_account
        .merkle_root_upload_authority
        .ne(merkle_root_upload_authority_info.key())
    {
        return Err(TipDistributionError::Unauthorized.into());
    }

    load_signer(merkle_root_upload_authority_info, false)?;
    load_signer(payer_info, true)?;

    let rent = Rent::get()?;
    let space = 8usize
        .checked_add(ClaimStatus::LEN)
        .ok_or(TipDistributionError::ArithmeticError)?;

    let (claim_status_pubkey, claim_status_bump) = ClaimStatus::find_program_address(
        program_id,
        claimant_info.key(),
        tip_distribution_account_info.key(),
    );

    if claim_status_pubkey.ne(claim_status_info.key()) {
        log!("ClaimStatus account is not at the correct PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    let claim_status_bump_slice = [claim_status_bump];

    // Create the seeds for the PDA
    let claim_status_seeds = [
        Seed::from(b"CLAIM_STATUS".as_slice()),
        Seed::from(claimant_info.key().as_ref()),
        Seed::from(tip_distribution_account_info.key().as_ref()),
        Seed::from(claim_status_bump_slice.as_slice()),
    ];

    let signers = [Signer::from(claim_status_seeds.as_slice())];

    log!("Initializing ClaimStatus at address {}", config_info.key());
    create_account(
        payer_info,
        claim_status_info,
        system_program_info,
        program_id,
        &rent,
        space as u64,
        &signers,
    )?;

    let claim_status = unsafe {
        let claim_status_info_data = claim_status_info.borrow_mut_data_unchecked();
        claim_status_info_data[0..8].copy_from_slice(ClaimStatus::DISCRIMINATOR);
        load_mut_unchecked::<ClaimStatus>(&mut claim_status_info_data[8..])?
    };

    claim_status.bump = bump;

    let clock = Clock::get()?;
    if clock.epoch > tip_distribution_account.expires_at {
        return Err(TipDistributionError::ExpiredTipDistributionAccount.into());
    }

    // Redundant check since we shouldn't be able to init a claim status account using the same seeds.
    if claim_status.is_claimed {
        return Err(TipDistributionError::FundsAlreadyClaimed.into());
    }

    // let tip_distribution_info = tip_distribution_account.to_account_info();
    let tip_distribution_epoch_expires_at = tip_distribution_account.expires_at;
    let merkle_root = tip_distribution_account
        .merkle_root
        .as_mut()
        .ok_or(TipDistributionError::RootNotUploaded)?;

    // Verify the merkle proof.
    let node = &solana_program::hash::hashv(&[
        &[0u8],
        &solana_program::hash::hashv(&[claimant_info.key().as_slice(), &amount.to_le_bytes()])
            .to_bytes(),
    ]);

    if !verify(&proof, merkle_root.root, node.to_bytes()) {
        return Err(TipDistributionError::InvalidProof.into());
    }

    TipDistributionAccount::claim(tip_distribution_account_info, claimant_info, amount)?;

    // Mark it claimed.
    claim_status.amount = amount;
    claim_status.is_claimed = true;
    claim_status.slot_claimed_at = clock.slot;
    claim_status.claimant = *claimant_info.key();
    claim_status.claim_status_payer = *payer_info.key();
    claim_status.expires_at = tip_distribution_epoch_expires_at;

    merkle_root.total_funds_claimed = merkle_root
        .total_funds_claimed
        .checked_add(amount)
        .ok_or(TipDistributionError::ArithmeticError)?;
    if merkle_root.total_funds_claimed > merkle_root.max_total_claim {
        return Err(TipDistributionError::ExceedsMaxClaim.into());
    }

    merkle_root.num_nodes_claimed = merkle_root
        .num_nodes_claimed
        .checked_add(1)
        .ok_or(TipDistributionError::ArithmeticError)?;
    if merkle_root.num_nodes_claimed > merkle_root.max_num_nodes {
        return Err(TipDistributionError::ExceedsMaxNumNodes.into());
    }

    tip_distribution_account.validate()?;

    Ok(())
}
