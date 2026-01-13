use alloc::vec;
use alloc::vec::Vec;
use codama::CodamaAccount;
use pinocchio::{cpi::Seed, error::ProgramError, Address};

use crate::assert_no_padding;
use crate::traits::{
    AccountDeserialize, AccountSerialize, AccountSize, Discriminator, EscrowAccountDiscriminators, PdaSeeds, Versioned,
};

/// AllowedMint account state
///
/// Represents a mint that is allowed for deposits into a specific escrow.
/// The existence of this PDA indicates the mint is allowed.
/// The PDA seeds themselves validate the escrow and mint relationship.
///
/// # PDA Seeds
/// `[b"allowed_mint", escrow.as_ref(), mint.as_ref()]`
#[derive(Clone, Debug, PartialEq, CodamaAccount)]
#[repr(C)]
pub struct AllowedMint {
    pub bump: u8,
}

assert_no_padding!(AllowedMint, 1);

impl Discriminator for AllowedMint {
    const DISCRIMINATOR: u8 = EscrowAccountDiscriminators::AllowedMintDiscriminator as u8;
}

impl Versioned for AllowedMint {
    const VERSION: u8 = 1;
}

impl AccountSize for AllowedMint {
    const DATA_LEN: usize = 1; // bump only
}

impl AccountDeserialize for AllowedMint {}

impl AccountSerialize for AllowedMint {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        vec![self.bump]
    }
}

impl AllowedMint {
    #[inline(always)]
    pub fn new(bump: u8) -> Self {
        Self { bump }
    }

    #[inline(always)]
    pub fn from_account(data: &[u8]) -> Result<&Self, ProgramError> {
        Self::from_bytes(data)
    }
}

/// PDA context for AllowedMint - holds escrow and mint addresses for seed derivation
///
/// Since the AllowedMint account only stores the bump, we use this helper
/// to derive and validate PDAs using external escrow and mint addresses.
///
/// Implements `PdaSeeds` trait for consistent PDA handling across codebase.
pub struct AllowedMintPda<'a> {
    pub escrow: &'a Address,
    pub mint: &'a Address,
}

impl<'a> AllowedMintPda<'a> {
    #[inline(always)]
    pub fn new(escrow: &'a Address, mint: &'a Address) -> Self {
        Self { escrow, mint }
    }
}

impl PdaSeeds for AllowedMintPda<'_> {
    const PREFIX: &'static [u8] = b"allowed_mint";

    #[inline(always)]
    fn seeds(&self) -> Vec<&[u8]> {
        vec![Self::PREFIX, self.escrow.as_ref(), self.mint.as_ref()]
    }

    #[inline(always)]
    fn seeds_with_bump<'a>(&'a self, bump: &'a [u8; 1]) -> Vec<Seed<'a>> {
        vec![
            Seed::from(Self::PREFIX),
            Seed::from(self.escrow.as_ref()),
            Seed::from(self.mint.as_ref()),
            Seed::from(bump.as_slice()),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_allowed_mint() -> AllowedMint {
        AllowedMint::new(255)
    }

    fn create_test_seeds() -> (Address, Address) {
        let escrow = Address::new_from_array([1u8; 32]);
        let mint = Address::new_from_array([2u8; 32]);
        (escrow, mint)
    }

    #[test]
    fn test_allowed_mint_new() {
        let allowed_mint = AllowedMint::new(200);
        assert_eq!(allowed_mint.bump, 200);
    }

    #[test]
    fn test_allowed_mint_to_bytes_inner() {
        let allowed_mint = create_test_allowed_mint();
        let bytes = allowed_mint.to_bytes_inner();

        assert_eq!(bytes.len(), AllowedMint::DATA_LEN);
        assert_eq!(bytes[0], 255); // bump
    }

    #[test]
    fn test_allowed_mint_to_bytes() {
        let allowed_mint = create_test_allowed_mint();
        let bytes = allowed_mint.to_bytes();

        assert_eq!(bytes.len(), AllowedMint::LEN);
        assert_eq!(bytes[0], AllowedMint::DISCRIMINATOR);
        assert_eq!(bytes[1], AllowedMint::VERSION); // version auto-prepended
        assert_eq!(bytes[2], 255); // bump
    }

    #[test]
    fn test_allowed_mint_from_bytes() {
        let allowed_mint = create_test_allowed_mint();
        let bytes = allowed_mint.to_bytes();

        let deserialized = AllowedMint::from_bytes(&bytes).unwrap();

        assert_eq!(deserialized.bump, allowed_mint.bump);
    }

    #[test]
    fn test_allowed_mint_from_bytes_too_short() {
        let data = [0u8; 2]; // Only discriminator + version, no bump
        let result = AllowedMint::from_bytes(&data);
        assert_eq!(result, Err(ProgramError::InvalidInstructionData));
    }

    #[test]
    fn test_allowed_mint_from_bytes_wrong_discriminator() {
        let mut bytes = [0u8; 3];
        bytes[0] = 99; // wrong discriminator
        let result = AllowedMint::from_bytes(&bytes);
        assert_eq!(result, Err(ProgramError::InvalidAccountData));
    }

    #[test]
    fn test_allowed_mint_pda_seeds() {
        let (escrow, mint) = create_test_seeds();
        let pda = AllowedMintPda::new(&escrow, &mint);
        let seeds = pda.seeds();

        assert_eq!(seeds.len(), 3);
        assert_eq!(seeds[0], AllowedMintPda::PREFIX);
        assert_eq!(seeds[1], escrow.as_ref());
        assert_eq!(seeds[2], mint.as_ref());
    }

    #[test]
    fn test_allowed_mint_pda_seeds_with_bump() {
        let (escrow, mint) = create_test_seeds();
        let pda = AllowedMintPda::new(&escrow, &mint);
        let bump = [255u8];
        let seeds = pda.seeds_with_bump(&bump);

        assert_eq!(seeds.len(), 4);
    }

    #[test]
    fn test_allowed_mint_write_to_slice() {
        let allowed_mint = create_test_allowed_mint();
        let mut dest = [0u8; 100];

        assert!(allowed_mint.write_to_slice(&mut dest).is_ok());
        assert_eq!(dest[0], AllowedMint::DISCRIMINATOR);
        assert_eq!(dest[1], AllowedMint::VERSION); // version
        assert_eq!(dest[2], allowed_mint.bump);
    }

    #[test]
    fn test_allowed_mint_write_to_slice_too_small() {
        let allowed_mint = create_test_allowed_mint();
        let mut dest = [0u8; 2]; // Only room for discriminator + version

        let result = allowed_mint.write_to_slice(&mut dest);
        assert_eq!(result, Err(ProgramError::AccountDataTooSmall));
    }
}
