use pinocchio::error::ProgramError;

use crate::traits::InstructionData;

/// Instruction data for SetImmutable
///
/// No additional data is required.
pub struct SetImmutableData;

impl<'a> TryFrom<&'a [u8]> for SetImmutableData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

impl<'a> InstructionData<'a> for SetImmutableData {
    const LEN: usize = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_immutable_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = SetImmutableData::try_from(&data[..]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_set_immutable_data_try_from_with_extra_bytes() {
        let data = [1u8, 2, 3];
        let result = SetImmutableData::try_from(&data[..]);
        assert!(result.is_ok());
    }
}
