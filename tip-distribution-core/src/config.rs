use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::{
    account_info::AccountInfo,
    msg,
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
};

use crate::Transmutable;

/// Stores program config metadata.
#[derive(Debug)]
#[repr(C)]
pub struct Config {
    /// Account with authority over this PDA.
    pub authority: Pubkey,

    /// We want to expire funds after some time so that validators can be refunded the rent.
    /// Expired funds will get transferred to this account.
    pub expired_funds_account: Pubkey,

    /// Specifies the number of epochs a merkle root is valid for before expiring.
    pub num_epochs_valid: u64,

    /// The maximum commission a validator can set on their distribution account.
    pub max_validator_commission_bps: u16,

    /// The bump used to generate this account
    pub bump: u8,
}

unsafe impl Transmutable for Config {
    // header, fields, and InitBumps
    const LEN: usize = core::mem::size_of::<Self>();
}

impl Config {
    pub const SEED: &'static [u8] = b"CONFIG_ACCOUNT";
    pub const DISCRIMINATOR: &'static [u8] = &[155, 12, 170, 224, 30, 250, 204, 130];

    /// Initialize a [`Config`]
    #[inline(always)]
    pub const fn new(
        authority: Pubkey,
        expired_funds_account: Pubkey,
        num_epochs_valid: u64,
        max_validator_commission_bps: u16,
        bump: u8,
    ) -> Self {
        Self {
            authority,
            expired_funds_account,
            num_epochs_valid,
            max_validator_commission_bps,
            bump,
        }
    }

    #[inline(always)]
    pub fn validate(&self) -> Result<(), TipDistributionError> {
        const MAX_NUM_EPOCHS_VALID: u64 = 10;
        const MAX_VALIDATOR_COMMISSION_BPS: u16 = 10000;

        if self.num_epochs_valid == 0 || self.num_epochs_valid > MAX_NUM_EPOCHS_VALID {
            msg!("num_epochs should be more than 0 and less than 10");
            return Err(TipDistributionError::AccountValidationFailure);
        }

        if self.max_validator_commission_bps > MAX_VALIDATOR_COMMISSION_BPS {
            msg!("max_validator_commission_bps should be less than 10000");
            return Err(TipDistributionError::AccountValidationFailure);
        }

        let default_pubkey = Pubkey::default();
        if self.expired_funds_account == default_pubkey || self.authority == default_pubkey {
            msg!("expired_funds_account should not default pubkey");
            return Err(TipDistributionError::AccountValidationFailure);
        }

        Ok(())
    }

    /// Find the program address for the global configuration account
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// # Returns
    /// * `Pubkey` - The program address
    /// * `u8` - The bump seed
    /// * `Vec<Vec<u8>>` - The seeds used to generate the PDA
    #[inline(always)]
    pub fn find_program_address(program_id: &Pubkey) -> (Pubkey, u8) {
        // let seeds = Self::seeds();
        // let seeds_iter: Vec<_> = seeds.iter().map(|s| s.as_slice()).collect();
        let seeds = [Self::SEED];
        let (pda, bump) = find_program_address(&seeds, program_id);
        (pda, bump)
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
    #[inline(always)]
    pub unsafe fn load(
        program_id: &Pubkey,
        account: &AccountInfo,
        expect_writable: bool,
    ) -> Result<(), ProgramError> {
        if account.owner().ne(program_id) {
            msg!("Config account has an invalid owner");
            return Err(ProgramError::InvalidAccountOwner);
        }
        if account.data_is_empty() {
            msg!("Config account data is empty");
            return Err(ProgramError::InvalidAccountData);
        }
        if expect_writable && !account.is_writable() {
            msg!("Config account is not writable");
            return Err(ProgramError::InvalidAccountData);
        }

        let data = account.borrow_data_unchecked();
        if data[0..8].ne(Self::DISCRIMINATOR) {
            msg!("Config account discriminator is invalid");
            return Err(ProgramError::InvalidAccountData);
        }
        if account.key().ne(&Self::find_program_address(program_id).0) {
            msg!("Config account is not at the correct PDA");
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(())
    }
}
