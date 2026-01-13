/// Validate the length of the data.
///
/// # Arguments
/// * `data` - The data to validate.
/// * `len` - The expected length.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[macro_export]
macro_rules! require_len {
    ($data:expr, $len:expr) => {
        if $data.len() < $len {
            return Err(ProgramError::InvalidInstructionData);
        }
    };
}

/// Validate the discriminator of the account.
///
/// # Arguments
/// * `data` - The account's data to validate.
/// * `discriminator` - The expected discriminator.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[macro_export]
macro_rules! validate_discriminator {
    ($data:expr, $discriminator:expr) => {
        if $data.is_empty() || $data[0] != $discriminator {
            return Err(ProgramError::InvalidAccountData);
        }
    };
}

/// Compile-time assertion that a struct has no implicit padding.
/// Use this for zero-copy structs to ensure memory layout matches serialized format.
///
/// # Example
/// ```ignore
/// assert_no_padding!(Escrow, 1 + 1 + 32 + 32);
/// ```
#[macro_export]
macro_rules! assert_no_padding {
    ($struct:ty, $expected_size:expr) => {
        const _: () = assert!(
            core::mem::size_of::<$struct>() == $expected_size,
            concat!(stringify!($struct), " struct size mismatch - check for padding")
        );
    };
}

/// Implement boilerplate `From` and `TryFrom` traits for instruction structs.
///
/// # Example
/// ```ignore
/// impl_instruction!(UpdateAdmin, UpdateAdminAccounts, UpdateAdminData);
/// ```
#[macro_export]
macro_rules! impl_instruction {
    ($name:ident, $accounts:ident, $data:ident) => {
        impl<'a> From<($accounts<'a>, $data)> for $name<'a> {
            #[inline(always)]
            fn from((accounts, data): ($accounts<'a>, $data)) -> Self {
                Self { accounts, data }
            }
        }

        impl<'a> TryFrom<(&'a [u8], &'a [pinocchio::account::AccountView])> for $name<'a> {
            type Error = pinocchio::error::ProgramError;

            #[inline(always)]
            fn try_from(
                (data, accounts): (&'a [u8], &'a [pinocchio::account::AccountView]),
            ) -> Result<Self, Self::Error> {
                Self::parse(data, accounts)
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use pinocchio::error::ProgramError;

    fn test_require_len(data: &[u8], len: usize) -> Result<(), ProgramError> {
        require_len!(data, len);
        Ok(())
    }

    fn test_validate_discriminator(data: &[u8], discriminator: u8) -> Result<(), ProgramError> {
        validate_discriminator!(data, discriminator);
        Ok(())
    }

    #[test]
    fn test_require_len_success() {
        let data = [1, 2, 3, 4, 5];
        assert!(test_require_len(&data, 5).is_ok());
        assert!(test_require_len(&data, 3).is_ok());
        assert!(test_require_len(&data, 1).is_ok());
    }

    #[test]
    fn test_require_len_too_short() {
        let data = [1, 2, 3];
        let result = test_require_len(&data, 5);
        assert_eq!(result, Err(ProgramError::InvalidInstructionData));
    }

    #[test]
    fn test_validate_discriminator_success() {
        let data = [42, 1, 2, 3];
        assert!(test_validate_discriminator(&data, 42).is_ok());
    }

    #[test]
    fn test_validate_discriminator_mismatch() {
        let data = [42, 1, 2, 3];
        let result = test_validate_discriminator(&data, 99);
        assert_eq!(result, Err(ProgramError::InvalidAccountData));
    }

    #[test]
    fn test_validate_discriminator_empty() {
        let data: [u8; 0] = [];
        let result = test_validate_discriminator(&data, 0);
        assert_eq!(result, Err(ProgramError::InvalidAccountData));
    }
}
