use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct ExtensionRemovedEvent {
    pub escrow: Address,
    pub extension_type: u16,
}

impl EventDiscriminator for ExtensionRemovedEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::ExtensionRemoved as u8;
}

impl EventSerialize for ExtensionRemovedEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(&self.extension_type.to_le_bytes());
        data
    }
}

impl ExtensionRemovedEvent {
    pub const DATA_LEN: usize = 32 + 2; // escrow + extension_type

    #[inline(always)]
    pub fn new(escrow: Address, extension_type: u16) -> Self {
        Self { escrow, extension_type }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_extension_removed_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let event = ExtensionRemovedEvent::new(escrow, 2);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.extension_type, 2);
    }

    #[test]
    fn test_extension_removed_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let event = ExtensionRemovedEvent::new(escrow, 3);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + ExtensionRemovedEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::ExtensionRemoved as u8);

        let extension_type_offset = EVENT_DISCRIMINATOR_LEN + 32;
        assert_eq!(u16::from_le_bytes([bytes[extension_type_offset], bytes[extension_type_offset + 1]]), 3);
    }
}
