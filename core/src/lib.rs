use pinocchio::program_error::ProgramError;

pub mod config;
pub mod fees;
pub mod init_bumps;
pub mod tip_payment_account;

/// Marker trait for types that can be cast from a raw pointer.
///
/// # Safety
///
/// It is up to the type implementing this trait to guarantee that the cast is
/// safe, i.e., the fields of the type are well aligned and there are no padding
/// bytes.
pub unsafe trait Transmutable {
    /// The length of the type.
    ///
    /// This must be equal to the size of each individual field in the type.
    const LEN: usize;
}

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
