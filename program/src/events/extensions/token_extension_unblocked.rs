use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

/// Event emitted when a token extension is unblocked.
#[derive(CodamaType)]
pub struct TokenExtensionUnblocked {
    pub escrow: Address,
    pub unblocked_extension: u16,
}

impl EventDiscriminator for TokenExtensionUnblocked {
    const DISCRIMINATOR: u8 = EventDiscriminators::TokenExtensionUnblocked as u8;
}

impl EventSerialize for TokenExtensionUnblocked {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(&self.unblocked_extension.to_le_bytes());
        data
    }
}

impl TokenExtensionUnblocked {
    pub const DATA_LEN: usize = 32 + 2; // escrow + unblocked_extension

    #[inline(always)]
    pub fn new(escrow: Address, unblocked_extension: u16) -> Self {
        Self { escrow, unblocked_extension }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_token_extension_unblocked_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let event = TokenExtensionUnblocked::new(escrow, 42);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.unblocked_extension, 42);
    }

    #[test]
    fn test_token_extension_unblocked_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let event = TokenExtensionUnblocked::new(escrow, 100);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + TokenExtensionUnblocked::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::TokenExtensionUnblocked as u8);
        assert_eq!(&bytes[9..41], escrow.as_ref());
        assert_eq!(u16::from_le_bytes([bytes[41], bytes[42]]), 100);
    }
}
