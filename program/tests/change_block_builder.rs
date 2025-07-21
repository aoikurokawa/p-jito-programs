use helpers::lite_svm_with_programs;

mod helpers;

#[cfg(test)]
mod tests {
    use jito_tip_payment_program::{
        CONFIG_ACCOUNT_SEED, TIP_ACCOUNT_SEED_0, TIP_ACCOUNT_SEED_1, TIP_ACCOUNT_SEED_2,
        TIP_ACCOUNT_SEED_3, TIP_ACCOUNT_SEED_4, TIP_ACCOUNT_SEED_5, TIP_ACCOUNT_SEED_6,
        TIP_ACCOUNT_SEED_7,
    };
    use jito_tip_payment_sdk::sdk::{change_block_builder, change_tip_receiver, initialize_config};
    use solana_keypair::Keypair;
    use solana_pubkey::{pubkey, Pubkey};
    use solana_signer::Signer;
    use solana_transaction::Transaction;

    use super::*;

    #[test_log::test]
    fn change_block_builder_success() {
        let user_kp = Keypair::new();
        let mut svm = lite_svm_with_programs();
        svm.airdrop(&user_kp.pubkey(), 100_000_000).unwrap();

        let program_id = pubkey!("T1pyyaTNZsKv2WcRAB8oVnk93mLJw2XzjtVYqCsaHqf");

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
            svm.latest_blockhash(),
        );

        let _tx_resp = svm.send_transaction(transaction).unwrap();

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
            svm.latest_blockhash(),
        );

        let _tx_resp = svm.send_transaction(transaction).unwrap();

        let new_block_builder = Keypair::new();

        let ix = change_block_builder(
            &program_id,
            &config_pubkey,
            &new_tip_receiver.pubkey(),
            &block_builder.pubkey(),
            &new_block_builder.pubkey(),
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
            svm.latest_blockhash(),
        );

        let _tx_resp = svm.send_transaction(transaction).unwrap();
    }
}
