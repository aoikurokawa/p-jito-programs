use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
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
    const LEN: usize = std::mem::size_of::<Self>();
}

impl TipDistributionAccount {
    pub const SEED: &'static [u8] = b"TIP_DISTRIBUTION_ACCOUNT";
    pub const SIZE: usize = 8 + size_of::<Self>();
    pub const DISCRIMINATOR: &'static [u8] = &[85, 64, 113, 198, 234, 94, 120, 123];

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

    /// Returns the seeds for the PDA
    pub fn seeds(validator_vote_account: &Pubkey, epoch: u64) -> Vec<Vec<u8>> {
        vec![
            b"TIP_DISTRIBUTION_ACCOUNT".to_vec(),
            validator_vote_account.to_vec(),
            epoch.to_le_bytes().to_vec(),
        ]
    }

    /// Find the program address for the PDA
    pub fn find_program_address(
        program_id: &Pubkey,
        validator_vote_account: &Pubkey,
        epoch: u64,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(validator_vote_account, epoch);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Attempts to load the account as [`TipDistributionAccount`], returning an error if it's not valid.
    ///
    /// # Safety
    pub unsafe fn load(
        program_id: &Pubkey,
        tip_distribution_account: &AccountInfo,
        validator_vote_account: &Pubkey,
        epoch: u64,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if tip_distribution_account.owner().ne(program_id) {
            msg!("TipDistributionAccount has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if tip_distribution_account.data_is_empty() {
            msg!("TipDistributionAccount data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !tip_distribution_account.is_writable() {
            msg!("TipDistributionAccount is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        let data = tip_distribution_account.borrow_data_unchecked();
        if data[0..8].ne(Self::DISCRIMINATOR) {
            msg!("TipDistributionAccount discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        let expected_pubkey =
            Self::find_program_address(program_id, validator_vote_account, epoch).0;
        if tip_distribution_account.key().ne(&expected_pubkey) {
            msg!("TipDistributionAccount is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
