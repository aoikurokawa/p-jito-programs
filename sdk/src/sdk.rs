use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

use crate::instruction::JitoTipPaymentInstruction;

pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    tip_payment_account_0: &Pubkey,
    tip_payment_account_1: &Pubkey,
    tip_payment_account_2: &Pubkey,
    tip_payment_account_3: &Pubkey,
    tip_payment_account_4: &Pubkey,
    tip_payment_account_5: &Pubkey,
    tip_payment_account_6: &Pubkey,
    tip_payment_account_7: &Pubkey,
    payer: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*config, false),
        AccountMeta::new(*tip_payment_account_0, false),
        AccountMeta::new(*tip_payment_account_1, false),
        AccountMeta::new(*tip_payment_account_2, false),
        AccountMeta::new(*tip_payment_account_3, false),
        AccountMeta::new(*tip_payment_account_4, false),
        AccountMeta::new(*tip_payment_account_5, false),
        AccountMeta::new(*tip_payment_account_6, false),
        AccountMeta::new(*tip_payment_account_7, false),
        AccountMeta::new(*payer, true),
        AccountMeta::new_readonly(solana_system_interface::program::id(), false),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: vec![0],
    }
}
