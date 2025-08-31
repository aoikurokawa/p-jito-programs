use pinocchio::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
};
use shank::ShankAccount;

use crate::Transmutable;

#[derive(Debug, Default, ShankAccount)]
#[repr(C)]
pub struct ClaimStatus {
    /// If true, the tokens have been claimed.
    pub is_claimed: bool,

    /// Authority that claimed the tokens. Allows for delegated rewards claiming.
    pub claimant: Pubkey,

    /// The payer who created the claim.
    pub claim_status_payer: Pubkey,

    /// When the funds were claimed.
    pub slot_claimed_at: u64,

    /// Amount of funds claimed.
    pub amount: u64,

    /// The epoch (upto and including) that tip funds can be claimed.
    /// Copied since TDA can be closed, need to track to avoid making multiple claims
    pub expires_at: u64,

    /// The bump used to generate this account
    pub bump: u8,
}

unsafe impl Transmutable for ClaimStatus {
    // header, fields, and InitBumps
    const LEN: usize = core::mem::size_of::<Self>();
}

impl ClaimStatus {
    pub const DISCRIMINATOR: &'static [u8] = &[22, 183, 249, 157, 247, 95, 150, 96];

    /// Find the program address for the PDA
    #[inline(always)]
    pub fn find_program_address(
        program_id: &Pubkey,
        claimant: &Pubkey,
        tip_distribution_account: &Pubkey,
    ) -> (Pubkey, u8) {
        let seeds = [
            b"CLAIM_STATUS".as_slice(),
            claimant.as_ref(),
            tip_distribution_account.as_ref(),
        ];

        find_program_address(&seeds, program_id)
    }

    /// Attempts to load the account as [`ClaimStatus`], returning an error if it's not valid.
    ///
    /// # Safety
    #[inline(always)]
    pub unsafe fn load(
        program_id: &Pubkey,
        claim_status: &AccountInfo,
        claimant: &Pubkey,
        tip_distribution_account: &Pubkey,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if claim_status.owner().ne(program_id) {
            msg!("ClaimStatus has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if claim_status.data_is_empty() {
            msg!("ClaimStatus data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !claim_status.is_writable() {
            msg!("ClaimStatus is not writable");
            return Err(ProgramError::InvalidAccountData);
        }
        let data = claim_status.borrow_data_unchecked();
        if data[0..8].ne(Self::DISCRIMINATOR) {
            msg!("ClaimStatus discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        let expected_pubkey =
            Self::find_program_address(program_id, claimant, tip_distribution_account).0;
        if claim_status.key().ne(&expected_pubkey) {
            msg!("ClaimStatus is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
