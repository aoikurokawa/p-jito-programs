use pinocchio::{
    account_info::AccountInfo, instruction::Signer, program_error::ProgramError, pubkey::Pubkey,
    sysvars::rent::Rent, ProgramResult,
};
use pinocchio_system::instructions::{Allocate, Assign, CreateAccount, Transfer};

pub mod loader;

/// Creates a new account or initializes an existing account
/// # Arguments
/// * `payer` - The account that will pay for the lamports
/// * `new_account` - The account to create or initialize
/// * `system_program` - The system program account
/// * `program_owner` - The owner of the program
/// * `rent` - The rent sysvar
/// * `space` - The space to allocate
/// * `seeds` - The seeds to use for the PDA
/// # Returns
/// * `ProgramResult` - The result of the operation
#[inline(always)]
pub fn create_account(
    payer: &AccountInfo,
    new_account: &AccountInfo,
    _system_program: &AccountInfo,
    program_owner: &Pubkey,
    rent: &Rent,
    space: u64,
    signers: &[Signer],
) -> ProgramResult {
    let current_lamports = *new_account.try_borrow_lamports()?;
    if current_lamports == 0 {
        // If there are no lamports in the new account, we create it with the create_account instruction
        CreateAccount {
            from: payer,
            to: new_account,
            lamports: rent.minimum_balance(space as usize),
            space,
            owner: program_owner,
        }
        .invoke_signed(signers)
    } else {
        // someone can transfer lamports to accounts before they're initialized
        // in that case, creating the account won't work.
        // in order to get around it, you need to find the account with enough lamports to be rent exempt,
        // then allocate the required space and set the owner to the current program
        let required_lamports = rent
            .minimum_balance(space as usize)
            .max(1)
            .saturating_sub(current_lamports);
        if required_lamports > 0 {
            Transfer {
                from: payer,
                to: new_account,
                lamports: required_lamports,
            }
            .invoke()?;
        }
        // Allocate space.
        Allocate {
            account: new_account,
            space,
        }
        .invoke_signed(signers)?;

        // Assign to the specified program
        Assign {
            account: new_account,
            owner: program_owner,
        }
        .invoke_signed(signers)
    }
}

/// Closes the program account
///
/// # Safety
pub unsafe fn close_program_account(
    program_id: &Pubkey,
    account_to_close: &AccountInfo,
    destination_account: &AccountInfo,
) -> ProgramResult {
    // Check if the account is owned by the program
    if account_to_close.owner() != program_id {
        return Err(ProgramError::IllegalOwner);
    }

    let lamports = destination_account.borrow_mut_lamports_unchecked();
    *lamports = destination_account
        .lamports()
        .checked_add(account_to_close.lamports())
        .ok_or(ProgramError::ArithmeticOverflow)?;

    *account_to_close.borrow_mut_lamports_unchecked() = 0;

    account_to_close.assign(&pinocchio_system::id());
    account_to_close.realloc(0, false)?;

    Ok(())
}

pub fn realloc(
    account: &AccountInfo,
    new_size: usize,
    payer: &AccountInfo,
    rent: &Rent,
) -> ProgramResult {
    let new_minimum_balance = rent.minimum_balance(new_size);

    let lamports_diff = new_minimum_balance.saturating_sub(account.lamports());
    Transfer {
        from: payer,
        to: account,
        lamports: lamports_diff,
    }
    .invoke()?;
    account.realloc(new_size, false)?;
    Ok(())
}
