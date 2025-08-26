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
    const LEN: usize = std::mem::size_of::<Config>();
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
}
