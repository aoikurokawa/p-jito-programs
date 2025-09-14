use std::sync::Arc;

use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use jito_tip_distribution_legacy::state::Config;
use solana_client::rpc_client::RpcClient;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_pubkey::Pubkey;
use solana_sdk::system_program;
use solana_signer::Signer;
use solana_transaction::Transaction;

use crate::tip_distribution::{ConfigActions, TipDistributionCommands};

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
        }
    }

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
}
