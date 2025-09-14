use clap::Subcommand;
use solana_pubkey::Pubkey;

/// The CLI handler for the jito-tip-distribution program
#[derive(Subcommand)]
pub enum TipDistributionCommands {
    /// Initialize, get the config struct
    Config {
        #[command(subcommand)]
        action: ConfigActions,
    },

    /// Initialize, get the TipDistributionAccount struct
    TipDistributionAccount {
        #[command(subcommand)]
        action: TipDistributionAccountActions,
    },
}

/// The actions that can be performed on the tip_distribution_account config
#[derive(Subcommand)]
pub enum ConfigActions {
    /// Initialize the config
    Initialize {
        /// Authority
        authority: Pubkey,

        /// Expired funds account
        expired_funds_account: Pubkey,

        /// Number of epochs is valid
        num_epochs_valid: u64,

        /// Max validator commission BPS
        max_validator_commission_bps: u16,
    },

    /// Get the config
    Get,
}

/// The actions that can be performed on the TipDistributionAccount
#[derive(Subcommand)]
pub enum TipDistributionAccountActions {
    /// Initialize the TipDistributionAccount
    Initialize {
        /// Validator vote account pubkey
        vote_account: Pubkey,

        /// Merkle root upload authority
        merkle_root_upload_authority: Pubkey,

        /// Validator commission BPS
        validator_commission_bps: u16,
    },

    /// Get the TipDistributionAccount
    Get {
        /// Validator vote account pubkey
        vote_account: String,

        /// Epoch for the tip distribution account
        epoch: u64,
    },
}
