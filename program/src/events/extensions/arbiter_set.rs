use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct ArbiterSetEvent {
    pub escrow: Address,
    pub arbiter: Address,
}

impl EventDiscriminator for ArbiterSetEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::ArbiterSet as u8;
}

impl EventSerialize for ArbiterSetEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(self.arbiter.as_ref());
        data
    }
}

impl ArbiterSetEvent {
    pub const DATA_LEN: usize = 32 + 32; // escrow + arbiter

    #[inline(always)]
    pub fn new(escrow: Address, arbiter: Address) -> Self {
        Self { escrow, arbiter }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_arbiter_set_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let arbiter = Address::new_from_array([2u8; 32]);
        let event = ArbiterSetEvent::new(escrow, arbiter);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.arbiter, arbiter);
    }

    #[test]
    fn test_arbiter_set_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let arbiter = Address::new_from_array([0u8; 32]);
        let event = ArbiterSetEvent::new(escrow, arbiter);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + ArbiterSetEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::ArbiterSet as u8);
    }
}
