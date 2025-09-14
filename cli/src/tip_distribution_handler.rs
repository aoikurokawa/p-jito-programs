use std::{str::FromStr, sync::Arc};

use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use jito_tip_distribution_legacy::state::{Config, TipDistributionAccount};
use jito_tip_distribution_sdk_legacy::derive_tip_distribution_account_address;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_sdk::system_program;
use solana_signer::Signer;
use solana_transaction::Transaction;

use crate::tip_distribution::{
    ConfigActions, TipDistributionAccountActions, TipDistributionCommands,
};

pub struct TipDistributionCliHandler {
    /// RPC Client
    client: RpcClient,

    /// Keypair
    keypair: Arc<Keypair>,

    /// The Pubkey of Jito Restaking Program ID
    program_id: Pubkey,

    /// Config PDA
    config_pda: Pubkey,

    /// Config Bump
    config_bump: u8,
    // The configuration of CLI
    // cli_config: CliConfig,
}

impl TipDistributionCliHandler {
    pub const fn new(
        client: RpcClient,
        keypair: Arc<Keypair>,
        program_id: Pubkey,
        config_pda: Pubkey,
        config_bump: u8,
    ) -> Self {
        Self {
            client,
            keypair,
            program_id,
            config_pda,
            config_bump,
        }
    }

    /// Handle tip_distribution_program operations
    pub fn handle(&self, action: TipDistributionCommands) -> anyhow::Result<()> {
        match action {
            TipDistributionCommands::Config {
                action:
                    ConfigActions::Initialize {
                        authority,
                        expired_funds_account,
                        num_epochs_valid,
                        max_validator_commission_bps,
                    },
            } => self.initialize_config(
                authority,
                expired_funds_account,
                num_epochs_valid,
                max_validator_commission_bps,
            ),
            TipDistributionCommands::Config {
                action: ConfigActions::Get,
            } => self.get_config(),
            TipDistributionCommands::TipDistributionAccount {
                action:
                    TipDistributionAccountActions::Initialize {
                        vote_account,
                        merkle_root_upload_authority,
                        validator_commission_bps,
                    },
            } => self.initialize_tip_distribution_account(
                vote_account,
                merkle_root_upload_authority,
                validator_commission_bps,
            ),
            TipDistributionCommands::TipDistributionAccount {
                action:
                    TipDistributionAccountActions::Get {
                        vote_account,
                        epoch,
                    },
            } => self.get_tip_distribution_account(vote_account, epoch),
            TipDistributionCommands::TipDistributionAccount {
                action:
                    TipDistributionAccountActions::Close {
                        vote_account,
                        epoch,
                    },
            } => self.close_tip_distribution_account(vote_account, epoch),
        }
    }

    /// Get TipDistribution config
    pub fn get_config(&self) -> anyhow::Result<()> {
        println!("Config Account Address: {}", self.config_pda);

        let config_data = self.client.get_account(&self.config_pda)?.data;
        let config: Config = Config::try_deserialize(&mut config_data.as_slice())?;

        println!("Config Account Data:");
        println!("  Authority: {}", config.authority);
        println!("  Expired Funds Account: {}", config.expired_funds_account);
        println!("  Num Epochs Valid: {}", config.num_epochs_valid);
        println!(
            "  Max Validator Commission BPS: {}",
            config.max_validator_commission_bps
        );
        println!("  Bump: {}", config.bump);

        Ok(())
    }

