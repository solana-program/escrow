use pinocchio::error::ProgramError;

use crate::traits::InstructionData;

/// Instruction data for Withdraw
///
/// All withdrawal information comes from the receipt account.
pub struct WithdrawData {}

impl<'a> TryFrom<&'a [u8]> for WithdrawData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}

impl<'a> InstructionData<'a> for WithdrawData {
    const LEN: usize = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_withdraw_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = WithdrawData::try_from(&data[..]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_withdraw_data_try_from_with_extra_bytes() {
        let data = [1u8, 2, 3, 4, 5];
        let result = WithdrawData::try_from(&data[..]);
        assert!(result.is_ok());
    }
}
