use std::{str::FromStr, sync::Arc};

use clap::Parser;
use jito_tip_cli::{
    cli_args::{Cli, ProgramCommand},
    tip_distribution_handler::TipDistributionCliHandler,
};
use jito_tip_distribution_sdk_legacy::derive_config_account_address;
use solana_client::rpc_client::RpcClient;
use solana_keypair::read_keypair_file;
use solana_pubkey::Pubkey;

fn main() -> anyhow::Result<()> {
    let args: Cli = Cli::parse();

    let program_id = Pubkey::from_str(&args.tip_distribution_program_id)?;

    let client = RpcClient::new(args.rpc_url);

    let keypair = read_keypair_file(args.keypair_path).expect("Failed to read keypair");
    let keypair = Arc::new(keypair);

    let (config_pda, config_bump) = derive_config_account_address(&program_id);

    match args.command.expect("Command not found") {
        ProgramCommand::TipDistribution { action } => {
            TipDistributionCliHandler::new(client, keypair, program_id, config_pda, config_bump)
                .handle(action)?
        }
    }

    Ok(())
}
