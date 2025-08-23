use pinocchio::pubkey::Pubkey;
use shank::ShankAccount;

#[derive(Debug, Default, ShankAccount)]
#[repr(C)]
pub struct ClaimStatus {
    /// If true, the tokens have been claimed.
    pub is_claimed: bool,

    /// Authority that claimed the tokens. Allows for delegated rewards claiming.
    pub claimant: Pubkey,

    /// The payer who created the claim.
    pub claim_status_payer: Pubkey,

    /// When the funds were claimed.
    pub slot_claimed_at: u64,

    /// Amount of funds claimed.
    pub amount: u64,

    /// The epoch (upto and including) that tip funds can be claimed.
    /// Copied since TDA can be closed, need to track to avoid making multiple claims
    pub expires_at: u64,

    /// The bump used to generate this account
    pub bump: u8,
}

impl ClaimStatus {
    pub fn seeds() -> Vec<Vec<u8>> {
        vec![b"CLAIM_STATUS".to_vec()]
    }
}
