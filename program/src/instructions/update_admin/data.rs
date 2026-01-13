use pinocchio::error::ProgramError;

use crate::traits::InstructionData;

/// Instruction data for UpdateAdmin
///
/// No additional data needed - new admin is read from accounts
pub struct UpdateAdminData;

impl<'a> TryFrom<&'a [u8]> for UpdateAdminData {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(_data: &'a [u8]) -> Result<Self, Self::Error> {
        Ok(Self)
    }
}

impl<'a> InstructionData<'a> for UpdateAdminData {
    const LEN: usize = 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_admin_data_try_from_empty() {
        let data: [u8; 0] = [];
        let result = UpdateAdminData::try_from(&data[..]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_admin_data_try_from_with_extra_bytes() {
        let data = [1u8, 2, 3];
        let result = UpdateAdminData::try_from(&data[..]);
        assert!(result.is_ok());
    }
}
