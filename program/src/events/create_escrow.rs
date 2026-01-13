use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct CreatesEscrowEvent {
    pub escrow_seed: Address,
    pub admin: Address,
}

impl EventDiscriminator for CreatesEscrowEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::CreatesEscrow as u8;
}

impl EventSerialize for CreatesEscrowEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow_seed.as_ref());
        data.extend_from_slice(self.admin.as_ref());
        data
    }
}

impl CreatesEscrowEvent {
    pub const DATA_LEN: usize = 32 + 32; // escrow_seed + admin

    #[inline(always)]
    pub fn new(escrow_seed: Address, admin: Address) -> Self {
        Self { escrow_seed, admin }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_creates_escrow_event_new() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);

        let event = CreatesEscrowEvent::new(escrow_seed, admin);

        assert_eq!(event.escrow_seed, escrow_seed);
        assert_eq!(event.admin, admin);
    }

    #[test]
    fn test_creates_escrow_event_to_bytes_inner() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let event = CreatesEscrowEvent::new(escrow_seed, admin);

        let bytes = event.to_bytes_inner();
        assert_eq!(bytes.len(), CreatesEscrowEvent::DATA_LEN);
        assert_eq!(&bytes[..32], escrow_seed.as_ref());
        assert_eq!(&bytes[32..64], admin.as_ref());
    }

    #[test]
    fn test_creates_escrow_event_to_bytes() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let event = CreatesEscrowEvent::new(escrow_seed, admin);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + CreatesEscrowEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], 0);
        assert_eq!(&bytes[9..41], escrow_seed.as_ref());
        assert_eq!(&bytes[41..73], admin.as_ref());
    }
}
