use pinocchio::error::ProgramError;

use crate::{require_len, traits::InstructionData};

/// Instruction data for SetArbiter
///
/// # Layout
/// * `extensions_bump` (u8) - Bump for extensions PDA
pub struct SetArbiterData {
    pub extensions_bump: u8,
}

impl<'a> TryFrom<&'a [u8]> for SetArbiterData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        Ok(Self { extensions_bump: data[0] })
    }
}

impl<'a> InstructionData<'a> for SetArbiterData {
    const LEN: usize = 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_arbiter_data_try_from_valid() {
        let data = [255u8; 1];

        let result = SetArbiterData::try_from(&data[..]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions_bump, 255);
    }

    #[test]
    fn test_set_arbiter_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = SetArbiterData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
