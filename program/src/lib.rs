#![no_std]

use core::convert::TryFrom;

use change_tip_receiver::process_change_tip_receiver;
use initialize::process_initialize;
use jito_tip_payment_core::{fees::Fees, tip_payment_account::TipPaymentAccount};
use jito_tip_payment_sdk::{error::TipPaymentError, instruction::JitoTipPaymentInstruction};
use pinocchio::{
    account_info::AccountInfo, entrypoint, msg, program_error::ProgramError, pubkey::Pubkey,
    sysvars::rent::Rent, ProgramResult,
};
use solana_sdk_ids::{
    bpf_loader, bpf_loader_deprecated, bpf_loader_upgradeable, loader_v4, native_loader,
    secp256r1_program,
};

mod change_block_builder;
mod change_tip_receiver;
mod initialize;

entrypoint!(process_instruction);
// nostd_panic_handler!();

pinocchio_pubkey::declare_id!("22222222222222222222222222222222222222222222");

/// We've decided to hardcode the seeds, effectively meaning the following PDAs owned by this program are singleton.
///
/// This ensures that `initialize` can only be invoked once,
/// otherwise the tx would fail since the accounts would have
/// already been initialized on subsequent calls.
pub const CONFIG_ACCOUNT_SEED: &[u8] = b"CONFIG_ACCOUNT";
pub const TIP_ACCOUNT_SEED_0: &[u8] = b"TIP_ACCOUNT_0";
pub const TIP_ACCOUNT_SEED_1: &[u8] = b"TIP_ACCOUNT_1";
pub const TIP_ACCOUNT_SEED_2: &[u8] = b"TIP_ACCOUNT_2";
pub const TIP_ACCOUNT_SEED_3: &[u8] = b"TIP_ACCOUNT_3";
pub const TIP_ACCOUNT_SEED_4: &[u8] = b"TIP_ACCOUNT_4";
pub const TIP_ACCOUNT_SEED_5: &[u8] = b"TIP_ACCOUNT_5";
pub const TIP_ACCOUNT_SEED_6: &[u8] = b"TIP_ACCOUNT_6";
pub const TIP_ACCOUNT_SEED_7: &[u8] = b"TIP_ACCOUNT_7";

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if *program_id != id() {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = JitoTipPaymentInstruction::try_from_slice(instruction_data)?;

    match instruction {
        JitoTipPaymentInstruction::Initialize => {
            msg!("Instruction: InitializeConfig");
            process_initialize(program_id, accounts)
        }
        JitoTipPaymentInstruction::ChangeTipReceiver => {
            msg!("Instruction: ChangeTipReceiver");
            process_change_tip_receiver(program_id, accounts)
        }
        JitoTipPaymentInstruction::ChangeBlockBuilder {
            block_builder_commission,
        } => {
            msg!("Instruction: ChangeBlockBuilder");
            process
        }
    }
}

#[inline(always)]
unsafe fn is_program(account: &AccountInfo) -> bool {
    account.owner() == &bpf_loader::id().to_bytes()
        || *account.owner() == bpf_loader_deprecated::id().to_bytes()
        || *account.owner() == bpf_loader_upgradeable::id().to_bytes()
        || *account.owner() == loader_v4::id().to_bytes()
        || *account.owner() == native_loader::id().to_bytes()

        || *account.key() == native_loader::id().to_bytes()
        // can remove once feature enable_secp256r1_precompile gets activated
        || *account.key() == secp256r1_program::id().to_bytes()

    // note: SIMD-0162 will remove support for this flag: https://github.com/solana-foundation/solana-improvement-documents/blob/main/proposals/0162-remove-accounts-executable-flag-checks.md
    // || account.executable
}

/// Assumptions:
/// - The transfer_amount are "dangling" lamports and need to be transferred somewhere to have a balanced instruction.
/// - The receiver needs to remain rent exempt
#[inline(always)]
unsafe fn transfer_or_credit_tip_pda(
    rent: &Rent,
    receiver: &AccountInfo,
    transfer_amount: u64,
    tip_pda_fallback: &AccountInfo,
) -> Result<u64, ProgramError> {
    let balance_post_transfer = receiver
        .lamports()
        .checked_add(transfer_amount)
        .ok_or(TipPaymentError::ArithmeticError)?;

    // Ensure the transfer amount is greater than 0, the account is rent-exempt after the transfer, and
    // the transfer is not to a program
    let can_transfer = transfer_amount > 0
        && rent.is_exempt(balance_post_transfer, receiver.data_len())
        // programs can't receive lamports until remove_accounts_executable_flag_checks is activated
        && !is_program(receiver);

    if can_transfer {
        *receiver.try_borrow_mut_lamports()? = balance_post_transfer;
        // Transfer {
        //     from:
        //     to:
        //     amount
        // }.invoke()?;
        Ok(transfer_amount)
    } else {
        // These lamports can't be left dangling
        let new_tip_pda_balance = tip_pda_fallback
            .lamports()
            .checked_add(transfer_amount)
            .ok_or(TipPaymentError::ArithmeticError)?;
        *tip_pda_fallback.try_borrow_mut_lamports()? = new_tip_pda_balance;
        Ok(0)
    }
}

/// Handles payment of the tips to the block builder and tip receiver
/// Assumptions:
/// - block_builder_commission_percent is a valid number (<= 100)
#[inline(always)]
unsafe fn handle_payments(
    rent: &Rent,
    tip_accounts: &[&AccountInfo],
    tip_receiver: &AccountInfo,
    block_builder: &AccountInfo,
    block_builder_commission_percent: u64,
) -> Result<(), ProgramError> {
    let total_tips = TipPaymentAccount::drain_accounts(rent, tip_accounts)?;

    let Fees {
        block_builder_fee_lamports,
        tip_receiver_fee_lamports,
    } = Fees::calculate(total_tips, block_builder_commission_percent)?;

    let amount_transferred_to_tip_receiver = if tip_receiver_fee_lamports > 0 {
        let amount_transferred_to_tip_receiver = transfer_or_credit_tip_pda(
            rent,
            tip_receiver,
            tip_receiver_fee_lamports,
            tip_accounts.first().unwrap(),
        )?;
        if amount_transferred_to_tip_receiver == 0 {
            // msg!(
            //     "WARN: did not transfer tip receiver lamports to {:?}",
            //     tip_receiver.key()
            // );
        }
        amount_transferred_to_tip_receiver
    } else {
        0
    };

    let amount_transferred_to_block_builder = if block_builder_fee_lamports > 0 {
        let amount_transferred_to_block_builder = transfer_or_credit_tip_pda(
            rent,
            block_builder,
            block_builder_fee_lamports,
            tip_accounts.first().unwrap(),
        )?;
        if amount_transferred_to_block_builder == 0 {
            // msg!(
            //     "WARN: did not transfer block builder lamports to {:?}",
            //     block_builder.key()
            // );
        }
        amount_transferred_to_block_builder
    } else {
        0
    };

    if amount_transferred_to_tip_receiver > 0 || amount_transferred_to_block_builder > 0 {
        let tip_receiver = if amount_transferred_to_tip_receiver > 0 {
            tip_receiver.key()
        } else {
            &Pubkey::default()
        };
        let block_builder = if amount_transferred_to_block_builder > 0 {
            block_builder.key()
        } else {
            &Pubkey::default()
        };

        // emit!(TipsClaimed {
        //     tip_receiver,
        //     tip_receiver_amount: amount_transferred_to_tip_receiver,
        //     block_builder,
        //     block_builder_amount: amount_transferred_to_block_builder,
        // });
    }
    Ok(())
}
