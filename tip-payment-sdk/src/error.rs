use pinocchio::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum TipPaymentError {
    #[error("ArithmeticError")]
    ArithmeticError,

    #[error("InvalidFee")]
    InvalidFee,

    #[error("InvalidTipReceiver")]
    InvalidTipReceiver,

    #[error("InvalidBlockBuilder")]
    InvalidBlockBuilder,
}

impl From<TipPaymentError> for ProgramError {
    fn from(value: TipPaymentError) -> Self {
        Self::Custom(value as u32)
    }
}
