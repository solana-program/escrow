use pinocchio::{error::ProgramError, Address};

use crate::{assert_no_padding, require_len};

/// Hook points for escrow operations
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HookPoint {
    PreDeposit = 0,
    PostDeposit = 1,
    PreWithdraw = 2,
    PostWithdraw = 3,
}

/// Hook extension data (stored in TLV format)
///
/// Stores the hook program address that will be invoked during escrow operations.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct HookData {
    pub hook_program: Address,
}

assert_no_padding!(HookData, 32);

impl HookData {
    pub const LEN: usize = 32;

    pub fn new(hook_program: Address) -> Self {
        Self { hook_program }
    }

    pub fn to_bytes(&self) -> [u8; Self::LEN] {
        *self.hook_program.as_array()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        require_len!(data, Self::LEN);

        Ok(Self { hook_program: Address::new_from_array(data[0..32].try_into().unwrap()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_data_new() {
        let program = Address::new_from_array([1u8; 32]);
        let hook = HookData::new(program);
        assert_eq!(hook.hook_program, program);
    }

    #[test]
    fn test_hook_data_roundtrip() {
        let program = Address::new_from_array([2u8; 32]);
        let hook = HookData::new(program);
        let bytes = hook.to_bytes();
        let parsed = HookData::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, hook);
    }
}
