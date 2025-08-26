use jito_tip_core::loader::{load_signer, load_system_account, load_system_program};
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
};
use pinocchio_system::instructions::CreateAccount;

use crate::{
    CONFIG_ACCOUNT_SEED, TIP_ACCOUNT_SEED_0, TIP_ACCOUNT_SEED_1, TIP_ACCOUNT_SEED_2,
    TIP_ACCOUNT_SEED_3, TIP_ACCOUNT_SEED_4, TIP_ACCOUNT_SEED_5, TIP_ACCOUNT_SEED_6,
    TIP_ACCOUNT_SEED_7,
};

pub fn process_initialize(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
) -> Result<(), ProgramError> {
    let [config, tip_payment_account_0, tip_payment_account_1, tip_payment_account_2, tip_payment_account_3, tip_payment_account_4, tip_payment_account_5, tip_payment_account_6, tip_payment_account_7, system_program, payer] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    unsafe {
        load_system_account(config, true)?;
        load_system_account(tip_payment_account_0, true)?;
        load_system_account(tip_payment_account_1, true)?;
        load_system_account(tip_payment_account_2, true)?;
        load_system_account(tip_payment_account_3, true)?;
        load_system_account(tip_payment_account_4, true)?;
        load_system_account(tip_payment_account_5, true)?;
        load_system_account(tip_payment_account_6, true)?;
        load_system_account(tip_payment_account_7, true)?;
    }
    load_system_program(system_program)?;
    load_signer(payer, true)?;

    let rent = Rent::get()?;

    let space = Config::LEN;
    let required_lamports = rent.minimum_balance(space);

    let (_config_pubkey, config_bump) = find_program_address(&[CONFIG_ACCOUNT_SEED], program_id);

    let bindings = [config_bump];
    let seeds = [Seed::from(CONFIG_ACCOUNT_SEED), Seed::from(&bindings)];
    let signers = [Signer::from(&seeds)];
    CreateAccount {
        from: payer,
        to: config,
        lamports: required_lamports,
        space: space as u64,
        owner: program_id,
    }
    .invoke_signed(&signers)?;

    let config = unsafe { load_mut_unchecked::<Config>(config.borrow_mut_data_unchecked())? };
    config.tip_receiver = *payer.key();
    config.block_builder = *payer.key();

    let mut bumps = InitBumps {
        config: config_bump,
        ..Default::default()
    };

    bumps.tip_payment_account_0 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_0,
        program_id,
        tip_payment_account_0,
        payer,
        system_program,
        &rent,
    )?;

    bumps.tip_payment_account_1 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_1,
        program_id,
        tip_payment_account_1,
        payer,
        system_program,
        &rent,
    )?;

    bumps.tip_payment_account_2 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_2,
        program_id,
        tip_payment_account_2,
        payer,
        system_program,
        &rent,
    )?;

    bumps.tip_payment_account_3 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_3,
        program_id,
        tip_payment_account_3,
        payer,
        system_program,
        &rent,
    )?;

    bumps.tip_payment_account_4 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_4,
        program_id,
        tip_payment_account_4,
        payer,
        system_program,
        &rent,
    )?;

    bumps.tip_payment_account_5 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_5,
        program_id,
        tip_payment_account_5,
        payer,
        system_program,
        &rent,
    )?;

    bumps.tip_payment_account_6 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_6,
        program_id,
        tip_payment_account_6,
        payer,
        system_program,
        &rent,
    )?;

    bumps.tip_payment_account_7 = TipPaymentAccount::initialize(
        TIP_ACCOUNT_SEED_7,
        program_id,
        tip_payment_account_7,
        payer,
        system_program,
        &rent,
    )?;

    config.bumps = bumps;
    config.block_builder_commission_pct = 0;

    Ok(())
}
