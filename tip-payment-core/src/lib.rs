#![no_std]

use jito_tip_core::transmutable::Transmutable;
use pinocchio::program_error::ProgramError;

pub mod config;
pub mod fees;
pub mod init_bumps;
pub mod tip_payment_account;

// Trait to represent a type that can be initialized.
// pub trait Initializable {
//     /// Return `true` if the object is initialized.
//     fn is_initialized(&self) -> Result<bool, ProgramError>;
// }

/// Return a mutable `T` reference from the given bytes.
///
/// This function does not check if the data is initialized.
///
/// # Safety
///
/// The caller must ensure that `bytes` contains a valid representation of `T`.
#[inline(always)]
pub unsafe fn load_mut_unchecked<T: Transmutable>(
    bytes: &mut [u8],
) -> Result<&mut T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }

    Ok(&mut *(bytes.as_mut_ptr() as *mut T))
}

// Return a reference for an initialized `T` from the given bytes.
//
// # Safety
//
// The caller must ensure that `bytes` contains a valid representation of `T`.
// #[inline(always)]
// pub unsafe fn load<T: Initializable + Transmutable>(bytes: &[u8]) -> Result<&T, ProgramError> {
//     load_unchecked(bytes).and_then(|t: &T| {
//         // checks if the data is initialized
//         if t.is_initialized()? {
//             Ok(t)
//         } else {
//             Err(ProgramError::UninitializedAccount)
//         }
//     })
// }

/// Return a `T` reference from the given bytes.
///
/// This function does not check if the data is initialized.
///
/// # Safety
///
/// The caller must ensure that `bytes` contains a valid representation of `T`.
#[inline(always)]
pub const unsafe fn load_unchecked<T: Transmutable>(bytes: &[u8]) -> Result<&T, ProgramError> {
    if bytes.len() != T::LEN {
        return Err(ProgramError::InvalidAccountData);
    }
    Ok(&*(bytes.as_ptr() as *const T))
}
