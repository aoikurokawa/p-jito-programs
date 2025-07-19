use litesvm::LiteSVM;
use solana_pubkey::pubkey;

/// Get LiteSvm with myproject loaded.
pub fn lite_svm_with_programs() -> LiteSVM {
    let mut svm = LiteSVM::new();
    let bytes =
        include_bytes!("../../../target/sbpf-solana-solana/release//jito_tip_payment_program.so");
    svm.add_program(
        pubkey!("22222222222222222222222222222222222222222222"),
        bytes,
    );

    svm
}
