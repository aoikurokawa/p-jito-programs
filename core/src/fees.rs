use jito_tip_payment_sdk::error::TipPaymentError;

struct Fees {
    block_builder_fee_lamports: u64,
    tip_receiver_fee_lamports: u64,
}

impl Fees {
    fn calculate(
        total_tips: u64,
        block_builder_commission_pct: u64,
    ) -> Result<Self, TipPaymentError> {
        let block_builder_fee_lamports = total_tips
            .checked_mul(block_builder_commission_pct)
            .ok_or(TipPaymentError::ArithmeticError)?
            .checked_div(100)
            .ok_or(TipPaymentError::ArithmeticError)?;

        let tip_receiver_fee_lamports = total_tips
            .checked_sub(block_builder_fee_lamports)
            .ok_or(TipPaymentError::ArithmeticError)?;

        Ok(Self {
            block_builder_fee_lamports,
            tip_receiver_fee_lamports,
        })
    }
}
