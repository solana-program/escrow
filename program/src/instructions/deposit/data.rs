use pinocchio::error::ProgramError;

use crate::{require_len, traits::InstructionData};

/// Instruction data for Deposit
///
/// # Layout
/// * `bump` (u8) - Bump for the deposit receipt PDA
/// * `amount` (u64) - Amount of tokens to deposit
pub struct DepositData {
    pub bump: u8,
    pub amount: u64,
}

impl<'a> TryFrom<&'a [u8]> for DepositData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(data: &'a [u8]) -> Result<Self, Self::Error> {
        require_len!(data, Self::LEN);

        let bump = data[0];
        let amount = u64::from_le_bytes(data[1..9].try_into().map_err(|_| ProgramError::InvalidInstructionData)?);

        Ok(Self { bump, amount })
    }
}

impl<'a> InstructionData<'a> for DepositData {
    const LEN: usize = 1 + 8; // bump + amount
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deposit_data_try_from_valid() {
        let mut data = [0u8; 9];
        data[0] = 255; // bump
        data[1..9].copy_from_slice(&1000u64.to_le_bytes()); // amount

        let result = DepositData::try_from(&data[..]);
        assert!(result.is_ok());
        let deposit_data = result.unwrap();
        assert_eq!(deposit_data.bump, 255);
        assert_eq!(deposit_data.amount, 1000);
    }

    #[test]
    fn test_deposit_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = DepositData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }

    #[test]
    fn test_deposit_data_try_from_too_short() {
        let data = [0u8; 5];
        let result = DepositData::try_from(&data[..]);
        assert!(matches!(result, Err(ProgramError::InvalidInstructionData)));
    }
}
