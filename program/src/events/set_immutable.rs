use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct SetImmutableEvent {
    pub escrow: Address,
    pub admin: Address,
}

impl EventDiscriminator for SetImmutableEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::SetImmutable as u8;
}

impl EventSerialize for SetImmutableEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(self.admin.as_ref());
        data
    }
}

impl SetImmutableEvent {
    pub const DATA_LEN: usize = 32 + 32; // escrow + admin

    #[inline(always)]
    pub fn new(escrow: Address, admin: Address) -> Self {
        Self { escrow, admin }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_set_immutable_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let event = SetImmutableEvent::new(escrow, admin);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.admin, admin);
    }

    #[test]
    fn test_set_immutable_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let event = SetImmutableEvent::new(escrow, admin);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + SetImmutableEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::SetImmutable as u8);
    }
}
