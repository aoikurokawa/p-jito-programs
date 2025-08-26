use std::str::FromStr;

use anyhow::anyhow;
use clap::{Parser, Subcommand};
use jito_tip_payment_core::{config::Config, load_unchecked};
use jito_tip_payment_program::{
    CONFIG_ACCOUNT_SEED, TIP_ACCOUNT_SEED_0, TIP_ACCOUNT_SEED_1, TIP_ACCOUNT_SEED_2,
    TIP_ACCOUNT_SEED_3, TIP_ACCOUNT_SEED_4, TIP_ACCOUNT_SEED_5, TIP_ACCOUNT_SEED_6,
    TIP_ACCOUNT_SEED_7,
};
use jito_tip_payment_sdk::sdk::initialize_config;
use solana_client::rpc_client::RpcClient;
use solana_keypair::read_keypair_file;
use solana_pubkey::{pubkey, Pubkey};
use solana_signer::Signer;
use solana_transaction::Transaction;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// RPC URL for the Solana cluster
    #[arg(short, long, default_value = "https://api.devnet.solana.com")]
    rpc_url: String,

    /// Tip Payment program ID
    #[arg(
        short,
        long,
        default_value = "3YsnULkzMZ3pJcN1zX2uSyDRdY1RKKhiC32QvhPoUJ3c"
    )]
    program_id: String,

    #[arg(short, long)]
    keypair_path: String,

    #[command(subcommand)]
    command: Commands,
}

#[allow(clippy::enum_variant_names)]
#[derive(Subcommand)]
enum Commands {
    /// Initialize the config account information
    InitConfig,

    /// Get the config account information
    GetConfig,

    /// Get all tip payment accounts information
    GetAllTipAccounts,

