use alloc::vec::Vec;
use pinocchio::error::ProgramError;

/// Trait for consistent serialization/deserialization across all extension types.
pub trait ExtensionData: Sized {
    fn from_bytes(data: &[u8]) -> Result<Self, ProgramError>;
    fn to_bytes(&self) -> Vec<u8>;
}
