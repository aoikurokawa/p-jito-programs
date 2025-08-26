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
pub struct MerkleRootUploadConfig {
    /// The authority that overrides the TipDistributionAccount merkle_root_upload_authority
    pub override_authority: Pubkey,

    /// The original merkle root upload authority that can be changed to the new overrided
    /// authority. E.g. Jito Labs authority GZctHpWXmsZC1YHACTGGcHhYxjdRqQvTpYkb9LMvxDib
    pub original_upload_authority: Pubkey,

    /// The bump used to generate this account
    pub bump: u8,
}

unsafe impl Transmutable for MerkleRootUploadConfig {
    // header, fields, and InitBumps
    const LEN: usize = std::mem::size_of::<MerkleRootUploadConfig>();
}

impl MerkleRootUploadConfig {
    pub const DISCRIMINATOR: &'static [u8] = &[213, 125, 30, 192, 25, 121, 87, 33];

    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"ROOT_UPLOAD_CONFIG".to_vec()]
    }

    /// Find the program address for the global configuration account
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// # Returns
    /// * `Pubkey` - The program address
    /// * `u8` - The bump seed
    /// * `Vec<Vec<u8>>` - The seeds used to generate the PDA
    pub fn find_program_address(program_id: &Pubkey) -> (Pubkey, u8, Vec<Vec<u8>>) {
        let seeds = Self::seeds();
        let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let (pda, bump) = find_program_address(&seeds_iter, program_id);
        (pda, bump, seeds)
    }

    /// Attempts to load the account as [`Config`], returning an error if it's not valid.
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// * `account` - The account to load the configuration from
    /// * `expect_writable` - Whether the account should be writable
    ///
    /// # Returns
    /// * `Result<(), ProgramError>` - The result of the operation
    ///
    /// # Safety
    pub unsafe fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner().ne(program_id) {
            msg!("MerkleRootUploadConfig account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("MerkleRootUploadConfig account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable() {
            msg!("MerkleRootUploadConfig account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }

        let data = account.borrow_data_unchecked();
        if data[0..8].ne(Self::DISCRIMINATOR) {
            msg!("MerkleRootUploadConfig account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.key().ne(&Self::find_program_address(program_id).0) {
            msg!("MerkleRootUploadConfig account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
