use borsh::{BorshDeserialize, BorshSerialize};
use jito_tip_core::loader::load_signer;
use jito_tip_distribution_core::{
    config::Config, merkle_root::MerkleRoot, tip_distribution_account::TipDistributionAccount,
};
use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{clock::Clock, Sysvar},
};

/// Uploads a merkle root to the provided [TipDistributionAccount].
///
/// This instruction may be invoked many times as long as the account is at least one epoch old and not expired; and
/// no funds have already been claimed. Only the `merkle_root_upload_authority` has the
/// authority to invoke.
pub fn process_upload_merkle_root(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    root: [u8; 32],
    max_total_claim: u64,
    max_num_nodes: u64,
) -> Result<(), ProgramError> {
    let [config_info, tip_distribution_account_info, merkle_root_upload_authority_info] = accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let current_epoch = Clock::get()?.epoch;

    unsafe {
        Config::load(program_id, config_info, false)?;
    }

    load_signer(merkle_root_upload_authority_info, false)?;

    let mut tip_distribution_account = unsafe {
        TipDistributionAccount::deserialize(
            &mut tip_distribution_account_info.borrow_mut_data_unchecked()[8..].as_ref(),
        )
        .map_err(|_e| ProgramError::BorshIoError)?
    };

    if tip_distribution_account
        .merkle_root_upload_authority
        .ne(merkle_root_upload_authority_info.key())
    {
        return Err(TipDistributionError::Unauthorized.into());
    }

    if let Some(merkle_root) = tip_distribution_account.merkle_root {
        if merkle_root.num_nodes_claimed > 0 {
            return Err(TipDistributionError::Unauthorized.into());
        }
    }

    if current_epoch <= tip_distribution_account.epoch_created_at {
        return Err(TipDistributionError::PrematureMerkleRootUpload.into());
    }

    if current_epoch > tip_distribution_account.expires_at {
        return Err(TipDistributionError::ExpiredTipDistributionAccount.into());
    }

    tip_distribution_account.merkle_root = Some(MerkleRoot {
        root,
        max_total_claim,
        max_num_nodes,
        total_funds_claimed: 0,
        num_nodes_claimed: 0,
    });

    let tip_distribution_account = unsafe {
        let tip_distribution_account_data =
            tip_distribution_account_info.borrow_mut_data_unchecked();
        let mut writer = &mut tip_distribution_account_data[8..];
        tip_distribution_account
            .serialize(&mut writer)
            .map_err(|_e| ProgramError::BorshIoError)?;

        tip_distribution_account
    };

    tip_distribution_account.validate()?;

    Ok(())
}
