use alloc::vec::Vec;
use pinocchio::{account::AccountView, error::ProgramError, ProgramResult};

use crate::{assert_no_padding, errors::EscrowProgramError, require_len, traits::ExtensionData};

/// Arbiter extension data (stored in TLV format)
///
/// Stores the address of a third-party signer who must authorize withdrawals.
/// Once set, this is immutable — the arbiter cannot be changed.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct ArbiterData {
    pub arbiter: pinocchio::Address,
}

assert_no_padding!(ArbiterData, 32);

impl ArbiterData {
    pub const LEN: usize = 32;

    pub fn new(arbiter: pinocchio::Address) -> Self {
        Self { arbiter }
    }

    /// Validate that the arbiter account is present and is a signer.
    ///
    /// The arbiter must be the first account in remaining_accounts.
    /// Callers are responsible for advancing past the consumed account.
    pub fn validate(&self, remaining_accounts: &[AccountView]) -> ProgramResult {
        let arbiter_account = remaining_accounts.first().ok_or(EscrowProgramError::InvalidArbiter)?;

        if arbiter_account.address() != &self.arbiter {
            return Err(EscrowProgramError::InvalidArbiter.into());
        }

        if !arbiter_account.is_signer() {
            return Err(EscrowProgramError::InvalidArbiter.into());
        }

        Ok(())
    }
}

impl ExtensionData for ArbiterData {
    fn to_bytes(&self) -> Vec<u8> {
        self.arbiter.as_array().to_vec()
    }

    fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        require_len!(data, Self::LEN);

        Ok(Self { arbiter: pinocchio::Address::new_from_array(data[0..32].try_into().unwrap()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pinocchio::Address;

    #[test]
    fn test_arbiter_data_new() {
        let arbiter = Address::new_from_array([1u8; 32]);
        let data = ArbiterData::new(arbiter);
        assert_eq!(data.arbiter, arbiter);
    }

    #[test]
    fn test_arbiter_data_roundtrip() {
        let arbiter = Address::new_from_array([2u8; 32]);
        let data = ArbiterData::new(arbiter);
        let bytes = data.to_bytes();
        let parsed = ArbiterData::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, data);
    }
}