    /// Initialize config
    pub fn initialize_config(
        &self,
        authority: Pubkey,
        expired_funds_account: Pubkey,
        num_epochs_valid: u64,
        max_validator_commission_bps: u16,
    ) -> anyhow::Result<()> {
        let ix = Instruction {
            program_id: self.program_id,
            data: jito_tip_distribution_legacy::instruction::Initialize {
                authority,
                expired_funds_account,
                num_epochs_valid,
                max_validator_commission_bps,
                bump: self.config_bump,
            }
            .data(),
            accounts: jito_tip_distribution_legacy::accounts::Initialize {
                config: self.config_pda,
                system_program: system_program::ID,
                initializer: self.keypair.pubkey(),
            }
            .to_account_metas(None),
        };

        let blockhash = self.client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.keypair.pubkey()),
            &[self.keypair.clone()],
            blockhash,
        );

        self.client.send_transaction(&tx)?;

        Ok(())
    }

    pub fn get_tip_distribution_account(
        &self,
        vote_account: String,
        epoch: u64,
    ) -> anyhow::Result<()> {
        let vote_pubkey = Pubkey::from_str(&vote_account)?;
        let (tip_dist_pda, _) =
            derive_tip_distribution_account_address(&self.program_id, &vote_pubkey, epoch);
        println!("Tip Distribution Account Address: {}", tip_dist_pda);

        let account_data = self.client.get_account(&tip_dist_pda)?.data;
        let tip_dist: TipDistributionAccount =
            TipDistributionAccount::try_deserialize(&mut account_data.as_slice())?;

        println!("Tip Distribution Account Data:");
        println!("  Vote Account: {}", tip_dist.validator_vote_account);
        println!(
            "  Merkle Root Upload Authority: {}",
            tip_dist.merkle_root_upload_authority
        );
        println!("  Epoch Created At: {}", tip_dist.epoch_created_at);
        println!(
            "  Validator Commission BPS: {}",
            tip_dist.validator_commission_bps
        );
        println!("  Expires At: {}", tip_dist.expires_at);
        println!("  Bump: {}", tip_dist.bump);

        if let Some(merkle_root) = tip_dist.merkle_root {
            println!("  Merkle Root:");
            println!("    Root: {:?}", merkle_root.root);
            println!("    Max Total Claim: {}", merkle_root.max_total_claim);
            println!("    Max Num Nodes: {}", merkle_root.max_num_nodes);
            println!(
                "    Total Funds Claimed: {}",
                merkle_root.total_funds_claimed
            );
            println!("    Num Nodes Claimed: {}", merkle_root.num_nodes_claimed);
        } else {
            println!("  Merkle Root: None");
        }

        Ok(())
    }

    pub fn initialize_tip_distribution_account(
        &self,
        vote_account: Pubkey,
        merkle_root_upload_authority: Pubkey,
        validator_commission_bps: u16,
    ) -> anyhow::Result<()> {
        let epoch = self.client.get_epoch_info()?.epoch;
        let (tip_distribution_pubkey, tip_distribution_bump) =
            derive_tip_distribution_account_address(&self.program_id, &vote_account, epoch);

        let ix = Instruction {
            program_id: self.program_id,
            data: jito_tip_distribution_legacy::instruction::InitializeTipDistributionAccount {
                merkle_root_upload_authority,
                validator_commission_bps,
                bump: tip_distribution_bump,
            }
            .data(),
            accounts: jito_tip_distribution_legacy::accounts::InitializeTipDistributionAccount {
                config: self.config_pda,
                tip_distribution_account: tip_distribution_pubkey,
                validator_vote_account: vote_account,
                signer: self.keypair.pubkey(),
                system_program: system_program::ID,
            }
            .to_account_metas(None),
        };

        let blockhash = self.client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.keypair.pubkey()),
            &[self.keypair.clone()],
            blockhash,
        );

        self.client.send_transaction(&tx)?;

        Ok(())
    }

    pub fn close_tip_distribution_account(
        &self,
        vote_account: Pubkey,
        epoch: u64,
    ) -> anyhow::Result<()> {
        let (tip_distribution_pda, _tip_distribution_bump) =
            derive_tip_distribution_account_address(&self.program_id, &vote_account, epoch);

        let ix = Instruction {
            program_id: self.program_id,
            data: jito_tip_distribution_legacy::instruction::CloseTipDistributionAccount {
                _epoch: epoch,
            }
            .data(),
            accounts: jito_tip_distribution_legacy::accounts::CloseTipDistributionAccount {
                config: self.config_pda,
                expired_funds_account: self.keypair.pubkey(),
                tip_distribution_account: tip_distribution_pda,
                validator_vote_account: vote_account,
                signer: self.keypair.pubkey(),
            }
            .to_account_metas(None),
        };

        let blockhash = self.client.get_latest_blockhash()?;
        let tx = Transaction::new_signed_with_payer(
            &[ix],
            Some(&self.keypair.pubkey()),
            &[self.keypair.clone()],
            blockhash,
        );

        self.client.send_transaction(&tx)?;

        Ok(())
    }
}
