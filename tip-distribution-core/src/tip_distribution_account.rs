use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
};
use pinocchio_system::instructions::Transfer;
use shank::ShankAccount;

use crate::{merkle_root::MerkleRoot, Transmutable};

/// The account that validators register as **tip_receiver** with the tip-payment program.
#[derive(Debug, Default, ShankAccount)]
#[repr(C)]
pub struct TipDistributionAccount {
    /// The validator's vote account, also the recipient of remaining lamports after
    /// upon closing this account.
    pub validator_vote_account: Pubkey,

    /// The only account authorized to upload a merkle-root for this account.
    pub merkle_root_upload_authority: Pubkey,

    /// The merkle root used to verify user claims from this account.
    pub merkle_root: Option<MerkleRoot>,

    /// Epoch for which this account was created.  
    pub epoch_created_at: u64,

    /// The commission basis points this validator charges.
    pub validator_commission_bps: u16,

    /// The epoch (upto and including) that tip funds can be claimed.
    pub expires_at: u64,

    /// The bump used to generate this account
    pub bump: u8,
}

unsafe impl Transmutable for TipDistributionAccount {
    // header, fields, and InitBumps
    const LEN: usize = 8 + 32 + 32 + 8 + 1;
}

impl TipDistributionAccount {
    pub fn validate(&self) -> Result<(), TipDistributionError> {
        let default_pubkey = Pubkey::default();
        if self.validator_vote_account == default_pubkey
            || self.merkle_root_upload_authority == default_pubkey
        {
            return Err(TipDistributionError::AccountValidationFailure);
        }

        Ok(())
    }

    pub fn claim_expired(from: &AccountInfo, to: &AccountInfo) -> Result<u64, ProgramError> {
        let rent = Rent::get()?;
        let min_rent_lamports = rent.minimum_balance(from.data_len());

        let amount = from
            .lamports()
            .checked_sub(min_rent_lamports)
            .ok_or(TipDistributionError::ArithmeticError)?;
        Self::transfer_lamports(from, to, amount)?;

        Ok(amount)
    }

    pub fn claim(from: &AccountInfo, to: &AccountInfo, amount: u64) -> Result<(), ProgramError> {
        Self::transfer_lamports(from, to, amount)
    }

    fn transfer_lamports(
        from: &AccountInfo,
        to: &AccountInfo,
        amount: u64,
    ) -> Result<(), ProgramError> {
        Transfer {
            from,
            to,
            lamports: amount,
        }
        .invoke()?;
        // debit lamports
        // *from.try_borrow_mut_lamports()? = from
        //     .lamports()
        //     .checked_sub(amount)
        //     .ok_or(TipDistributionError::ArithmeticError)?;
        // credit lamports
        // to.try_borrow_mut_lamports()? = to
        //     .lamports()
        //     .checked_add(amount)
        //     .ok_or(TipDistributionError::ArithmeticError)?;

        Ok(())
    }

    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"TIP_DISTRIBUTION_ACCOUNT".to_vec()]
    }
}
