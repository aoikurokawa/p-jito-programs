use jito_tip_distribution_sdk::instruction::JitoTipDistributionInstruction;
use pinocchio::{
    account_info::AccountInfo, entrypoint, msg, program_error::ProgramError, pubkey::Pubkey,
    ProgramResult,
};

use crate::{
    close_claim_status::process_close_claim_status, initialize::process_initialize,
    initialize_tip_distribution_account::process_initialize_tip_distribution_account,
    update_config::process_update_config, upload_merkle_root::process_upload_merkle_root,
};

mod close_claim_status;
mod initialize;
mod initialize_tip_distribution_account;
mod update_config;
mod upload_merkle_root;

entrypoint!(process_instruction);

pinocchio_pubkey::declare_id!("4R3gSG8BpU4t19KYj8CfnbtRpnT8gtk4dvTHxVRwc2r7");

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = JitoTipDistributionInstruction::try_from_slice(instruction_data)?;

    match instruction {
        JitoTipDistributionInstruction::Initialize {
            authority,
            expired_funds_account,
            num_epochs_valid,
            max_validator_commission_bps,
            bump,
        } => {
            msg!("Instruction: InitializeConfig");
            process_initialize(
                program_id,
                accounts,
                authority,
                expired_funds_account,
                num_epochs_valid,
                max_validator_commission_bps,
                bump,
            )
        }
        JitoTipDistributionInstruction::InitializeTipDistributionAccount {
            merkle_root_upload_authority,
            validator_commission_bps,
            bump,
        } => {
            msg!("Instruction: InitializeTipDistributionAccount");
            process_initialize_tip_distribution_account(
                program_id,
                accounts,
                merkle_root_upload_authority,
                validator_commission_bps,
                bump,
            )
        }
        JitoTipDistributionInstruction::UpdateConfig {
            authority,
            expired_funds_account,
            num_epochs_valid,
            max_validator_commission_bps,
        } => {
            msg!("Instruction: UpdateConfig");
            process_update_config(
                program_id,
                accounts,
                authority,
                expired_funds_account,
                num_epochs_valid,
                max_validator_commission_bps,
            )
        }
        JitoTipDistributionInstruction::UploadMerkleRoot {
            root,
            max_total_claim,
            max_num_nodes,
        } => {
            msg!("Instruction: UploadMerkleRoot");
            process_upload_merkle_root(program_id, accounts, root, max_total_claim, max_num_nodes)
        }
        JitoTipDistributionInstruction::CloseClaimStatus => {
            msg!("Instruction: CloseClaimStatus");
            process_close_claim_status(program_id, accounts)
        }
        _ => todo!(),
    }
}

pub fn verify(proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    let mut computed_hash = leaf;
    for proof_element in proof.into_iter() {
        if computed_hash <= proof_element {
            // Hash(current computed hash + current element of the proof)
            computed_hash =
                solana_program::hash::hashv(&[&[1u8], &computed_hash, &proof_element]).to_bytes();
        } else {
            // Hash(current element of the proof + current computed hash)
            computed_hash =
                solana_program::hash::hashv(&[&[1u8], &proof_element, &computed_hash]).to_bytes();
        }
    }
    // Check if the computed hash (root) is equal to the provided root
    computed_hash == root
}
