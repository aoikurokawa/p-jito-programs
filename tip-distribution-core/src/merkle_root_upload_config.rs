use pinocchio::pubkey::Pubkey;
use shank::ShankAccount;

#[derive(Debug, Default, ShankAccount)]
#[repr(C)]
pub struct MerkleRootUploadConfig {
    /// The authority that overrides the TipDistributionAccount merkle_root_upload_authority
    pub override_authority: Pubkey,

    /// The original merkle root upload authority that can be changed to the new overrided
    /// authority. E.g. Jito Labs authority GZctHpWXmsZC1YHACTGGcHhYxjdRqQvTpYkb9LMvxDib
    pub original_upload_authority: Pubkey,

    /// The bump used to generate this account
    pub bump: u8,
}
