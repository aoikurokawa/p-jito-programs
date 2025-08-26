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
    const LEN: usize = std::mem::size_of::<Self>();
}

impl ClaimStatus {
    pub const DISCRIMINATOR: &'static [u8] = &[22, 183, 249, 157, 247, 95, 150, 96];

    /// Seeds
    pub fn seeds(claimant: Pubkey, tip_distribution_account: Pubkey) -> Vec<Vec<u8>> {
        vec![
            b"CLAIM_STATUS".to_vec(),
            claimant.to_vec(),
            tip_distribution_account.to_vec(),
        ]
    }

    /// Find the program address for the PDA
    pub fn find_program_address(
        program_id: &Pubkey,
        claimant: Pubkey,
        tip_distribution_account: Pubkey,
    ) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds(claimant, tip_distribution_account);
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Attempts to load the account as [`TipDistributionAccount`], returning an error if it's not valid.
    ///
    /// # Safety
    pub unsafe fn load(
        program_id: &Pubkey,
        claim_status: &AccountInfo,
        claimant: Pubkey,
        tip_distribution_account: Pubkey,
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
