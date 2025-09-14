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

// fn derive_merkle_root_upload_config_account_address(
//     tip_distribution_program_id: &Pubkey,
// ) -> (Pubkey, u8) {
//     Pubkey::find_program_address(&[MerkleRootUploadConfig::SEED], tip_distribution_program_id)
// }
//
// fn derive_claim_status_account_address(
//     tip_distribution_program_id: &Pubkey,
//     claimant: &Pubkey,
//     tip_distribution_account: &Pubkey,
// ) -> (Pubkey, u8) {
//     Pubkey::find_program_address(
//         &[
//             ClaimStatus::SEED,
//             claimant.to_bytes().as_ref(),
//             tip_distribution_account.to_bytes().as_ref(),
//         ],
//         tip_distribution_program_id,
//     )
// }

// #[derive(Parser)]
// #[command(author, version, about, long_about = None)]
// struct Cli {
//     /// RPC URL for the Solana cluster
//     #[arg(short, long, default_value = "https://api.devnet.solana.com")]
//     rpc_url: String,
//
//     /// Tip Distribution program ID
//     #[arg(
//         short,
//         long,
//         default_value = "3vgVYgJxqFKF2cFYHV4GPBUnLynCJYmKizq9DRmZmTUf"
//     )]
//     program_id: String,
//
//     #[arg(short, long)]
//     keypair_path: String,
//
//     #[command(subcommand)]
//     command: Commands,
// }

