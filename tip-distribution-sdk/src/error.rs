use pinocchio::program_error::ProgramError;
use thiserror::Error;

#[derive(Clone, Debug, Eq, PartialEq, Error)]
pub enum TipDistributionError {
    #[error("Account failed validation.")]
    AccountValidationFailure,

    #[error("Encountered an arithmetic under/overflow error.")]
    ArithmeticError,

    #[error("The maximum number of funds to be claimed has been exceeded.")]
    ExceedsMaxClaim,

    #[error("The maximum number of claims has been exceeded.")]
    ExceedsMaxNumNodes,

    #[error("The given TipDistributionAccount has expired.")]
    ExpiredTipDistributionAccount,

    #[error("The funds for the given index and TipDistributionAccount have already been claimed.")]
    FundsAlreadyClaimed,

    #[error("Supplied invalid parameters.")]
    InvalidParameters,

    #[error("The given proof is invalid.")]
    InvalidProof,

    #[error("Failed to deserialize the supplied vote account data.")]
    InvalidVoteAccountData,

    #[error("Validator's commission basis points must be less than or equal to the Config account's max_validator_commission_bps.")]
    MaxValidatorCommissionFeeBpsExceeded,

    #[error("The given TipDistributionAccount is not ready to be closed.")]
    PrematureCloseTipDistributionAccount,

    #[error("The given ClaimStatus account is not ready to be closed.")]
    PrematureCloseClaimStatus,

    #[error("Must wait till at least one epoch after the tip distribution account was created to upload the merkle root.")]
    PrematureMerkleRootUpload,

    #[error("No merkle root has been uploaded to the given TipDistributionAccount.")]
    RootNotUploaded,

    #[error("Unauthorized signer.")]
    Unauthorized,

    #[error("TDA not valid for migration.")]
    InvalidTdaForMigration,
}

impl From<TipDistributionError> for ProgramError {
    fn from(value: TipDistributionError) -> Self {
        Self::Custom(value as u32)
    }
}