    /// Get a specific tip payment account
    GetTipAccount {
        /// Index of the tip account (0-7)
        #[arg(value_parser = clap::value_parser!(u8).range(0..8))]
        index: u8,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let program_id = pubkey!("3YsnULkzMZ3pJcN1zX2uSyDRdY1RKKhiC32QvhPoUJ3c");
    let client = RpcClient::new(cli.rpc_url);

    let keypair = match read_keypair_file(cli.keypair_path) {
        Ok(keypair) => Ok(keypair),
        Err(e) => Err(anyhow!("{}", e)),
    };

    let payer = keypair.unwrap();

    match cli.command {
        Commands::InitConfig => {
            println!("Hello");
            println!("Program ID: {:?}", program_id);
            // let p_config_pubkey = Config::find_program_address(&p_program_id).0;
            // let config_pubkey = solana_pubkey::Pubkey::new_from_array(p_config_pubkey);

            println!("Hello1");
            // let config_pubkey = find_program_address(&[CONFIG_ACCOUNT_SEED], &p_program_id).0;
            // let tip_account_0 = find_program_address(&[TIP_ACCOUNT_SEED_0], &p_program_id).0;
            // let tip_account_1 = find_program_address(&[TIP_ACCOUNT_SEED_1], &p_program_id).0;
            // let tip_account_2 = find_program_address(&[TIP_ACCOUNT_SEED_2], &p_program_id).0;
            // let tip_account_3 = find_program_address(&[TIP_ACCOUNT_SEED_3], &p_program_id).0;
            // let tip_account_4 = find_program_address(&[TIP_ACCOUNT_SEED_4], &p_program_id).0;
            // let tip_account_5 = find_program_address(&[TIP_ACCOUNT_SEED_5], &p_program_id).0;
            // let tip_account_6 = find_program_address(&[TIP_ACCOUNT_SEED_6], &p_program_id).0;
            // let tip_account_7 = find_program_address(&[TIP_ACCOUNT_SEED_7], &p_program_id).0;

            println!("Hello2");
            let ix = initialize_config(
                &Pubkey::find_program_address(&[CONFIG_ACCOUNT_SEED], program_id).0,
                &solana_pubkey::Pubkey::new_from_array(config_pubkey),
                &solana_pubkey::Pubkey::new_from_array(tip_account_0),
                &solana_pubkey::Pubkey::new_from_array(tip_account_1),
                &solana_pubkey::Pubkey::new_from_array(tip_account_2),
                &solana_pubkey::Pubkey::new_from_array(tip_account_3),
                &solana_pubkey::Pubkey::new_from_array(tip_account_4),
                &solana_pubkey::Pubkey::new_from_array(tip_account_5),
                &solana_pubkey::Pubkey::new_from_array(tip_account_6),
                &solana_pubkey::Pubkey::new_from_array(tip_account_7),
                &payer.pubkey(),
            );

            // let mut ix_builder = InitializeConfigBuilder::new();
            // ix_builder
            //     .config(config_address)
            //     .admin(signer.pubkey())
            //     .vault_program(self.vault_program_id);
            // let mut ix = ix_builder.instruction();
            // ix.program_id = self.restaking_program_id;

            // info!("Initializing restaking config parameters: {:?}", ix_builder);

            let blockhash = client.get_latest_blockhash().unwrap();
            let tx = Transaction::new_signed_with_payer(
                &[ix],
                Some(&payer.pubkey()),
                &[payer],
                blockhash,
            );

            client.send_transaction(&tx).unwrap();
        }
        Commands::GetConfig => {
            let config_pda = Config::find_program_address(&p_program_id).0;
            let config_data = client
                .get_account(&solana_pubkey::Pubkey::new_from_array(config_pda))?
                .data;

            let config = unsafe { load_unchecked::<Config>(config_data.as_slice()).unwrap() };

            println!("Config Account:");
            println!(
                "  Tip Receiver: {}",
                solana_pubkey::Pubkey::new_from_array(config.tip_receiver)
            );
            println!(
                "  Block Builder: {}",
                solana_pubkey::Pubkey::new_from_array(config.block_builder)
            );
            println!(
                "  Block Builder Commission %: {}",
                config.block_builder_commission_pct
            );
            println!("  Bumps:");
            println!("    Config: {}", config.bumps.config);
            println!("    Tip Account 0: {}", config.bumps.tip_payment_account_0);
            println!("    Tip Account 1: {}", config.bumps.tip_payment_account_1);
            println!("    Tip Account 2: {}", config.bumps.tip_payment_account_2);
            println!("    Tip Account 3: {}", config.bumps.tip_payment_account_3);
            println!("    Tip Account 4: {}", config.bumps.tip_payment_account_4);
            println!("    Tip Account 5: {}", config.bumps.tip_payment_account_5);
            println!("    Tip Account 6: {}", config.bumps.tip_payment_account_6);
            println!("    Tip Account 7: {}", config.bumps.tip_payment_account_7);
        }
        _ => todo!(), // Commands::GetAllTipAccounts => {
                      //     let tip_account_seeds = [
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_0,
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_1,
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_2,
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_3,
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_4,
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_5,
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_6,
                      //         jito_tip_payment::TIP_ACCOUNT_SEED_7,
                      //     ];

                      //     for (i, seed) in tip_account_seeds.iter().enumerate() {
                      //         let tip_pda = Pubkey::find_program_address(&[seed], &program_id).0;
                      //         let lamports = client.get_account(&tip_pda)?.lamports;

                      //         println!("Tip Payment Account {}:", i);
                      //         println!("  Address: {}", tip_pda);
                      //         println!("  Lamports: {}", lamports);
                      //     }
                      // }
                      // Commands::GetTipAccount { index } => {
                      //     let seed = match index {
                      //         0 => jito_tip_payment::TIP_ACCOUNT_SEED_0,
                      //         1 => jito_tip_payment::TIP_ACCOUNT_SEED_1,
                      //         2 => jito_tip_payment::TIP_ACCOUNT_SEED_2,
                      //         3 => jito_tip_payment::TIP_ACCOUNT_SEED_3,
                      //         4 => jito_tip_payment::TIP_ACCOUNT_SEED_4,
                      //         5 => jito_tip_payment::TIP_ACCOUNT_SEED_5,
                      //         6 => jito_tip_payment::TIP_ACCOUNT_SEED_6,
                      //         7 => jito_tip_payment::TIP_ACCOUNT_SEED_7,
                      //         _ => unreachable!(),
                      //     };

                      //     let tip_pda = Pubkey::find_program_address(&[seed], &program_id).0;
                      //     let lamports = client.get_account(&tip_pda)?.lamports;

                      //     println!("Tip Payment Account {}:", index);
                      //     println!("  Address: {}", tip_pda);
                      //     println!("  Lamports: {}", lamports);
                      // }
    }

    Ok(())
}
