use alloc::vec::Vec;
use codama::CodamaType;
use pinocchio::Address;

use crate::traits::{EventDiscriminator, EventDiscriminators, EventSerialize};

#[derive(CodamaType)]
pub struct HookSetEvent {
    pub escrow: Address,
    pub hook_program: Address,
}

impl EventDiscriminator for HookSetEvent {
    const DISCRIMINATOR: u8 = EventDiscriminators::HookSet as u8;
}

impl EventSerialize for HookSetEvent {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(self.hook_program.as_ref());
        data
    }
}

impl HookSetEvent {
    pub const DATA_LEN: usize = 32 + 32; // escrow + hook_program

    #[inline(always)]
    pub fn new(escrow: Address, hook_program: Address) -> Self {
        Self { escrow, hook_program }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::EVENT_IX_TAG_LE;
    use crate::traits::EVENT_DISCRIMINATOR_LEN;

    #[test]
    fn test_hook_set_event_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let hook_program = Address::new_from_array([2u8; 32]);
        let event = HookSetEvent::new(escrow, hook_program);

        assert_eq!(event.escrow, escrow);
        assert_eq!(event.hook_program, hook_program);
    }

    #[test]
    fn test_hook_set_event_to_bytes() {
        let escrow = Address::new_from_array([1u8; 32]);
        let hook_program = Address::new_from_array([0u8; 32]);
        let event = HookSetEvent::new(escrow, hook_program);

        let bytes = event.to_bytes();
        assert_eq!(bytes.len(), EVENT_DISCRIMINATOR_LEN + HookSetEvent::DATA_LEN);
        assert_eq!(&bytes[..8], EVENT_IX_TAG_LE);
        assert_eq!(bytes[8], EventDiscriminators::HookSet as u8);
    }
}
