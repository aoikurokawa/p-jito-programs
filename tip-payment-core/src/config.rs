use pinocchio::pubkey::{find_program_address, Pubkey};
use shank::ShankAccount;

use crate::{init_bumps::InitBumps, Transmutable};

/// Stores program config metadata.
#[derive(Debug, Default, ShankAccount)]
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
    const LEN: usize = core::mem::size_of::<Self>();
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

    pub const fn seeds() -> &'static [&'static [u8]] {
        &[b"CONFIG_ACCOUNT"]
    }

    /// Find the program address for the global configuration account
    ///
    /// # Arguments
    /// * `program_id` - The program ID
    /// # Returns
    /// * `Pubkey` - The program address
    /// * `u8` - The bump seed
    /// * `Vec<Vec<u8>>` - The seeds used to generate the PDA
    pub fn find_program_address(program_id: &Pubkey) -> (Pubkey, u8) {
        let seeds = Self::seeds();
        let (pda, bump) = find_program_address(seeds, program_id);
        (pda, bump)
    }
}
