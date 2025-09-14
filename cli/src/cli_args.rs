use clap::{Parser, Subcommand};

use crate::tip_distribution::TipDistributionCommands;

#[derive(Parser)]
#[command(author, version, about = "A CLI for managing jito-tip-distribution operations", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<ProgramCommand>,

    #[arg(
        long,
        global = true,
        default_value = "https://api.devnet.solana.com",
        help = "RPC URL to use"
    )]
    pub rpc_url: String,

    #[arg(
        long,
        global = true,
        default_value = "~/.config/solana/id.json",
        help = "Keypair path"
    )]
    pub keypair_path: String,

    #[arg(long, global = true, help = "Commitment level")]
    pub commitment: Option<String>,

    #[arg(
        long,
        global = true,
        default_value = "3vgVYgJxqFKF2cFYHV4GPBUnLynCJYmKizq9DRmZmTUf",
        help = "Restaking program ID"
    )]
    pub tip_distribution_program_id: String,

    #[arg(long, global = true, help = "Filepath or URL to a keypair")]
    pub signer: Option<String>,
}

#[derive(Subcommand)]
pub enum ProgramCommand {
    /// Jito Tip Distribution program commands
    TipDistribution {
        #[command(subcommand)]
        action: TipDistributionCommands,
    },
}
