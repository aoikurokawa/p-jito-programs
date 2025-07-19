use pinocchio::pubkey::Pubkey;

use crate::{init_bumps::InitBumps, Transmutable};

/// Stores program config metadata.
#[derive(Debug, Default)]
#[repr(C)]
pub struct Config {
    /// The account claiming tips from the mev_payment accounts.
    pub tip_receiver: Pubkey,

    /// Block builder that receives a % of fees
    pub block_builder: Pubkey,

    /// The percentage of block builder commission
    pub block_builder_commission_pct: u64,

    /// Bumps used to derive PDAs
    pub bumps: InitBumps,
}

unsafe impl Transmutable for Config {
    // header, fields, and InitBumps
    const LEN: usize = 8 + 32 + 32 + 8 + InitBumps::SIZE;
}

impl Config {
    /// Initialize a [`Config`]
    pub fn new(
        tip_receiver: Pubkey,
        block_builder: Pubkey,
        block_builder_commission_pct: u64,
    ) -> Self {
        Self {
            tip_receiver,
            block_builder,
            block_builder_commission_pct,
            bumps: InitBumps::default(),
        }
    }

    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"CONFIG_ACCOUNT".to_vec()]
    }
}
