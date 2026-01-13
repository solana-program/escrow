use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct AllowMintEvent {
    pub escrow: Address,
    pub mint: Address,
}

impl EventDiscriminator for AllowMintEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::AllowMint as u8;
}

impl EventSerialize for AllowMintEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(self.mint.as_ref());
        data
    }
}

impl AllowMintEvent {
    pub const DATA_LEN: usize = 32 + 32; // escrow + mint

    #[inline(always)]
    pub fn new(escrow: Address, mint: Address) -> Self {
        Self { escrow, mint }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_allow_mint_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let mint = Address::new_from_array([2u8; 32]);

        let event = AllowMintEvent::new(escrow, mint);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.mint, mint);
    }

    #[test]
    fn test_allow_mint_event_to_bytes_inner() {
        let escrow = Address::new_from_array([1u8; 32]);
        let mint = Address::new_from_array([2u8; 32]);
        let event = AllowMintEvent::new(escrow, mint);

        let bytes = event.to_bytes_inner();
        assert_eq!(bytes.len(), AllowMintEvent::DATA_LEN);
        assert_eq!(&bytes[..32], escrow.as_ref());
        assert_eq!(&bytes[32..64], mint.as_ref());
    }

    #[test]
    fn test_allow_mint_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let mint = Address::new_from_array([2u8; 32]);
        let event = AllowMintEvent::new(escrow, mint);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + AllowMintEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::AllowMint as u8);
        assert_eq!(&bytes[9..41], escrow.as_ref());
    }
}
