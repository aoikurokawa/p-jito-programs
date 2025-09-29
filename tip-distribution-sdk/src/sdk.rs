use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

#[allow(clippy::too_many_arguments)]
pub fn initialize_config(
    program_id: &Pubkey,
    config: &Pubkey,
    initializer: &Pubkey,
) -> Instruction {
    let accounts = vec![
        AccountMeta::new(*config, false),
        AccountMeta::new_readonly(solana_system_interface::program::id(), false),
        AccountMeta::new(*initializer, true),
    ];
    Instruction {
        program_id: *program_id,
        accounts,
        data: vec![175, 175, 109, 31, 13, 152, 155, 237],
    }
}
