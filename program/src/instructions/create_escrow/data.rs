use pinocchio::error::ProgramError;

use crate::{require_len, traits::InstructionData};

/// Instruction data for CreateEscrow
///
/// # Layout
/// * `bump` (u8) - Bump for the escrow PDA
pub struct CreateEscrowData {
    pub bump: u8,
}

impl<'a> TryFrom<&'a [u8]> for CreateEscrowData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        Ok(Self { bump: data[0] })
    }
}

impl<'a> InstructionData<'a> for CreateEscrowData {
    const LEN: usize = 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_escrow_data_try_from_valid() {
        let data = [255u8];
        let result = CreateEscrowData::try_from(&data[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().bump, 255);

        let data = [0u8, 1, 2, 3];
        let result = CreateEscrowData::try_from(&data[..]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().bump, 0);
    }

    #[test]
    fn test_create_escrow_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = CreateEscrowData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
