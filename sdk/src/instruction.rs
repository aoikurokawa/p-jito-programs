use shank::ShankInstruction;

#[repr(C)]
#[derive(Clone, Debug, PartialEq, ShankInstruction)]
pub enum JitoTipPaymentInstruction {
    /// Initialize
    Initialize,

    /// Change tip receiver
    ChangeTipReceiver,

    /// Change block builder
    ChangeBlockBuilder,
}
