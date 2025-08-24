use pinocchio::{program_error::ProgramError, pubkey::Pubkey};
use shank::ShankInstruction;

#[derive(Clone, Debug, PartialEq, Eq, ShankInstruction)]
#[repr(C)]
pub enum JitoTipDistributionInstruction {
    /// Initialize
    #[account(0, writable, name = "config")]
    #[account(1, name = "system_program")]
    #[account(2, writable, signer, name = "initializer")]
    Initialize {
        authority: Pubkey,
        expired_funds_account: Pubkey,
        num_epochs_valid: u64,
        max_validator_commission_bps: u16,
        bump: u8,
    },

    /// Initialize Tip Distribution Account
    #[account(0, writable, name = "config")]
    #[account(1, writable, name = "tip_distribution_account")]
    #[account(2, name = "validator_vote_account")]
    #[account(3, writable, signer, name = "signer")]
    #[account(4, name = "system_program")]
    InitializeTipDistributionAccount {
        merkle_root_upload_authority: Pubkey,
        validator_commission_bps: u16,
        bump: u8,
    },

    /// Update config
    #[account(0, writable, name = "config")]
    #[account(1, signer, name = "authority")]
    UpdateConfig {
        authority: Pubkey,
        expired_funds_account: Pubkey,
        num_epochs_valid: u64,
        max_validator_commission_bps: u16,
    },

    /// Upload merkle root
    #[account(0, name = "config")]
    #[account(1, writable, name = "tip_distribution_account")]
    #[account(2, writable, signer, name = "merkle_root_upload_authority")]
    UploadMerkleRoot {
        root: [u8; 32],
        max_total_claim: u32,
        max_num_nodes: u64,
    },

    /// Close claim status
    #[account(0, name = "config")]
    #[account(1, writable, name = "claim_status")]
    #[account(2, signer, name = "claim_status_payer")]
    CloseClaimStatus,

    /// Close claim status
    #[account(0, name = "config")]
    #[account(1, writable, name = "expired_funds_account")]
    #[account(2, writable, name = "tip_distribution_account")]
    #[account(3, writable, name = "validator_vote_account")]
    #[account(4, writable, signer, name = "signer")]
    CloseTipDistributionAccount,

    /// Claim
    #[account(0, name = "config")]
    #[account(1, writable, name = "tip_distribution_account")]
    #[account(2, writable, signer, name = "merkle_root_upload_authority")]
    #[account(3, writable, name = "claim_status")]
    #[account(4, writable, name = "claimant")]
    #[account(5, writable, signer, name = "payer")]
    #[account(6, name = "system_program")]
    Claim {
        bump: u8,
        amount: u64,
        proof: Vec<[u8; 32]>,
    },

    /// Initialize merkle root upload config
    #[account(0, name = "config")]
    #[account(1, writable, name = "merkle_root_upload_config")]
    #[account(2, signer, name = "authority")]
    #[account(3, writable, signer, name = "payer")]
    #[account(4, name = "system_program")]
    InitializeMerkleRootUploadConfig {
        authority: Pubkey,
        original_authority: Pubkey,
    },

    /// Initialize merkle root upload config
    #[account(0, name = "config")]
    #[account(1, writable, name = "merkle_root_upload_config")]
    #[account(2, signer, name = "authority")]
    #[account(3, name = "system_program")]
    UpdateMerkleRootUploadConfig {
        authority: Pubkey,
        original_authority: Pubkey,
    },

    /// Initialize merkle root upload config
    #[account(0, writable, name = "tip_distribution_account")]
    #[account(1, writable, name = "merkle_root_upload_config")]
    MigrateTdaMerkleRootUploadAuthority,
}

