use alloc::vec::Vec;
use pinocchio::{account::AccountView, cpi::Seed, error::ProgramError, Address};

/// PDA seed generation tied to state structs
pub trait PdaSeeds {
    /// Static prefix seed (e.g., b"escrow")
    const PREFIX: &'static [u8];

    /// Generate seeds for PDA derivation (without bump)
    /// Used for find_program_address
    fn seeds(&self) -> Vec<&[u8]>;

    /// Generate seeds with bump for signing
    /// Used for invoke_signed
    fn seeds_with_bump<'a>(&'a self, bump: &'a [u8; 1]) -> Vec<Seed<'a>>;

    /// Derive PDA address from seeds
    #[inline(always)]
    fn derive_address(&self, program_id: &Address) -> (Address, u8) {
        let seeds = self.seeds();
        Address::find_program_address(&seeds, program_id)
    }

    /// Validate that account matches derived PDA
    #[inline(always)]
    fn validate_pda(&self, account: &AccountView, program_id: &Address, expected_bump: u8) -> Result<(), ProgramError> {
        let (derived, bump) = self.derive_address(program_id);
        if bump != expected_bump {
            return Err(ProgramError::InvalidSeeds);
        }
        if account.address() != &derived {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(())
    }

    /// Validate that account address matches derived PDA, returns canonical bump
    #[inline(always)]
    fn validate_pda_address(&self, account: &AccountView, program_id: &Address) -> Result<u8, ProgramError> {
        let (derived, bump) = self.derive_address(program_id);
        if account.address() != &derived {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(bump)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Escrow;
    use crate::ID;

    #[test]
    fn test_derive_address_deterministic() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let escrow = Escrow::new(0, escrow_seed, admin);

        let (address1, bump1) = escrow.derive_address(&ID);
        let (address2, bump2) = escrow.derive_address(&ID);

        assert_eq!(address1, address2);
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn test_derive_address_different_seeds() {
        let admin = Address::new_from_array([2u8; 32]);

        let escrow1 = Escrow::new(0, Address::new_from_array([1u8; 32]), admin);
        let escrow2 = Escrow::new(0, Address::new_from_array([3u8; 32]), admin);

        let (address1, _) = escrow1.derive_address(&ID);
        let (address2, _) = escrow2.derive_address(&ID);

        assert_ne!(address1, address2);
    }
}
