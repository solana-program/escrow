use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct TimelockAddedEvent {
    pub escrow: Address,
    pub lock_duration: u64,
}

impl EventDiscriminator for TimelockAddedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::TimelockAdded as u8;
}

impl EventSerialize for TimelockAddedEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(&self.lock_duration.to_le_bytes());
        data
    }
}

impl TimelockAddedEvent {
    pub const DATA_LEN: usize = 32 + 8; // escrow + lock_duration

    #[inline(always)]
    pub fn new(escrow: Address, lock_duration: u64) -> Self {
        Self { escrow, lock_duration }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_timelock_added_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let event = TimelockAddedEvent::new(escrow, 3600);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.lock_duration, 3600);
    }

    #[test]
    fn test_timelock_added_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let event = TimelockAddedEvent::new(escrow, 0);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + TimelockAddedEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::TimelockAdded as u8);
    }
}
