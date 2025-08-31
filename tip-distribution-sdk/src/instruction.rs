use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

// const MAX_PROOF_SIZE: usize = 32;

#[derive(Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub enum JitoTipDistributionInstruction {
    /// Initialize
    Initialize {
        authority: Pubkey,
        expired_funds_account: Pubkey,
        num_epochs_valid: u64,
        max_validator_commission_bps: u16,
        bump: u8,
    },

    /// Initialize Tip Distribution Account
    InitializeTipDistributionAccount {
        merkle_root_upload_authority: Pubkey,
        validator_commission_bps: u16,
        bump: u8,
    },

    /// Update config
    UpdateConfig {
        authority: Pubkey,
        expired_funds_account: Pubkey,
        num_epochs_valid: u64,
        max_validator_commission_bps: u16,
    },

    /// Upload merkle root
    UploadMerkleRoot {
        root: [u8; 32],
        max_total_claim: u64,
        max_num_nodes: u64,
    },

    /// Close claim status
    CloseClaimStatus,

    /// Close claim status
    CloseTipDistributionAccount,

    // Claim
    // Claim {
    //     bump: u8,
    //     amount: u64,
    //     proof: [[u8; 32]; MAX_PROOF_SIZE],
    // },
    /// Initialize merkle root upload config
    InitializeMerkleRootUploadConfig {
        authority: Pubkey,
        original_authority: Pubkey,
    },

    /// Initialize merkle root upload config
    UpdateMerkleRootUploadConfig {
        authority: Pubkey,
        original_authority: Pubkey,
    },

    /// Initialize merkle root upload config
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
                    num_epochs_valid: u64::from_le_bytes(num_epochs_valid),
                    max_validator_commission_bps: u16::from_le_bytes(max_validator_commission_bps),
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
                    validator_commission_bps: u16::from_le_bytes(validator_commission_bps),
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
                    max_validator_commission_bps: u16::from_le_bytes(max_validator_commission_bps),
                })
            }

            // Upload merkle root
            [70, 3, 110, 29, 199, 190, 205, 176] => {
                let mut root = [0; 32];
                root.copy_from_slice(&remaining[0..32]);

                let mut max_total_claim = [0; 8];
                max_total_claim.copy_from_slice(&remaining[32..40]);

                let mut max_num_nodes = [0; 8];
                max_num_nodes.copy_from_slice(&remaining[40..48]);

                Ok(Self::UploadMerkleRoot {
                    root,
                    max_total_claim: u64::from_le_bytes(max_total_claim),
                    max_num_nodes: u64::from_le_bytes(max_num_nodes),
                })
            }

            // Close claim status
            [163, 214, 191, 165, 245, 188, 17, 185] => Ok(Self::CloseClaimStatus),

            // Close tip distribution account
            [47, 136, 208, 190, 125, 243, 74, 227] => Ok(Self::CloseTipDistributionAccount),

            // Claim
            // [62, 198, 214, 193, 213, 159, 108, 210] => {
            //     let bump = remaining[0];

            //     let mut amount = [0; 8];
            //     amount.copy_from_slice(&remaining[1..9]);

            //     let mut proof_len_bytes = [0; 4];
            //     proof_len_bytes.copy_from_slice(&remaining[9..13]);
            //     let proof_len = u32::from_le_bytes(proof_len_bytes) as usize;

            //     // Check if we have enough bytes for all proof elements
            //     let expected_bytes = 13 + (proof_len * 32);
            //     if remaining.len() < expected_bytes {
            //         return Err(ProgramError::InvalidInstructionData);
            //     }

            //     let mut proof = [[0u8; 32]; MAX_PROOF_SIZE];
            //     for i in 0..proof_len {
            //         let start_idx = 13 + (i * 32); // Start after discriminator + bump + amount + proof_len
            //         let end_idx = start_idx + 32;
            //         proof[i].copy_from_slice(&remaining[start_idx..end_idx]);
            //     }

            //     Ok(Self::Claim {
            //         bump,
            //         amount: u64::from_le_bytes(amount),
            //         proof,
            //     })
            // }

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
