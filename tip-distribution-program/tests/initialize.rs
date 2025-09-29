#[cfg(test)]
mod tests {
    use jito_tip_distribution_core::config::Config;
    use jito_tip_distribution_sdk::{self, sdk::initialize_config};
    use solana_commitment_config::CommitmentLevel;
    use solana_keypair::Keypair;
    use solana_native_token::sol_to_lamports;
    use solana_program_test::ProgramTest;
    use solana_pubkey::Pubkey;
    use solana_signer::Signer;
    use solana_system_interface::instruction::transfer;
    use solana_transaction::Transaction;

    #[tokio::test]
    async fn initialize_config_success() {
        let program_id = Pubkey::new_from_array(jito_tip_distribution_program::id());

        let context = ProgramTest::new("jito_tip_distribution_program", program_id, None)
            .start_with_context()
            .await;

        let user_kp = Keypair::new();

        let blockhash = context.banks_client.get_latest_blockhash().await.unwrap();
        context
            .banks_client
            .process_transaction_with_preflight_and_commitment(
                Transaction::new_signed_with_payer(
                    &[transfer(
                        &context.payer.pubkey(),
                        &user_kp.pubkey(),
                        sol_to_lamports(1f64),
                    )],
                    Some(&context.payer.pubkey()),
                    &[&context.payer],
                    blockhash,
                ),
                CommitmentLevel::Processed,
            )
            .await
            .unwrap();

        let (config_pubkey, _) = Pubkey::find_program_address(&[Config::SEED], &program_id);

        let ix = initialize_config(&program_id, &config_pubkey, &user_kp.pubkey());

        let transaction = Transaction::new_signed_with_payer(
            &[ix],
            Some(&user_kp.pubkey()),
            &[&user_kp],
            blockhash,
        );

        let _tx_resp = context
            .banks_client
            .send_transaction(transaction)
            .await
            .unwrap();
    }
}
