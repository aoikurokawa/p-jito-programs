use jito_tip_payment_core::{config::Config, load_mut_unchecked};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

use crate::handle_payments;

pub struct ChangeTipReceiverAccounts<'a> {
    /// Config
    pub config: &'a AccountInfo,

    /// Old tip receiver
    pub old_tip_receiver: &'a AccountInfo,

    /// New tip receiver
    pub new_tip_receiver: &'a AccountInfo,

    /// Block builder
    pub block_builder: &'a AccountInfo,

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

    /// Signer
    pub signer: &'a AccountInfo,
}

impl<'a> TryFrom<&'a [AccountInfo]> for ChangeTipReceiverAccounts<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let [config, old_tip_receiver, new_tip_receiver, block_builder, tip_payment_account_0, tip_payment_account_1, tip_payment_account_2, tip_payment_account_3, tip_payment_account_4, tip_payment_account_5, tip_payment_account_6, tip_payment_account_7, signer] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // Basic Accounts Checks
        // if !owner.is_signer() {
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        // if !vault.is_owned_by(&pinocchio_system::ID) {
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        // if vault.lamports().eq(&0) {
        //     return Err(ProgramError::InvalidAccountData);
        // }

        // let (vault_key, bump) = find_program_address(&[b"vault", owner.key().as_ref()], &crate::ID);
        // if &vault_key != vault.key() {
        //     return Err(ProgramError::InvalidAccountOwner);
        // }

        Ok(Self {
            config,
            old_tip_receiver,
            new_tip_receiver,
            block_builder,
            tip_payment_account_0,
            tip_payment_account_1,
            tip_payment_account_2,
            tip_payment_account_3,
            tip_payment_account_4,
            tip_payment_account_5,
            tip_payment_account_6,
            tip_payment_account_7,
            signer,
        })
    }
}

pub struct ChangeTipReceiver<'a> {
    pub accounts: ChangeTipReceiverAccounts<'a>,
}

impl<'a> TryFrom<&'a [AccountInfo]> for ChangeTipReceiver<'a> {
    type Error = ProgramError;

    fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
        let accounts = ChangeTipReceiverAccounts::try_from(accounts)?;

        Ok(Self { accounts })
    }
}

impl<'a> ChangeTipReceiver<'a> {
    pub const DISCRIMINATOR: &'a u8 = &1;

    pub fn process(&mut self) -> ProgramResult {
        let rent = Rent::get()?;

        // Create PDA signer seeds
        // let seeds = [
        //     Seed::from(b"vault"),
        //     Seed::from(self.accounts.owner.key().as_ref()),
        //     Seed::from(&self.accounts.bumps),
        // ];
        // let signers = [Signer::from(&seeds)];

        // Transfer all lamports from vault to owner
        // Transfer {
        //     from: self.accounts.vault,
        //     to: self.accounts.owner,
        //     lamports: self.accounts.vault.lamports(),
        // }
        // .invoke_signed(&signers)?;

        let tip_accounts = &[
            self.accounts.tip_payment_account_0,
            self.accounts.tip_payment_account_1,
            self.accounts.tip_payment_account_2,
            self.accounts.tip_payment_account_3,
            self.accounts.tip_payment_account_4,
            self.accounts.tip_payment_account_5,
            self.accounts.tip_payment_account_6,
            self.accounts.tip_payment_account_7,
        ];

        let config = unsafe {
            load_mut_unchecked::<Config>(self.accounts.config.borrow_mut_data_unchecked())?
        };

        unsafe {
            handle_payments(
                &rent,
                tip_accounts,
                &self.accounts.old_tip_receiver,
                &self.accounts.block_builder,
                config.block_builder_commission_pct,
            )?;
        }

        config.tip_receiver = *self.accounts.new_tip_receiver.key();

        Ok(())
    }
}