impl JitoTipDistributionInstruction {
    pub fn try_from_slice(instruction_data: &[u8]) -> Result<Self, ProgramError> {
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
            [175, 175, 109, 31, 13, 152, 155, 237] => {
                let mut authority: Pubkey = [0; 32];
                authority.copy_from_slice(&remaining[0..32]);

                let mut expired_funds_account: Pubkey = [0; 32];
                expired_funds_account.copy_from_slice(&remaining[32..64]);

                let mut num_epochs_valid = [0; 8];
                num_epochs_valid.copy_from_slice(&remaining[64..72]);

                let mut max_validator_commission_bps = [0; 2];
                max_validator_commission_bps.copy_from_slice(&remaining[72..74]);

                Ok(Self::Initialize {
                    authority,
                    expired_funds_account,
                    num_epochs_valid: u64::from_be_bytes(num_epochs_valid),
                    max_validator_commission_bps: u16::from_be_bytes(max_validator_commission_bps),
                    bump: remaining[74],
                })
            }

            // Initialize tip distribution account
            [120, 191, 25, 182, 111, 49, 179, 55] => {
                let mut merkle_root_upload_authority: Pubkey = [0; 32];
                merkle_root_upload_authority.copy_from_slice(&remaining[0..32]);

                let mut validator_commission_bps = [0; 2];
                validator_commission_bps.copy_from_slice(&remaining[32..34]);

                Ok(Self::InitializeTipDistributionAccount {
                    merkle_root_upload_authority,
                    validator_commission_bps: u16::from_be_bytes(validator_commission_bps),
                    bump: remaining[34],
                })
            }

            // Update config
            [29, 158, 252, 191, 10, 83, 219, 99] => {
                let mut authority: Pubkey = [0; 32];
                authority.copy_from_slice(&remaining[0..32]);

                let mut expired_funds_account: Pubkey = [0; 32];
                expired_funds_account.copy_from_slice(&remaining[32..64]);

                let mut num_epochs_valid = [0; 8];
                num_epochs_valid.copy_from_slice(&remaining[64..72]);

                let mut max_validator_commission_bps = [0; 2];
                max_validator_commission_bps.copy_from_slice(&remaining[72..74]);

                Ok(Self::UpdateConfig {
                    authority,
                    expired_funds_account,
                    num_epochs_valid: u64::from_be_bytes(num_epochs_valid),
                    max_validator_commission_bps: u16::from_be_bytes(max_validator_commission_bps),
                })
            }

            // Upload merkle root
            [70, 3, 110, 29, 199, 190, 205, 176] => {
                let mut root = [0; 32];
                root.copy_from_slice(&remaining[0..32]);

                let mut max_total_claim = [0; 4];
                max_total_claim.copy_from_slice(&remaining[32..36]);

                let mut max_num_nodes = [0; 8];
                max_num_nodes.copy_from_slice(&remaining[36..44]);

                Ok(Self::UploadMerkleRoot {
                    root,
                    max_total_claim: u32::from_be_bytes(max_total_claim),
                    max_num_nodes: u64::from_be_bytes(max_num_nodes),
                })
            }

            // Close claim status
            [163, 214, 191, 165, 245, 188, 17, 185] => Ok(Self::CloseClaimStatus),

            // Close tip distribution account
            [47, 136, 208, 190, 125, 243, 74, 227] => Ok(Self::CloseTipDistributionAccount),

            // Claim
            [62, 198, 214, 193, 213, 159, 108, 210] => {
                let mut amount = [0; 8];
                amount.copy_from_slice(&remaining[1..9]);

                let mut proof_len_bytes = [0; 4];
                proof_len_bytes.copy_from_slice(&remaining[9..13]);
                let proof_len = u32::from_le_bytes(proof_len_bytes) as usize;

                // Check if we have enough bytes for all proof elements
                let expected_bytes = 13usize
                    .checked_add(
                        proof_len
                            .checked_mul(32)
                            .ok_or(ProgramError::InvalidInstructionData)?,
                    )
                    .ok_or(ProgramError::InvalidInstructionData)?;
                if remaining.len() < expected_bytes {
                    return Err(ProgramError::InvalidInstructionData);
                }

                // Read the proof vector
                let mut proof = Vec::with_capacity(proof_len);
                for i in 0..proof_len {
                    let start_idx = 13usize
                        .checked_add(
                            i.checked_mul(32)
                                .ok_or(ProgramError::InvalidInstructionData)?,
                        )
                        .ok_or(ProgramError::InvalidInstructionData)?;
                    let end_idx = start_idx
                        .checked_add(32usize)
                        .ok_or(ProgramError::InvalidInstructionData)?;

                    let mut hash = [0u8; 32];
                    hash.copy_from_slice(&remaining[start_idx..end_idx]);
                    proof.push(hash);
                }

                Ok(Self::Claim {
                    bump: remaining[0],
                    amount: u64::from_be_bytes(amount),
                    proof,
                })
            }

            // Initialize merkle root upload config
            [232, 87, 72, 14, 89, 40, 40, 27] => {
                let mut authority: Pubkey = [0; 32];
                authority.copy_from_slice(&remaining[0..32]);

                let mut original_authority: Pubkey = [0; 32];
                original_authority.copy_from_slice(&remaining[0..32]);

                Ok(Self::InitializeMerkleRootUploadConfig {
                    authority,
                    original_authority,
                })
            }

            // Update merkle root upload config
            [128, 227, 159, 139, 176, 128, 118, 2] => {
                let mut authority: Pubkey = [0; 32];
                authority.copy_from_slice(&remaining[0..32]);

                let mut original_authority: Pubkey = [0; 32];
                original_authority.copy_from_slice(&remaining[0..32]);

                Ok(Self::UpdateMerkleRootUploadConfig {
                    authority,
                    original_authority,
                })
            }

            // Migrate tda merkle root upload authority
            [13, 226, 163, 144, 56, 202, 214, 23] => Ok(Self::MigrateTdaMerkleRootUploadAuthority),

            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
