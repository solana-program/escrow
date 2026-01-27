use alloc::vec::Vec;
use pinocchio::{
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

use crate::{assert_no_padding, errors::EscrowProgramError, require_len, traits::ExtensionData};

/// Timelock extension data (stored in TLV format)
///
/// This is escrow config only - stores lock duration.
/// The actual deposit_timestamp is tracked per-deposit in Receipt accounts.
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(C)]
pub struct TimelockData {
    pub lock_duration: u64,
}

assert_no_padding!(TimelockData, 8);

impl TimelockData {
    pub const LEN: usize = 8;

    pub fn new(lock_duration: u64) -> Self {
        Self { lock_duration }
    }

    /// Check if timelock is enabled
    pub fn is_enabled(&self) -> bool {
        self.lock_duration != 0
    }

    /// Validate timelock constraint against the deposit timestamp
    pub fn validate(&self, deposited_at: i64) -> ProgramResult {
        if !self.is_enabled() {
            return Ok(());
        }

        let unlock_time =
            deposited_at.checked_add(self.lock_duration as i64).ok_or(ProgramError::ArithmeticOverflow)?;
        let clock = Clock::get()?;
        if clock.unix_timestamp < unlock_time {
            return Err(EscrowProgramError::TimelockNotExpired.into());
        }
        Ok(())
    }
}

impl ExtensionData for TimelockData {
    fn to_bytes(&self) -> Vec<u8> {
        self.lock_duration.to_le_bytes().to_vec()
    }

    fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        require_len!(data, Self::LEN);

        Ok(Self { lock_duration: u64::from_le_bytes(data[0..8].try_into().unwrap()) })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timelock_data_new() {
        let timelock = TimelockData::new(3600);
        assert_eq!(timelock.lock_duration, 3600);
    }

    #[test]
    fn test_timelock_data_roundtrip() {
        let timelock = TimelockData::new(7200);
        let bytes = timelock.to_bytes();
        let parsed = TimelockData::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, timelock);
    }

    #[test]
    fn test_timelock_is_enabled() {
        let no_timelock = TimelockData::new(0);
        assert!(!no_timelock.is_enabled());

        let with_duration = TimelockData::new(3600);
        assert!(with_duration.is_enabled());
    }
}
