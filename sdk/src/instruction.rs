#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum JitoTipPaymentInstruction {
    /// Initialize
    Initialize,

    /// Change tip receiver
    ChangeTipReceiver,

    /// Change block builder
    ChangeBlockBuilder,
}
