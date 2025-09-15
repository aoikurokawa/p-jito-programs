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

    /// Initialize, get the MerkleRootUploadConfig struct
    MerkleRootUploadConfig {
        #[command(subcommand)]
        action: MerkleRootUploadConfigActions,
    },

    /// Initialize, get the ClaimStatus struct
    ClaimStatus {
        #[command(subcommand)]
        action: ClaimStatusActions,
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

    //     /// Update the config account information
    Update {
        /// Authority pubkey
        #[arg(long)]
        authority: String,

        /// Expired funds account pubkey
        #[arg(long)]
        expired_funds_account: String,

        /// Number of epochs valid
        #[arg(long)]
        num_epochs_valid: u64,

        /// Max validator commission BPS
        #[arg(long)]
        max_validator_commission_bps: u16,
    },
}

/// The actions that can be performed on the TipDistributionAccount
#[derive(Subcommand)]
pub enum TipDistributionAccountActions {
    /// Initialize the TipDistributionAccount
    Initialize {
        /// Validator vote account pubkey
        #[clap(long)]
        vote_account: Pubkey,

        /// Merkle root upload authority
        #[clap(long)]
        merkle_root_upload_authority: Pubkey,

        /// Validator commission BPS
        #[clap(long)]
        validator_commission_bps: u16,
    },

    /// Upload merkle root
    UploadMerkleRoot {
        /// Validator vote account pubkey
        #[arg(long)]
        vote_account: Pubkey,

        /// Root
        #[arg(long)]
        root: String,

        /// Max Total Claim
        #[arg(long)]
        max_total_claim: u64,

        /// Max number of nodes
        #[arg(long)]
        max_num_nodes: u64,
    },

    /// Get the TipDistributionAccount
    Get {
        /// Validator vote account pubkey
        #[clap(long)]
        vote_account: String,

        /// Epoch number
        #[clap(long)]
        epoch: u64,
    },

    /// Close the TipDistributionAccount
    Close {
        /// Validator vote account pubkey
        #[arg(long)]
        vote_account: Pubkey,

        /// Epoch number
        #[arg(long)]
        epoch: u64,
    },
}

/// The actions that can be performed on the MerkleRootUploadConfig
#[derive(Subcommand)]
pub enum MerkleRootUploadConfigActions {
    /// Initialize the MerkleRootUploadConfig
    Initialize,

    /// Update the MerkleRootUploadConfig
    Update,

    /// Update the MerkleRootUploadConfig authority
    MigrateTdaMerkleRootUploadAuthority {
        /// Validator vote account pubkey
        #[arg(long)]
        vote_account: Pubkey,

        /// Epoch number
        #[arg(long)]
        epoch: u64,
    },
}

/// The actions that can be performed on the MerkleRootUploadConfig
#[derive(Subcommand)]
pub enum ClaimStatusActions {
    /// Claim
    Claim {
        #[arg(long)]
        vote_account: Pubkey,

        #[arg(long)]
        epoch: u64,

        #[arg(long)]
        claimant: Pubkey,

        #[arg(long)]
        amount: u64,
    },

    /// Get claim status for a specific validator, epoch and claimant
    GetClaimStatus {
        /// Validator vote account pubkey
        #[arg(long)]
        vote_account: String,

        /// Epoch for the tip distribution account
        #[arg(long)]
        epoch: u64,

        /// Claimant pubkey
        #[arg(long)]
        claimant: String,
    },

    /// Close claim status
    CloseClaimStatus {
        #[arg(long)]
        vote_account: Pubkey,

        #[arg(long)]
        epoch: u64,

        #[arg(long)]
        claimant: Pubkey,
    },
}