// #[derive(Subcommand)]
// enum Commands {
//     /// Initialize the config account information
//     InitConfig {
//         /// Authority
//         authority: Pubkey,
//
//         /// Expired funds account
//         expired_funds_account: Pubkey,
//
//         /// Number of epochs is valid
//         num_epochs_valid: u64,
//
//         /// Max validator commission BPS
//         max_validator_commission_bps: u16,
//     },
//
//     /// Get the config account information
//     GetConfig,
//
//     /// Get tip distribution account information for a specific validator and epoch
//     InitializeTipDistributionAccount {
//         /// Validator vote account pubkey
//         #[arg(long)]
//         vote_account: Pubkey,
//
//         /// Merkle root upload authority
//         #[arg(long)]
//         merkle_root_upload_authority: Pubkey,
//
//         /// Validator commission BPS
//         #[arg(long)]
//         validator_commission_bps: u16,
//     },
//
//     InitializeMerkleRootUploadConfig,
//
//     /// Get tip distribution account information for a specific validator and epoch
//     GetTipDistributionAccount {
//         /// Validator vote account pubkey
//         #[arg(long)]
//         vote_account: String,
//
//         /// Epoch for the tip distribution account
//         #[arg(long)]
//         epoch: u64,
//     },
//
//     /// Get claim status for a specific validator, epoch and claimant
//     GetClaimStatus {
//         /// Validator vote account pubkey
//         #[arg(long)]
//         vote_account: String,
//
//         /// Epoch for the tip distribution account
//         #[arg(long)]
//         epoch: u64,
//
//         /// Claimant pubkey
//         #[arg(long)]
//         claimant: String,
//     },
//
//     /// Update the config account information
//     UpdateConfig {
//         /// Authority pubkey
//         #[arg(long)]
//         authority: String,
//
//         /// Expired funds account pubkey
//         #[arg(long)]
//         expired_funds_account: String,
//
//         /// Number of epochs valid
//         #[arg(long)]
//         num_epochs_valid: u64,
//
//         /// Max validator commission BPS
//         #[arg(long)]
//         max_validator_commission_bps: u16,
//     },
//
//     /// Upload merkle root
//     UploadMerkleRoot {
//         #[arg(long)]
//         vote_account: Pubkey,
//
//         #[arg(long)]
//         root: String,
//
//         #[arg(long)]
//         max_total_claim: u64,
//
//         #[arg(long)]
//         max_num_nodes: u64,
//     },
//
//     UpdateMerkleRootUploadConfig,
//
//     MigrateTdaMerkleRootUploadAuthority {
//         #[arg(long)]
//         vote_account: Pubkey,
//
//         #[arg(long)]
//         epoch: u64,
//     },
//
//     CloseClaimStatus {
//         #[arg(long)]
//         vote_account: Pubkey,
//
//         #[arg(long)]
//         epoch: u64,
//
//         #[arg(long)]
//         claimant: Pubkey,
//     },
//
//     CloseTipDistributionAccount {
//         #[arg(long)]
//         vote_account: Pubkey,
//
//         #[arg(long)]
//         epoch: u64,
//     },
//
//     Claim {
//         #[arg(long)]
//         vote_account: Pubkey,
//
//         #[arg(long)]
//         epoch: u64,
//
//         #[arg(long)]
//         claimant: Pubkey,
//
//         #[arg(long)]
//         amount: u64,
//     },
// }

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
        } // Commands::GetConfig => {
          //     let (config_pda, _) = derive_config_account_address(&program_id);
          //     println!("Config Account Address: {}", config_pda);

          //     let config_data = client.get_account(&config_pda)?.data;
          //     let config: Config = Config::try_deserialize(&mut config_data.as_slice())?;

          //     println!("Config Account Data:");
          //     println!("  Authority: {}", config.authority);
          //     println!("  Expired Funds Account: {}", config.expired_funds_account);
          //     println!("  Num Epochs Valid: {}", config.num_epochs_valid);
          //     println!(
          //         "  Max Validator Commission BPS: {}",
          //         config.max_validator_commission_bps
          //     );
          //     println!("  Bump: {}", config.bump);
          // }

          // Commands::InitializeTipDistributionAccount {
          //     vote_account,
          //     merkle_root_upload_authority,
          //     validator_commission_bps,
          // } => {
          //     let epoch = client.get_epoch_info()?.epoch;
          //     let (tip_distribution_pubkey, tip_distribution_bump) =
          //         derive_tip_distribution_account_address(&program_id, &vote_account, epoch);

          //     let ix = Instruction {
          //         program_id,
          //         data: jito_tip_distribution_legacy::instruction::InitializeTipDistributionAccount {
          //             merkle_root_upload_authority,
          //             validator_commission_bps,
          //             bump: tip_distribution_bump,
          //         }
          //         .data(),
          //         accounts:
          //             jito_tip_distribution_legacy::accounts::InitializeTipDistributionAccount {
          //                 config: config_pda,
          //                 tip_distribution_account: tip_distribution_pubkey,
          //                 validator_vote_account: vote_account,
          //                 signer: keypair.pubkey(),
          //                 system_program: system_program::ID,
          //             }
          //             .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }

          // Commands::InitializeMerkleRootUploadConfig => {
          //     let (merkle_root_upload_upload_config_pda, _merkle_root_upload_upload_config_bump) =
          //         derive_merkle_root_upload_config_account_address(&program_id);

          //     let ix = Instruction {
          //         program_id,
          //         data: jito_tip_distribution_legacy::instruction::InitializeMerkleRootUploadConfig {
          //             authority: keypair.pubkey(),
          //             original_authority: keypair.pubkey(),
          //         }
          //         .data(),
          //         accounts:
          //             jito_tip_distribution_legacy::accounts::InitializeMerkleRootUploadConfig {
          //                 config: config_pda,
          //                 merkle_root_upload_config: merkle_root_upload_upload_config_pda,
          //                 authority: keypair.pubkey(),
          //                 payer: keypair.pubkey(),
          //                 system_program: system_program::ID,
          //             }
          //             .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }

          // Commands::GetTipDistributionAccount {
          //     vote_account,
          //     epoch,
          // } => {
          //     let vote_pubkey = Pubkey::from_str(&vote_account)?;
          //     let (tip_dist_pda, _) =
          //         derive_tip_distribution_account_address(&program_id, &vote_pubkey, epoch);
          //     println!("Tip Distribution Account Address: {}", tip_dist_pda);

          //     let account_data = client.get_account(&tip_dist_pda)?.data;
          //     let tip_dist: TipDistributionAccount =
          //         TipDistributionAccount::try_deserialize(&mut account_data.as_slice())?;

          //     println!("Tip Distribution Account Data:");
          //     println!("  Vote Account: {}", tip_dist.validator_vote_account);
          //     println!(
          //         "  Merkle Root Upload Authority: {}",
          //         tip_dist.merkle_root_upload_authority
          //     );
          //     println!("  Epoch Created At: {}", tip_dist.epoch_created_at);
          //     println!(
          //         "  Validator Commission BPS: {}",
          //         tip_dist.validator_commission_bps
          //     );
          //     println!("  Expires At: {}", tip_dist.expires_at);
          //     println!("  Bump: {}", tip_dist.bump);

          //     if let Some(merkle_root) = tip_dist.merkle_root {
          //         println!("  Merkle Root:");
          //         println!("    Root: {:?}", merkle_root.root);
          //         println!("    Max Total Claim: {}", merkle_root.max_total_claim);
          //         println!("    Max Num Nodes: {}", merkle_root.max_num_nodes);
          //         println!(
          //             "    Total Funds Claimed: {}",
          //             merkle_root.total_funds_claimed
          //         );
          //         println!("    Num Nodes Claimed: {}", merkle_root.num_nodes_claimed);
          //     } else {
          //         println!("  Merkle Root: None");
          //     }
          // }

          // Commands::GetClaimStatus {
          //     vote_account,
          //     epoch,
          //     claimant,
          // } => {
          //     let vote_pubkey = Pubkey::from_str(&vote_account)?;
          //     let claimant_pubkey = Pubkey::from_str(&claimant)?;

          //     // First get the tip distribution account address
          //     let (tip_dist_pda, _) =
          //         derive_tip_distribution_account_address(&program_id, &vote_pubkey, epoch);

          //     // Then derive claim status PDA using same seeds as in the program
          //     let (claim_status_pda, _) = Pubkey::find_program_address(
          //         &[
          //             ClaimStatus::SEED,
          //             claimant_pubkey.as_ref(),
          //             tip_dist_pda.as_ref(),
          //         ],
          //         &program_id,
          //     );
          //     println!("Claim Status Account Address: {}", claim_status_pda);

          //     let account_data = client.get_account(&claim_status_pda)?.data;
          //     let claim_status: ClaimStatus =
          //         ClaimStatus::try_deserialize(&mut account_data.as_slice())?;

          //     println!("Claim Status Data:");
          //     println!("  Is Claimed: {}", claim_status.is_claimed);
          //     println!("  Claimant: {}", claim_status.claimant);
          //     println!("  Claim Status Payer: {}", claim_status.claim_status_payer);
          //     println!("  Slot Claimed At: {}", claim_status.slot_claimed_at);
          //     println!("  Amount: {}", claim_status.amount);
          //     println!("  Expires At: {}", claim_status.expires_at);
          //     println!("  Bump: {}", claim_status.bump);
          // }

          // Commands::UpdateConfig {
          //     authority,
          //     expired_funds_account,
          //     num_epochs_valid,
          //     max_validator_commission_bps,
          // } => {
          //     let authority_pubkey = Pubkey::from_str(&authority)?;
          //     let expired_funds_account_pubkey = Pubkey::from_str(&expired_funds_account)?;

          //     let config = Config {
          //         authority: authority_pubkey,
          //         expired_funds_account: expired_funds_account_pubkey,
          //         num_epochs_valid,
          //         max_validator_commission_bps,
          //         bump: config_bump,
          //     };

          //     let accounts = UpdateConfigAccounts {
          //         config: Pubkey::default(),
          //         authority: authority_pubkey,
          //     };

          //     let instruction = update_config_ix(
          //         program_id,
          //         UpdateConfigArgs { new_config: config },
          //         accounts,
          //     );

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[instruction],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }
          // Commands::UploadMerkleRoot {
          //     vote_account,
          //     root,
          //     max_total_claim,
          //     max_num_nodes,
          // } => {
          //     let root_bytes: Vec<u8> = root
          //         .split(',')
          //         .map(|byte_str| {
          //             byte_str
          //                 .trim()
          //                 .parse::<u8>()
          //                 .map_err(|e| anyhow::anyhow!("Invalid byte '{}': {}", byte_str, e))
          //         })
          //         .collect::<Result<Vec<u8>, _>>()?;

          //     if root_bytes.len() != 32 {
          //         return Err(anyhow::anyhow!(
          //             "Root must be exactly 32 bytes, got {}",
          //             root_bytes.len()
          //         ));
          //     }

          //     let mut source: [u8; 32] = [0; 32];
          //     source.copy_from_slice(&root_bytes);

          //     let epoch = client.get_epoch_info()?.epoch;
          //     let (tip_distribution_pubkey, _tip_distribution_bump) =
          //         derive_tip_distribution_account_address(&program_id, &vote_account, epoch);

          //     let ix = Instruction {
          //         program_id,
          //         data: jito_tip_distribution_legacy::instruction::UploadMerkleRoot {
          //             root: source,
          //             max_total_claim,
          //             max_num_nodes,
          //         }
          //         .data(),
          //         accounts: jito_tip_distribution_legacy::accounts::UploadMerkleRoot {
          //             config: config_pda,
          //             merkle_root_upload_authority: keypair.pubkey(),
          //             tip_distribution_account: tip_distribution_pubkey,
          //         }
          //         .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }
          // Commands::UpdateMerkleRootUploadConfig => {
          //     let (merkle_root_upload_config_pda, _merkle_root_upload_config_bump) =
          //         derive_merkle_root_upload_config_account_address(&program_id);

          //     let ix = Instruction {
          //         program_id,
          //         data: jito_tip_distribution_legacy::instruction::UpdateMerkleRootUploadConfig {
          //             authority: keypair.pubkey(),
          //             original_authority: keypair.pubkey(),
          //         }
          //         .data(),
          //         accounts: jito_tip_distribution_legacy::accounts::UpdateMerkleRootUploadConfig {
          //             config: config_pda,
          //             merkle_root_upload_config: merkle_root_upload_config_pda,
          //             authority: keypair.pubkey(),
          //             system_program: system_program::ID,
          //         }
          //         .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }

          // Commands::MigrateTdaMerkleRootUploadAuthority {
          //     vote_account,
          //     epoch,
          // } => {
          //     let (tip_distribution_pda, _tip_distribution_bump) =
          //         derive_tip_distribution_account_address(&program_id, &vote_account, epoch);
          //     let (merkle_root_upload_config_pda, _merkle_root_upload_config_bump) =
          //         derive_merkle_root_upload_config_account_address(&program_id);

          //     let ix = Instruction {
          //         program_id,
          //         data:
          //             jito_tip_distribution_legacy::instruction::MigrateTdaMerkleRootUploadAuthority
          //                 .data(),
          //         accounts:
          //             jito_tip_distribution_legacy::accounts::MigrateTdaMerkleRootUploadAuthority {
          //                 tip_distribution_account: tip_distribution_pda,
          //                 merkle_root_upload_config: merkle_root_upload_config_pda,
          //             }
          //             .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }

          // Commands::CloseClaimStatus {
          //     vote_account,
          //     epoch,
          //     claimant,
          // } => {
          //     let (tip_distribution_pda, _tip_distribution_bump) =
          //         derive_tip_distribution_account_address(&program_id, &vote_account, epoch);
          //     let (claim_status_pda, _claim_status_bump) =
          //         derive_claim_status_account_address(&program_id, &claimant, &tip_distribution_pda);

          //     let ix = Instruction {
          //         program_id,
          //         data: jito_tip_distribution_legacy::instruction::CloseClaimStatus.data(),
          //         accounts: jito_tip_distribution_legacy::accounts::CloseClaimStatus {
          //             config: config_pda,
          //             claim_status: claim_status_pda,
          //             claim_status_payer: keypair.pubkey(),
          //         }
          //         .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }

          // Commands::CloseTipDistributionAccount {
          //     vote_account,
          //     epoch,
          // } => {
          //     let (tip_distribution_pda, _tip_distribution_bump) =
          //         derive_tip_distribution_account_address(&program_id, &vote_account, epoch);

          //     let ix = Instruction {
          //         program_id,
          //         data: jito_tip_distribution_legacy::instruction::CloseTipDistributionAccount {
          //             _epoch: epoch,
          //         }
          //         .data(),
          //         accounts: jito_tip_distribution_legacy::accounts::CloseTipDistributionAccount {
          //             config: config_pda,
          //             expired_funds_account: keypair.pubkey(),
          //             tip_distribution_account: tip_distribution_pda,
          //             validator_vote_account: vote_account,
          //             signer: keypair.pubkey(),
          //         }
          //         .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }

          // Commands::Claim {
          //     vote_account,
          //     epoch,
          //     claimant,
          //     amount,
          // } => {
          //     let (tip_distribution_pda, _tip_distribution_bump) =
          //         derive_tip_distribution_account_address(&program_id, &vote_account, epoch);
          //     let (claim_status_pda, claim_status_bump) =
          //         derive_claim_status_account_address(&program_id, &claimant, &tip_distribution_pda);

          //     let proof = vec![];

          //     let ix = Instruction {
          //         program_id,
          //         data: jito_tip_distribution_legacy::instruction::Claim {
          //             bump: claim_status_bump,
          //             amount,
          //             proof,
          //         }
          //         .data(),
          //         accounts: jito_tip_distribution_legacy::accounts::Claim {
          //             config: config_pda,
          //             tip_distribution_account: tip_distribution_pda,
          //             merkle_root_upload_authority: keypair.pubkey(),
          //             claim_status: claim_status_pda,
          //             claimant,
          //             payer: keypair.pubkey(),
          //             system_program: system_program::ID,
          //         }
          //         .to_account_metas(None),
          //     };

          //     let blockhash = client.get_latest_blockhash()?;
          //     let tx = Transaction::new_signed_with_payer(
          //         &[ix],
          //         Some(&keypair.pubkey()),
          //         &[keypair],
          //         blockhash,
          //     );

          //     client.send_transaction(&tx)?;
          // }
    }

    Ok(())
}
