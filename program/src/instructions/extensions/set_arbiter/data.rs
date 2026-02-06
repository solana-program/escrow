use pinocchio::{error::ProgramError, Address};

use crate::{require_len, traits::InstructionData};

/// Instruction data for SetArbiter
///
/// # Layout
/// * `extensions_bump` (u8) - Bump for extensions PDA
/// * `arbiter` (Address) - Arbiter address that must sign withdrawals
pub struct SetArbiterData {
    pub extensions_bump: u8,
    pub arbiter: Address,
}

impl<'a> TryFrom<&'a [u8]> for SetArbiterData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        Ok(Self { extensions_bump: data[0], arbiter: Address::new_from_array(data[1..33].try_into().unwrap()) })
    }
}

impl<'a> InstructionData<'a> for SetArbiterData {
    const LEN: usize = 1 + 32; // 33 bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_arbiter_data_try_from_valid() {
        let mut data = [0u8; 33];
        data[0] = 255; // extensions_bump
        data[1..33].copy_from_slice(&[1u8; 32]); // arbiter

        let result = SetArbiterData::try_from(&data[..]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions_bump, 255);
        assert_eq!(parsed.arbiter, Address::new_from_array([1u8; 32]));
    }

    #[test]
    fn test_set_arbiter_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = SetArbiterData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
