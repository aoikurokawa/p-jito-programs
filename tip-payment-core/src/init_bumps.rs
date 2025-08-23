use shank::ShankType;

#[derive(Debug, Default, Clone, ShankType)]
#[repr(C)]
pub struct InitBumps {
    /// Config
    pub config: u8,

    /// Tip payment account 0
    pub tip_payment_account_0: u8,

    /// Tip payment account 1
    pub tip_payment_account_1: u8,

    /// Tip payment account 2
    pub tip_payment_account_2: u8,

    /// Tip payment account 3
    pub tip_payment_account_3: u8,

    /// Tip payment account 4
    pub tip_payment_account_4: u8,

    /// Tip payment account 5
    pub tip_payment_account_5: u8,

    /// Tip payment account 6
    pub tip_payment_account_6: u8,

    /// Tip payment account 7
    pub tip_payment_account_7: u8,
}

impl InitBumps {
    pub(crate) const SIZE: usize = 9;
}
