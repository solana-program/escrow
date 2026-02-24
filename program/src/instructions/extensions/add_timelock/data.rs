use pinocchio::error::ProgramError;

use crate::{require_len, traits::InstructionData};

/// Instruction data for AddTimelock
///
/// # Layout
/// * `extensions_bump` (u8) - Bump for extensions PDA
/// * `lock_duration` (u64) - Relative lock duration in seconds from deposit
pub struct AddTimelockData {
    pub extensions_bump: u8,
    pub lock_duration: u64,
}

impl<'a> TryFrom<&'a [u8]> for AddTimelockData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        let lock_duration = u64::from_le_bytes(data[1..9].try_into().unwrap());
        if lock_duration > i64::MAX as u64 {
            return Err(ProgramError::InvalidInstructionData);
        }

        Ok(Self { extensions_bump: data[0], lock_duration })
    }
}

impl<'a> InstructionData<'a> for AddTimelockData {
    const LEN: usize = 1 + 8; // 9 bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_timelock_data_try_from_valid() {
        let mut data = [0u8; 9];
        data[0] = 255; // extensions_bump
        data[1..9].copy_from_slice(&3600u64.to_le_bytes());

        let result = AddTimelockData::try_from(&data[..]);
        assert!(result.is_ok());
        let parsed = result.unwrap();
        assert_eq!(parsed.extensions_bump, 255);
        assert_eq!(parsed.lock_duration, 3600);
    }

    #[test]
    fn test_add_timelock_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = AddTimelockData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
