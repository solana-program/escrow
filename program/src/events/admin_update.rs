use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct AdminUpdateEvent {
    pub escrow: Address,
    pub old_admin: Address,
    pub new_admin: Address,
}

impl EventDiscriminator for AdminUpdateEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::AdminUpdate as u8;
}

impl EventSerialize for AdminUpdateEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(self.old_admin.as_ref());
        data.extend_from_slice(self.new_admin.as_ref());
        data
    }
}

impl AdminUpdateEvent {
    pub const DATA_LEN: usize = 32 + 32 + 32; // escrow + old_admin + new_admin

    #[inline(always)]
    pub fn new(escrow: Address, old_admin: Address, new_admin: Address) -> Self {
        Self { escrow, old_admin, new_admin }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_admin_update_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let old_admin = Address::new_from_array([2u8; 32]);
        let new_admin = Address::new_from_array([3u8; 32]);
        let event = AdminUpdateEvent::new(escrow, old_admin, new_admin);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.old_admin, old_admin);
        assert_eq!(event.new_admin, new_admin);
    }

    #[test]
    fn test_admin_update_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let old_admin = Address::new_from_array([2u8; 32]);
        let new_admin = Address::new_from_array([3u8; 32]);
        let event = AdminUpdateEvent::new(escrow, old_admin, new_admin);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + AdminUpdateEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::AdminUpdate as u8);
    }
}
