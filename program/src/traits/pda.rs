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

    /// Validate that account matches the canonical PDA for these seeds.
    ///
    /// This enforces the canonical bump (highest valid bump) returned by
    /// `find_program_address`.
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

    /// Validate that account matches PDA derived with the provided bump.
    ///
    /// This does not enforce canonical bump selection. Use `validate_pda`
    /// when canonical bump invariants must be enforced (e.g., account creation).
    #[inline(always)]
    fn validate_pda_with_bump(
        &self,
        account: &AccountView,
        program_id: &Address,
        bump: u8,
    ) -> Result<(), ProgramError> {
        let seeds = self.seeds();
        let bump_arr = [bump];
        let mut seeds_with_bump = seeds;
        seeds_with_bump.push(&bump_arr[..]);
        let derived =
            Address::create_program_address(&seeds_with_bump, program_id).map_err(|_| ProgramError::InvalidSeeds)?;
        if account.address() != &derived {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(())
    }

    /// Validate that account address matches derived PDA and return the canonical bump.
    ///
    /// Uses `find_program_address` — only call this when the bump is not already known.
    /// When bump is available, prefer `validate_pda` or `validate_self`.
    #[inline(always)]
    fn validate_pda_address(&self, account: &AccountView, program_id: &Address) -> Result<u8, ProgramError> {
        let (derived, bump) = self.derive_address(program_id);
        if account.address() != &derived {
            return Err(ProgramError::InvalidSeeds);
        }
        Ok(bump)
    }
}

/// Extension trait for account types that store their PDA bump.
/// Provides convenience methods that use the stored bump value.
pub trait PdaAccount: PdaSeeds {
    /// Returns the stored bump seed for this account's PDA
    fn bump(&self) -> u8;

    /// Validate that account matches derived PDA using stored bump
    #[inline(always)]
    fn validate_self(&self, account: &AccountView, program_id: &Address) -> Result<(), ProgramError> {
        self.validate_pda_with_bump(account, program_id, self.bump())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Escrow;
    use crate::ID;
    use alloc::vec;

    struct TestPdaAccount {
        pub seed: Address,
        pub bump: u8,
    }

    impl PdaSeeds for TestPdaAccount {
        const PREFIX: &'static [u8] = b"test_account";

        fn seeds(&self) -> Vec<&[u8]> {
            vec![Self::PREFIX, self.seed.as_ref()]
        }

        fn seeds_with_bump<'a>(&'a self, bump: &'a [u8; 1]) -> Vec<Seed<'a>> {
            vec![Seed::from(Self::PREFIX), Seed::from(self.seed.as_ref()), Seed::from(&bump[..])]
        }
    }

    impl PdaAccount for TestPdaAccount {
        fn bump(&self) -> u8 {
            self.bump
        }
    }

    #[test]
    fn test_derive_address_deterministic() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        let escrow = Escrow::new(0, escrow_seed, admin, false);

        let (address1, bump1) = escrow.derive_address(&ID);
        let (address2, bump2) = escrow.derive_address(&ID);

        assert_eq!(address1, address2);
        assert_eq!(bump1, bump2);
    }

    #[test]
    fn test_derive_address_different_seeds() {
        let admin = Address::new_from_array([2u8; 32]);

        let escrow1 = Escrow::new(0, Address::new_from_array([1u8; 32]), admin, false);
        let escrow2 = Escrow::new(0, Address::new_from_array([3u8; 32]), admin, false);

        let (address1, _) = escrow1.derive_address(&ID);
        let (address2, _) = escrow2.derive_address(&ID);

        assert_ne!(address1, address2);
    }

    #[test]
    fn test_pda_account_bump() {
        let account = TestPdaAccount { seed: Address::new_from_array([1u8; 32]), bump: 254 };
        assert_eq!(account.bump(), 254);
    }

    #[test]
    fn test_pda_account_inherits_pda_seeds() {
        let account = TestPdaAccount { seed: Address::new_from_array([1u8; 32]), bump: 255 };
        let seeds = account.seeds();
        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0], TestPdaAccount::PREFIX);
    }

    #[test]
    fn test_pda_account_derive_address() {
        let account = TestPdaAccount { seed: Address::new_from_array([1u8; 32]), bump: 255 };
        let (address, _bump) = account.derive_address(&ID);
        assert!(!address.as_ref().iter().all(|&b| b == 0));
    }
}
