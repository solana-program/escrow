use pinocchio::error::ProgramError;

use crate::traits::InstructionData;

/// Instruction data for BlockMint
///
/// No additional data needed - all information is from accounts
pub struct BlockMintData;

impl<'a> TryFrom<&'a [u8]> for BlockMintData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

impl<'a> InstructionData<'a> for BlockMintData {
    const LEN: usize = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_mint_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = BlockMintData::try_from(&data[..]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_block_mint_data_try_from_with_extra_bytes() {
        let data = [1u8, 2, 3];
        let result = BlockMintData::try_from(&data[..]);
        assert!(result.is_ok());
    }
}
