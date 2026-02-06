use alloc::vec;
use alloc::vec::Vec;
use codama::CodamaAccount;
use pinocchio::{
    account::AccountView,
    cpi::{Seed, Signer},
    error::ProgramError,
    Address,
};

use crate::assert_no_padding;
use crate::errors::EscrowProgramError;
use crate::traits::{
    AccountDeserialize, AccountSerialize, AccountSize, Discriminator, EscrowAccountDiscriminators, PdaAccount,
    PdaSeeds, Versioned,
};

/// Escrow account state
///
/// # PDA Seeds
/// `[b"escrow", escrow_seed.as_ref()]`
#[derive(Clone, Debug, PartialEq, CodamaAccount)]
#[repr(C)]
pub struct Escrow {
    pub bump: u8,
    pub escrow_seed: Address,
    pub admin: Address,
}

assert_no_padding!(Escrow, 1 + 32 + 32);

impl Discriminator for Escrow {
    const DISCRIMINATOR: u8 = EscrowAccountDiscriminators::EscrowDiscriminator as u8;
}

impl Versioned for Escrow {
    const VERSION: u8 = 1;
}

impl AccountSize for Escrow {
    const DATA_LEN: usize = 1 + 32 + 32; // bump + escrow_seed + admin
}

impl AccountDeserialize for Escrow {}

impl AccountSerialize for Escrow {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.push(self.bump);
        data.extend_from_slice(self.escrow_seed.as_ref());
        data.extend_from_slice(self.admin.as_ref());
        data
    }
}

impl PdaSeeds for Escrow {
    const PREFIX: &'static [u8] = b"escrow";

    #[inline(always)]
    fn seeds(&self) -> Vec<&[u8]> {
        vec![Self::PREFIX, self.escrow_seed.as_ref()]
    }

    #[inline(always)]
    fn seeds_with_bump<'a>(&'a self, bump: &'a [u8; 1]) -> Vec<Seed<'a>> {
        vec![Seed::from(Self::PREFIX), Seed::from(self.escrow_seed.as_ref()), Seed::from(bump.as_slice())]
    }
}

impl PdaAccount for Escrow {
    #[inline(always)]
    fn bump(&self) -> u8 {
        self.bump
    }
}

impl Escrow {
    #[inline(always)]
    pub fn new(bump: u8, escrow_seed: Address, admin: Address) -> Self {
        Self { bump, escrow_seed, admin }
    }

    #[inline(always)]
    pub fn from_account<'a>(
        data: &'a [u8],
        account: &AccountView,
        program_id: &Address,
    ) -> Result<&'a Self, ProgramError> {
        let state = Self::from_bytes(data)?;
        state.validate_self(account, program_id)?;
        Ok(state)
    }

    #[inline(always)]
    pub fn validate_admin(&self, provided_admin: &Address) -> Result<(), ProgramError> {
        if self.admin != *provided_admin {
            return Err(EscrowProgramError::InvalidAdmin.into());
        }
        Ok(())
    }

    /// Execute a CPI with this escrow PDA as signer
    #[inline(always)]
    pub fn with_signer<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&[Signer<'_, '_>]) -> R,
    {
        let bump_seed = [self.bump];
        let seeds = [Seed::from(Self::PREFIX), Seed::from(self.escrow_seed.as_ref()), Seed::from(bump_seed.as_slice())];
        let signers = [Signer::from(&seeds)];
        f(&signers)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_escrow() -> Escrow {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);
        Escrow::new(255, escrow_seed, admin)
    }

    #[test]
    fn test_escrow_new() {
        let escrow_seed = Address::new_from_array([1u8; 32]);
        let admin = Address::new_from_array([2u8; 32]);

        let escrow = Escrow::new(200, escrow_seed, admin);

        assert_eq!(escrow.bump, 200);
        assert_eq!(escrow.escrow_seed, escrow_seed);
        assert_eq!(escrow.admin, admin);
    }

    #[test]
    fn test_escrow_validate_admin_success() {
        let escrow = create_test_escrow();
        let valid_admin = Address::new_from_array([2u8; 32]);

        assert!(escrow.validate_admin(&valid_admin).is_ok());
    }

    #[test]
    fn test_escrow_validate_admin_invalid() {
        let escrow = create_test_escrow();
        let invalid_admin = Address::new_from_array([99u8; 32]);

        let result = escrow.validate_admin(&invalid_admin);
        assert_eq!(result, Err(EscrowProgramError::InvalidAdmin.into()));
    }

    #[test]
    fn test_escrow_to_bytes_inner() {
        let escrow = create_test_escrow();
        let bytes = escrow.to_bytes_inner();

        assert_eq!(bytes.len(), Escrow::DATA_LEN);
        assert_eq!(bytes[0], 255); // bump
        assert_eq!(&bytes[1..33], &[1u8; 32]); // escrow_seed
        assert_eq!(&bytes[33..65], &[2u8; 32]); // admin
    }

    #[test]
    fn test_escrow_to_bytes() {
        let escrow = create_test_escrow();
        let bytes = escrow.to_bytes();

        assert_eq!(bytes.len(), Escrow::LEN);
        assert_eq!(bytes[0], Escrow::DISCRIMINATOR);
        assert_eq!(bytes[1], Escrow::VERSION); // version auto-prepended
        assert_eq!(bytes[2], 255); // bump
    }

    #[test]
    fn test_escrow_from_bytes() {
        let escrow = create_test_escrow();
        let bytes = escrow.to_bytes();

        let deserialized = Escrow::from_bytes(&bytes).unwrap();

        assert_eq!(deserialized.bump, escrow.bump);
        assert_eq!(deserialized.escrow_seed, escrow.escrow_seed);
        assert_eq!(deserialized.admin, escrow.admin);
    }

    #[test]
    fn test_escrow_from_bytes_too_short() {
        let data = [0u8; 10];
        let result = Escrow::from_bytes(&data);
        assert_eq!(result, Err(ProgramError::InvalidInstructionData));
    }

    #[test]
    fn test_escrow_from_bytes_wrong_discriminator() {
        let mut bytes = [0u8; 67];
        bytes[0] = 99; // wrong discriminator
        let result = Escrow::from_bytes(&bytes);
        assert_eq!(result, Err(ProgramError::InvalidAccountData));
    }

    #[test]
    fn test_escrow_seeds() {
        let escrow = create_test_escrow();
        let seeds = escrow.seeds();

        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0], Escrow::PREFIX);
        assert_eq!(seeds[1], escrow.escrow_seed.as_ref());
    }

    #[test]
    fn test_escrow_seeds_with_bump() {
        let escrow = create_test_escrow();
        let bump = [255u8];
        let seeds = escrow.seeds_with_bump(&bump);

        assert_eq!(seeds.len(), 3);
    }

    #[test]
    fn test_escrow_write_to_slice() {
        let escrow = create_test_escrow();
        let mut dest = [0u8; 100];

        assert!(escrow.write_to_slice(&mut dest).is_ok());
        assert_eq!(dest[0], Escrow::DISCRIMINATOR);
        assert_eq!(dest[1], Escrow::VERSION); // version
        assert_eq!(dest[2], escrow.bump);
    }

    #[test]
    fn test_escrow_write_to_slice_too_small() {
        let escrow = create_test_escrow();
        let mut dest = [0u8; 10];

        let result = escrow.write_to_slice(&mut dest);
        assert_eq!(result, Err(ProgramError::AccountDataTooSmall));
    }
}
