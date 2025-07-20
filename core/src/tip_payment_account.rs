use jito_tip_payment_sdk::error::TipPaymentError;
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::rent::Rent,
};
use pinocchio_system::instructions::CreateAccount;
use shank::ShankAccount;

#[derive(Debug, Default, Clone, ShankAccount)]
#[repr(C)]
pub struct TipPaymentAccount {}

impl TipPaymentAccount {
    pub const SIZE: usize = 8;

    /// Initialize a [`TipPaymentAccount`]
    pub fn initialize(
        seeds: &[u8],
        program_id: &Pubkey,
        account_info: &AccountInfo,
        payer: &AccountInfo,
        system_program: &AccountInfo,
        rent: &Rent,
    ) -> Result<u8, ProgramError> {
        let space = TipPaymentAccount::SIZE;

        // Validate PDA
        let (pubkey, bump) = find_program_address(&[seeds], program_id);
        if pubkey.ne(account_info.key()) {
            return Err(ProgramError::InvalidAccountOwner);
        }

        // CPI to system program to create account
        // let current_lamports = account_info.lamports();
        if account_info.lamports().ne(&0) {
            return Err(ProgramError::InvalidAccountData);
        }

        let required_lamports = rent.minimum_balance(space);
        // let cpi_accounts = anchor_lang::system_program::CreateAccount {
        //     from: payer.to_account_info(),
        //     to: account_info.to_account_info(),
        // };
        // let cpi_context = CpiContext::new(system_program.to_account_info(), cpi_accounts);
        // anchor_lang::system_program::create_account(
        //     cpi_context.with_signer(&[&[seeds, &[bump]]]),
        //     required_lamports,
        //     space as u64,
        //     program_id,
        // )?;

        // // set the discriminator
        // let mut account_data: std::cell::RefMut<'_, &mut [u8]> =
        //     account_info.try_borrow_mut_data()?;
        // account_data[..TipPaymentAccount::DISCRIMINATOR.len()]
        //     .copy_from_slice(TipPaymentAccount::DISCRIMINATOR);

        let bindings = [bump];
        let seeds = [Seed::from(seeds), Seed::from(&bindings)];
        let signers = [Signer::from(&seeds)];
        CreateAccount {
            from: payer,
            to: account_info,
            lamports: required_lamports,
            space: space as u64,
            owner: program_id,
        }
        .invoke_signed(&signers)?;

        Ok(bump)
    }

    /// Drains the tip accounts, leaves enough lamports for rent exemption.
    #[inline(always)]
    pub fn drain_accounts(rent: &Rent, accounts: &[&AccountInfo]) -> Result<u64, ProgramError> {
        let mut total_tips: u64 = 0;
        for a in accounts {
            total_tips = total_tips
                .checked_add(Self::drain_account(rent, a)?)
                .ok_or(TipPaymentError::ArithmeticError)?;
        }

        Ok(total_tips)
    }

    #[inline(always)]
    fn drain_account(rent: &Rent, account: &AccountInfo) -> Result<u64, ProgramError> {
        // Tips after rent exemption.
        let tips = account
            .lamports()
            .checked_sub(rent.minimum_balance(account.data_len()))
            .ok_or(TipPaymentError::ArithmeticError)?;

        let mut lamports = account.try_borrow_mut_lamports()?;
        *lamports = account
            .lamports()
            .checked_sub(tips)
            .ok_or(TipPaymentError::ArithmeticError)?;

        Ok(tips)
    }
}
