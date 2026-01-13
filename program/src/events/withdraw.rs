use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct WithdrawEvent {
    pub escrow: Address,
    pub withdrawer: Address,
    pub mint: Address,
    pub receipt_seed: Address,
    pub amount: u64,
}

impl EventDiscriminator for WithdrawEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::Withdraw as u8;
}

impl EventSerialize for WithdrawEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(self.withdrawer.as_ref());
        data.extend_from_slice(self.mint.as_ref());
        data.extend_from_slice(self.receipt_seed.as_ref());
        data.extend_from_slice(&self.amount.to_le_bytes());
        data
    }
}

impl WithdrawEvent {
    pub const DATA_LEN: usize = 32 + 32 + 32 + 32 + 8; // escrow + withdrawer + mint + receipt_seed + amount

    #[inline(always)]
    pub fn new(escrow: Address, withdrawer: Address, mint: Address, receipt_seed: Address, amount: u64) -> Self {
        Self { escrow, withdrawer, mint, receipt_seed, amount }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_withdraw_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let withdrawer = Address::new_from_array([2u8; 32]);
        let mint = Address::new_from_array([3u8; 32]);
        let receipt_seed = Address::new_from_array([4u8; 32]);

        let event = WithdrawEvent::new(escrow, withdrawer, mint, receipt_seed, 1000);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.withdrawer, withdrawer);
        assert_eq!(event.mint, mint);
        assert_eq!(event.receipt_seed, receipt_seed);
        assert_eq!(event.amount, 1000);
    }

    #[test]
    fn test_withdraw_event_to_bytes_inner() {
        let escrow = Address::new_from_array([1u8; 32]);
        let withdrawer = Address::new_from_array([2u8; 32]);
        let mint = Address::new_from_array([3u8; 32]);
        let receipt_seed = Address::new_from_array([4u8; 32]);
        let event = WithdrawEvent::new(escrow, withdrawer, mint, receipt_seed, 5000);

        let bytes = event.to_bytes_inner();
        assert_eq!(bytes.len(), WithdrawEvent::DATA_LEN);
        assert_eq!(&bytes[..32], escrow.as_ref());
        assert_eq!(&bytes[32..64], withdrawer.as_ref());
        assert_eq!(&bytes[64..96], mint.as_ref());
        assert_eq!(&bytes[96..128], receipt_seed.as_ref());
        assert_eq!(&bytes[128..136], &5000u64.to_le_bytes());
    }

    #[test]
    fn test_withdraw_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let withdrawer = Address::new_from_array([2u8; 32]);
        let mint = Address::new_from_array([3u8; 32]);
        let receipt_seed = Address::new_from_array([4u8; 32]);
        let event = WithdrawEvent::new(escrow, withdrawer, mint, receipt_seed, 1000);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + WithdrawEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::Withdraw as u8);
        assert_eq!(&bytes[9..41], escrow.as_ref());
    }
}
