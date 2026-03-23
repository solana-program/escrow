use pinocchio::error::ProgramError;

use crate::{require_len, traits::InstructionData};

/// Instruction data for RemoveExtension
///
/// # Layout
/// * `extensions_bump` (u8) - Bump for extensions PDA
/// * `extension_type` (u16) - Escrow extension type discriminator to remove
pub struct RemoveExtensionData {
    pub extensions_bump: u8,
    pub extension_type: u16,
}

impl<'a> TryFrom<&'a [u8]> for RemoveExtensionData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        Ok(Self { extensions_bump: data[0], extension_type: u16::from_le_bytes([data[1], data[2]]) })
    }
}

impl<'a> InstructionData<'a> for RemoveExtensionData {
    const LEN: usize = 1 + 2;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_extension_data_try_from_valid() {
        let mut data = [0u8; RemoveExtensionData::LEN];
        data[0] = 255; // extensions_bump
        data[1..3].copy_from_slice(&2u16.to_le_bytes()); // extension_type

        let result = RemoveExtensionData::try_from(&data[..]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions_bump, 255);
        assert_eq!(parsed.extension_type, 2);
    }

    #[test]
    fn test_remove_extension_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = RemoveExtensionData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
