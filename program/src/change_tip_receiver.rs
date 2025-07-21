use jito_tip_payment_core::{config::Config, load_mut_unchecked};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
};

use crate::handle_payments;

/// Validator should invoke this instruction before executing any transactions that contain tips.
/// Validator should also ensure it calls it if there's a fork detected.
pub fn process_change_tip_receiver(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let [config, old_tip_receiver, new_tip_receiver, block_builder, tip_payment_account_0, tip_payment_account_1, tip_payment_account_2, tip_payment_account_3, tip_payment_account_4, tip_payment_account_5, tip_payment_account_6, tip_payment_account_7, _signer] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

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
        tip_payment_account_0,
        tip_payment_account_1,
        tip_payment_account_2,
        tip_payment_account_3,
        tip_payment_account_4,
        tip_payment_account_5,
        tip_payment_account_6,
        tip_payment_account_7,
    ];

    let config = unsafe { load_mut_unchecked::<Config>(config.borrow_mut_data_unchecked())? };

    unsafe {
        handle_payments(
            &rent,
            tip_accounts,
            old_tip_receiver,
            block_builder,
            config.block_builder_commission_pct,
        )?;
    }

    config.tip_receiver = *new_tip_receiver.key();

    Ok(())
}
