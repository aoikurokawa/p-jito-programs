use jito_tip_payment_core::{
    config::Config, init_bumps::InitBumps, load_mut_unchecked,
    tip_payment_account::TipPaymentAccount, Transmutable,
};
use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::{find_program_address, Pubkey},
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};
use pinocchio_system::instructions::CreateAccount;

use crate::{
    CONFIG_ACCOUNT_SEED, TIP_ACCOUNT_SEED_0, TIP_ACCOUNT_SEED_1, TIP_ACCOUNT_SEED_2,
    TIP_ACCOUNT_SEED_3, TIP_ACCOUNT_SEED_4, TIP_ACCOUNT_SEED_5, TIP_ACCOUNT_SEED_6,
    TIP_ACCOUNT_SEED_7,
};

// pub struct DepositInstructionData {
//     pub amount: u64,
// }
//
// impl<'a> TryFrom<&'a [u8]> for DepositInstructionData {
//     type Error = ProgramError;
//
//     fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
//         if data.len() != size_of::<u64>() {
//             return Err(ProgramError::InvalidInstructionData);
//         }
//
//         let amount = u64::from_le_bytes(data.try_into().unwrap());
//
//         // Instruction Checks
//         if amount.eq(&0) {
//             return Err(ProgramError::InvalidInstructionData);
//         }
//
//         Ok(Self { amount })
//     }
// }

pub struct InitializeAccounts<'a> {
    /// Config
    pub config: &'a AccountInfo,

    /// Tip payment account 0
    pub tip_payment_account_0: &'a AccountInfo,

    /// Tip payment account 1
    pub tip_payment_account_1: &'a AccountInfo,

    /// Tip payment account 2
    pub tip_payment_account_2: &'a AccountInfo,

    /// Tip payment account 3
    pub tip_payment_account_3: &'a AccountInfo,

    /// Tip payment account 4
    pub tip_payment_account_4: &'a AccountInfo,

    /// Tip payment account 5
    pub tip_payment_account_5: &'a AccountInfo,

    /// Tip payment account 6
    pub tip_payment_account_6: &'a AccountInfo,

    /// Tip payment account 7
    pub tip_payment_account_7: &'a AccountInfo,

    /// Payer
    pub payer: &'a AccountInfo,

    /// System program
    pub system_program: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for InitializeAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [config, tip_payment_account_0, tip_payment_account_1, tip_payment_account_2, tip_payment_account_3, tip_payment_account_4, tip_payment_account_5, tip_payment_account_6, tip_payment_account_7, payer, system_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Accounts Checks
        // if !owner.is_signer() {
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        // if !vault.is_owned_by(&pinocchio_system::ID) {
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        // if vault.lamports().ne(&0) {
        //     return Err(ProgramError::InvalidAccountData);
        // }

        // let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
        // if vault.key().ne(&vault_key) {
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        // Return the accounts
        Ok(Self {
            config,
            tip_payment_account_0,
            tip_payment_account_1,
            tip_payment_account_2,
            tip_payment_account_3,
            tip_payment_account_4,
            tip_payment_account_5,
            tip_payment_account_6,
            tip_payment_account_7,
            payer,
            system_program,
        })
    }
}

pub struct Initialize<'a> {
    /// Program ID
    pub program_id: &'a Pubkey,

    /// Accounts
    pub accounts: InitializeAccounts<'a>,
    // pub instruction_data: InitializeInstructionData,
}

impl<'a> TryFrom<(&'a Pubkey, &'a [u8], &'a [AccountInfo])> for Initialize<'a> {
    type Error = ProgramError;

    fn try_from(
        (program_id, _data, accounts): (&'a Pubkey, &'a [u8], &'a [AccountInfo]),
    ) -> Result<Self, Self::Error> {
        let accounts = InitializeAccounts::try_from(accounts)?;
        // let instruction_data = DepositInstructionData::try_from(data)?;

        Ok(Self {
            program_id,
            accounts,
            // instruction_data,
        })
    }
}

impl<'a> Initialize<'a> {
    pub const DISCRIMINATOR: &'a u8 = &0;

    pub fn process(&mut self) -> ProgramResult {
        let rent = Rent::get()?;

        let space = Config::LEN;
        let required_lamports = rent.minimum_balance(space);

        let (_config_pubkey, config_bump) =
            find_program_address(&[CONFIG_ACCOUNT_SEED], self.program_id);

        let bindings = [config_bump];
        let seeds = [Seed::from(CONFIG_ACCOUNT_SEED), Seed::from(&bindings)];
        let signers = [Signer::from(&seeds)];
        CreateAccount {
            from: self.accounts.payer,
            to: self.accounts.config,
            lamports: required_lamports,
            space: space as u64,
            owner: self.program_id,
        }
        .invoke_signed(&signers)?;

        let config = unsafe {
            load_mut_unchecked::<Config>(self.accounts.config.borrow_mut_data_unchecked())?
        };
        config.tip_receiver = *self.accounts.payer.key();
        config.block_builder = *self.accounts.payer.key();

        let mut bumps = InitBumps {
            config: config_bump,
            ..Default::default()
        };

        bumps.tip_payment_account_0 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_0,
            self.program_id,
            self.accounts.tip_payment_account_0,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        bumps.tip_payment_account_1 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_1,
            self.program_id,
            self.accounts.tip_payment_account_1,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        bumps.tip_payment_account_2 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_2,
            self.program_id,
            self.accounts.tip_payment_account_2,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        bumps.tip_payment_account_3 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_3,
            self.program_id,
            self.accounts.tip_payment_account_3,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        bumps.tip_payment_account_4 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_4,
            self.program_id,
            self.accounts.tip_payment_account_4,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        bumps.tip_payment_account_5 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_5,
            self.program_id,
            self.accounts.tip_payment_account_5,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        bumps.tip_payment_account_6 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_6,
            self.program_id,
            self.accounts.tip_payment_account_6,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        bumps.tip_payment_account_7 = TipPaymentAccount::initialize(
            TIP_ACCOUNT_SEED_7,
            self.program_id,
            self.accounts.tip_payment_account_7,
            self.accounts.payer,
            self.accounts.system_program,
            &rent,
        )?;

        config.bumps = bumps;
        config.block_builder_commission_pct = 0;

        Ok(())
    }
}
