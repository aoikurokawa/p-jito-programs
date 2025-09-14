use clap::Subcommand;
use solana_pubkey::Pubkey;

/// The CLI handler for the jito-tip-distribution program
#[derive(Subcommand)]
pub enum TipDistributionCommands {
    /// Initialize, get, and set the config struct
    Config {
        #[command(subcommand)]
        action: ConfigActions,
    },
    // Ncn {
    //     #[command(subcommand)]
    //     action: NcnActions,
    // },
    // Operator {
    //     #[command(subcommand)]
    //     action: OperatorActions,
    // },
}

/// The actions that can be performed on the restaking config
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
