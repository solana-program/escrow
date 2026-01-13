use pinocchio::error::ProgramError;

use crate::{require_len, traits::InstructionData};

/// Instruction data for AllowMint
///
/// # Layout
/// * `bump` (u8) - Bump for the allowed_mint PDA
pub struct AllowMintData {
    pub bump: u8,
}

impl<'a> TryFrom<&'a [u8]> for AllowMintData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        Ok(Self { bump: data[0] })
    }
}

impl<'a> InstructionData<'a> for AllowMintData {
    const LEN: usize = 1; // bump
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_mint_data_try_from_valid() {
        let data = [255u8];
        let result = AllowMintData::try_from(&data[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().bump, 255);

        let data = [0u8, 1, 2, 3];
        let result = AllowMintData::try_from(&data[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().bump, 0);
    }

    #[test]
    fn test_allow_mint_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = AllowMintData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
