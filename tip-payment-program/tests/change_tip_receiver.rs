#[cfg(test)]
mod tests {
    use jito_tip_payment_program::{
        CONFIG_ACCOUNT_SEED, TIP_ACCOUNT_SEED_0, TIP_ACCOUNT_SEED_1, TIP_ACCOUNT_SEED_2,
        TIP_ACCOUNT_SEED_3, TIP_ACCOUNT_SEED_4, TIP_ACCOUNT_SEED_5, TIP_ACCOUNT_SEED_6,
        TIP_ACCOUNT_SEED_7,
    };
    use jito_tip_payment_sdk::sdk::{change_tip_receiver, initialize_config};
    use solana_commitment_config::CommitmentLevel;
    use solana_keypair::Keypair;
    use solana_native_token::sol_to_lamports;
    use solana_program_test::ProgramTest;
    use solana_pubkey::Pubkey;
    use solana_signer::Signer;
    use solana_system_interface::instruction::transfer;
    use solana_transaction::Transaction;

    #[tokio::test]
    async fn change_tip_receiver_success() {
        let program_id = Pubkey::new_from_array(jito_tip_payment_program::id());

        let context = ProgramTest::new("jito_tip_payment_program", program_id, None)
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

        let (config_pubkey, _) = Pubkey::find_program_address(&[CONFIG_ACCOUNT_SEED], &program_id);
        let (tip_payment_0_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_0], &program_id);
        let (tip_payment_1_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_1], &program_id);
        let (tip_payment_2_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_2], &program_id);
        let (tip_payment_3_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_3], &program_id);
        let (tip_payment_4_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_4], &program_id);
        let (tip_payment_5_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_5], &program_id);
        let (tip_payment_6_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_6], &program_id);
        let (tip_payment_7_pubkey, _) =
            Pubkey::find_program_address(&[TIP_ACCOUNT_SEED_7], &program_id);

        let ix = initialize_config(
            &program_id,
            &config_pubkey,
            &tip_payment_0_pubkey,
            &tip_payment_1_pubkey,
            &tip_payment_2_pubkey,
            &tip_payment_3_pubkey,
            &tip_payment_4_pubkey,
            &tip_payment_5_pubkey,
            &tip_payment_6_pubkey,
            &tip_payment_7_pubkey,
            &user_kp.pubkey(),
        );

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

        let old_tip_receiver = Keypair::new();
        let new_tip_receiver = Keypair::new();
        let block_builder = Keypair::new();

        let ix = change_tip_receiver(
            &program_id,
            &config_pubkey,
            &old_tip_receiver.pubkey(),
            &new_tip_receiver.pubkey(),
            &block_builder.pubkey(),
            &tip_payment_0_pubkey,
            &tip_payment_1_pubkey,
            &tip_payment_2_pubkey,
            &tip_payment_3_pubkey,
            &tip_payment_4_pubkey,
            &tip_payment_5_pubkey,
            &tip_payment_6_pubkey,
            &tip_payment_7_pubkey,
            &user_kp.pubkey(),
        );

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
