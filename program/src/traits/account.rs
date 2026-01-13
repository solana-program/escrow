use alloc::vec::Vec;
use pinocchio::error::ProgramError;

use crate::{require_len, validate_discriminator};

/// Discriminator for account types
pub trait Discriminator {
    const DISCRIMINATOR: u8;
}

/// Version marker for account types
pub trait Versioned {
    const VERSION: u8;
}

/// Account size constants
pub trait AccountSize: Discriminator + Versioned + Sized {
    /// Size of the account data (excluding discriminator and version)
    const DATA_LEN: usize;

    /// Total size including discriminator and version
    const LEN: usize = 1 + 1 + Self::DATA_LEN;
}

/// Zero-copy account deserialization
pub trait AccountDeserialize: AccountSize {
    /// Zero-copy read from byte slice (validates discriminator, skips version)
    #[inline(always)]
    fn from_bytes(data: &[u8]) -> Result<&Self, ProgramError> {
        require_len!(data, Self::LEN);
        validate_discriminator!(data, Self::DISCRIMINATOR);

        // Skip discriminator (byte 0) and version (byte 1)
        unsafe { Self::from_bytes_unchecked(&data[2..]) }
    }

    /// Zero-copy read without discriminator validation
    ///
    /// # Safety
    /// Caller must ensure data is valid, properly sized, and aligned.
    /// Struct must be `#[repr(C)]` with no padding.
    #[inline(always)]
    unsafe fn from_bytes_unchecked(data: &[u8]) -> Result<&Self, ProgramError> {
        if data.len() < Self::DATA_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(&*(data.as_ptr() as *const Self))
    }

    /// Mutable zero-copy access
    #[inline(always)]
    fn from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        require_len!(data, Self::LEN);
        validate_discriminator!(data, Self::DISCRIMINATOR);

        // Skip discriminator (byte 0) and version (byte 1)
        unsafe { Self::from_bytes_mut_unchecked(&mut data[2..]) }
    }

    /// Mutable zero-copy access without validation
    ///
    /// # Safety
    /// Caller must ensure data is valid, properly sized, and aligned.
    /// Struct must be `#[repr(C)]` with no padding.
    #[inline(always)]
    unsafe fn from_bytes_mut_unchecked(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        if data.len() < Self::DATA_LEN {
            return Err(ProgramError::InvalidAccountData);
        }
        Ok(&mut *(data.as_mut_ptr() as *mut Self))
    }
}

/// Account discriminator values for this program
#[repr(u8)]
pub enum EscrowAccountDiscriminators {
    EscrowDiscriminator = 0,
    EscrowExtensionsDiscriminator = 1,
    ReceiptDiscriminator = 2,
    AllowedMintDiscriminator = 3,
}

/// Manual account deserialization (non-zero-copy)
///
/// Use this for accounts where zero-copy deserialization isn't possible
/// due to alignment constraints.
pub trait AccountParse: AccountSize {
    /// Parse account from bytes (validates discriminator, skips version)
    fn parse_from_bytes(data: &[u8]) -> Result<Self, ProgramError>;
}

/// Account serialization with discriminator and version prefix
pub trait AccountSerialize: Discriminator + Versioned {
    /// Serialize account data without discriminator/version
    fn to_bytes_inner(&self) -> Vec<u8>;

    /// Serialize with discriminator and version prefix
    #[inline(always)]
    fn to_bytes(&self) -> Vec<u8> {
        let inner = self.to_bytes_inner();
        let mut data = Vec::with_capacity(1 + 1 + inner.len());
        data.push(Self::DISCRIMINATOR);
        data.push(Self::VERSION);
        data.extend_from_slice(&inner);
        data
    }

    /// Write directly to a mutable slice
    #[inline(always)]
    fn write_to_slice(&self, dest: &mut [u8]) -> Result<(), ProgramError> {
        let bytes = self.to_bytes();
        if dest.len() < bytes.len() {
            return Err(ProgramError::AccountDataTooSmall);
        }
        dest[..bytes.len()].copy_from_slice(&bytes);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::state::Escrow;
    use alloc::vec;
    use pinocchio::Address;

    #[test]
    fn test_from_bytes_mut_modifies_original() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let escrow = Escrow::new(100, escrow_seed, admin);
        let mut bytes = escrow.to_bytes();

        {
            let escrow_mut = Escrow::from_bytes_mut(&mut bytes).unwrap();
            escrow_mut.bump = 200;
        }

        let deserialized = Escrow::from_bytes(&bytes).unwrap();
        assert_eq!(deserialized.bump, 200);
    }

    #[test]
    fn test_from_bytes_unchecked_skips_discriminator_and_version() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let escrow = Escrow::new(100, escrow_seed, admin);
        let bytes = escrow.to_bytes();

        // Skip discriminator (byte 0) and version (byte 1)
        let result = unsafe { Escrow::from_bytes_unchecked(&bytes[2..]) };
        assert!(result.is_ok());

        let deserialized = result.unwrap();
        assert_eq!(deserialized.bump, 100);
    }

    #[test]
    fn test_from_bytes_unchecked_too_short() {
        let data = [0u8; 10];
        let result = unsafe { Escrow::from_bytes_unchecked(&data) };
        assert_eq!(result, Err(ProgramError::InvalidAccountData));
    }

    #[test]
    fn test_to_bytes_roundtrip() {
        let escrow_seed = Address::new_from_array([42u8; 32]);
        let admin = Address::new_from_array([99u8; 32]);
        let escrow = Escrow::new(128, escrow_seed, admin);

        let bytes = escrow.to_bytes();
        let deserialized = Escrow::from_bytes(&bytes).unwrap();

        assert_eq!(deserialized.bump, escrow.bump);
        assert_eq!(deserialized.escrow_seed, escrow.escrow_seed);
        assert_eq!(deserialized.admin, escrow.admin);
    }

    #[test]
    fn test_write_to_slice_exact_size() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let escrow = Escrow::new(100, escrow_seed, admin);

        let mut dest = vec![0u8; Escrow::LEN];
        assert!(escrow.write_to_slice(&mut dest).is_ok());

        let deserialized = Escrow::from_bytes(&dest).unwrap();
        assert_eq!(deserialized.bump, 100);
    }

    #[test]
    fn test_version_auto_serialized() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let escrow = Escrow::new(100, escrow_seed, admin);

        let bytes = escrow.to_bytes();

        // Byte 0 = discriminator, Byte 1 = version
        assert_eq!(bytes[0], Escrow::DISCRIMINATOR);
        assert_eq!(bytes[1], Escrow::VERSION);
    }
}
