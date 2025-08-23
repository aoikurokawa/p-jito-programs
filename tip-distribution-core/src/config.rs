use jito_tip_distribution_sdk::error::TipDistributionError;
use pinocchio::pubkey::Pubkey;
use shank::ShankAccount;

use crate::Transmutable;

/// Stores program config metadata.
#[derive(Debug, Default, ShankAccount)]
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
    const LEN: usize = 8 + 32 + 32 + 8 + 2 + 1;
}

impl Config {
    pub const SEED: &'static [u8] = b"CONFIG_ACCOUNT";
    pub const SIZE: usize = 8 + size_of::<Self>();

    /// Initialize a [`Config`]
    pub fn new(
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

    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"CONFIG_ACCOUNT".to_vec()]
    }

    pub fn validate(&self) -> Result<(), TipDistributionError> {
        const MAX_NUM_EPOCHS_VALID: u64 = 10;
        const MAX_VALIDATOR_COMMISSION_BPS: u16 = 10000;

        if self.num_epochs_valid == 0 || self.num_epochs_valid > MAX_NUM_EPOCHS_VALID {
            return Err(TipDistributionError::AccountValidationFailure.into());
        }

        if self.max_validator_commission_bps > MAX_VALIDATOR_COMMISSION_BPS {
            return Err(TipDistributionError::AccountValidationFailure.into());
        }

        let default_pubkey = Pubkey::default();
        if self.expired_funds_account == default_pubkey || self.authority == default_pubkey {
            return Err(TipDistributionError::AccountValidationFailure.into());
        }

        Ok(())
    }
}
