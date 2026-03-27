use pinocchio::error::ProgramError;

use crate::{require_len, traits::InstructionData};

/// Instruction data for UnblockTokenExtension
///
/// # Layout
/// * `extensions_bump` (u8) - Bump for extensions PDA
/// * `blocked_extension` (u16) - Token-2022 ExtensionType value to unblock
pub struct UnblockTokenExtensionData {
    pub extensions_bump: u8,
    pub blocked_extension: u16,
}

impl<'a> TryFrom<&'a [u8]> for UnblockTokenExtensionData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        Ok(Self { extensions_bump: data[0], blocked_extension: u16::from_le_bytes([data[1], data[2]]) })
    }
}

impl<'a> InstructionData<'a> for UnblockTokenExtensionData {
    const LEN: usize = 1 + 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unblock_token_extension_data_try_from_valid() {
        let mut data = [0u8; UnblockTokenExtensionData::LEN];
        data[0] = 255; // extensions_bump
        data[1..3].copy_from_slice(&42u16.to_le_bytes()); // blocked_extension

        let result = UnblockTokenExtensionData::try_from(&data[..]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions_bump, 255);
        assert_eq!(parsed.blocked_extension, 42);
    }

    #[test]
    fn test_unblock_token_extension_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = UnblockTokenExtensionData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
