use pinocchio::program_error::ProgramError;
use shank::ShankInstruction;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, Eq, ShankInstruction)]
pub enum JitoTipPaymentInstruction {
    /// Initialize
    #[account(0, writable, name = "config")]
    #[account(1, writable, name = "tip_payment_account_0")]
    #[account(2, writable, name = "tip_payment_account_1")]
    #[account(3, writable, name = "tip_payment_account_2")]
    #[account(4, writable, name = "tip_payment_account_3")]
    #[account(5, writable, name = "tip_payment_account_4")]
    #[account(6, writable, name = "tip_payment_account_5")]
    #[account(7, writable, name = "tip_payment_account_6")]
    #[account(8, writable, name = "tip_payment_account_7")]
    #[account(9, writable, signer, name = "payer")]
    Initialize,

    /// Change tip receiver
    #[account(0, writable, name = "config")]
    #[account(1, writable, name = "old_tip_receiver")]
    #[account(2, writable, name = "new_tip_receiver")]
    #[account(3, writable, name = "block_builder")]
    #[account(4, writable, name = "tip_payment_account_0")]
    #[account(5, writable, name = "tip_payment_account_1")]
    #[account(6, writable, name = "tip_payment_account_2")]
    #[account(7, writable, name = "tip_payment_account_3")]
    #[account(8, writable, name = "tip_payment_account_4")]
    #[account(9, writable, name = "tip_payment_account_5")]
    #[account(10, writable, name = "tip_payment_account_6")]
    #[account(11, writable, name = "tip_payment_account_7")]
    #[account(12, writable, signer, name = "signer")]
    ChangeTipReceiver,

    /// Change block builder
    #[account(0, writable, name = "config")]
    #[account(1, writable, name = "tip_receiver")]
    #[account(2, writable, name = "old_block_builder")]
    #[account(3, writable, name = "new_block_builder")]
    #[account(4, writable, name = "tip_payment_account_0")]
    #[account(5, writable, name = "tip_payment_account_1")]
    #[account(6, writable, name = "tip_payment_account_2")]
    #[account(7, writable, name = "tip_payment_account_3")]
    #[account(8, writable, name = "tip_payment_account_4")]
    #[account(9, writable, name = "tip_payment_account_5")]
    #[account(10, writable, name = "tip_payment_account_6")]
    #[account(11, writable, name = "tip_payment_account_7")]
    #[account(12, writable, signer, name = "signer")]
    ChangeBlockBuilder { block_builder_commission: u64 },
}

impl JitoTipPaymentInstruction {
    pub const fn try_from_slice(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let [discriminator0, discriminator1, discriminator2, discriminator3, discriminator4, discriminator5, discriminator6, discriminator7, remaining @ ..] =
            instruction_data
        else {
            return Err(ProgramError::InvalidInstructionData);
        };

        match [
            discriminator0,
            discriminator1,
            discriminator2,
            discriminator3,
            discriminator4,
            discriminator5,
            discriminator6,
            discriminator7,
        ] {
            // Initialize
            [175, 175, 109, 31, 13, 152, 155, 237] => Ok(Self::Initialize),

            // ChangeTipReceiver
            [69, 99, 22, 71, 11, 231, 86, 143] => Ok(Self::ChangeTipReceiver),

            // ChangeBlockBuilder
            [134, 80, 38, 137, 165, 21, 114, 123] => {
                let mut slice = [0, 0, 0, 0, 0, 0, 0, 0];
                slice.copy_from_slice(remaining);
                let block_builder_commission = u64::from_be_bytes(slice);

                Ok(Self::ChangeBlockBuilder {
                    block_builder_commission,
                })
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
