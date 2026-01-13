use pinocchio::{
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    ProgramResult,
};

use crate::{assert_no_padding, errors::EscrowProgramError, require_len, utils::ValidationContext};

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

    pub fn to_bytes(&self) -> [u8; Self::LEN] {
        self.lock_duration.to_le_bytes()
    }

    pub fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        require_len!(data, Self::LEN);

        Ok(Self { lock_duration: u64::from_le_bytes(data[0..8].try_into().unwrap()) })
    }

    /// Check if timelock is enabled
    pub fn is_enabled(&self) -> bool {
        self.lock_duration != 0
    }

    /// Validate timelock constraint
    pub fn validate(data: &[u8], ctx: &ValidationContext) -> ProgramResult {
        let timelock = Self::from_bytes(data)?;
        if timelock.is_enabled() {
            let clock = Clock::get()?;
            let unlock_time =
                ctx.deposited_at.checked_add(timelock.lock_duration as i64).ok_or(ProgramError::ArithmeticOverflow)?;
            if clock.unix_timestamp < unlock_time {
                return Err(EscrowProgramError::TimelockNotExpired.into());
            }
        }
        Ok(())
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
