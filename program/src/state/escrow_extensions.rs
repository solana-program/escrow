use alloc::vec;
use alloc::vec::Vec;
use codama::CodamaAccount;
use pinocchio::{cpi::Seed, error::ProgramError, Address};

use crate::state::extensions::TimelockData;
use crate::traits::{AccountSerialize, Discriminator, EscrowAccountDiscriminators, PdaSeeds, Versioned};
use crate::{assert_no_padding, require_len, validate_discriminator};

/// Extension type discriminators for TLV-encoded extension data
#[repr(u16)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ExtensionType {
    Timelock = 0,
    Hook = 1,
    BlockedTokenExtensions = 2,
}

impl TryFrom<u16> for ExtensionType {
    type Error = ProgramError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Timelock),
            1 => Ok(Self::Hook),
            2 => Ok(Self::BlockedTokenExtensions),
            _ => Err(ProgramError::InvalidAccountData),
        }
    }
}

/// TLV header size: type (u16) + length (u16)
pub const TLV_HEADER_SIZE: usize = 4;

/// Escrow extensions header (fixed size, stored at start of account)
///
/// # PDA Seeds
/// `[b"extensions", escrow.as_ref()]`
///
/// # Account Layout
/// ```text
/// [discriminator: 1][version: 1][header: 2][TLV extensions: variable]
/// ```
#[derive(Clone, Debug, PartialEq, CodamaAccount)]
#[repr(C)]
pub struct EscrowExtensionsHeader {
    pub bump: u8,
    pub extension_count: u8,
}

assert_no_padding!(EscrowExtensionsHeader, 1 + 1);

impl Discriminator for EscrowExtensionsHeader {
    const DISCRIMINATOR: u8 = EscrowAccountDiscriminators::EscrowExtensionsDiscriminator as u8;
}

impl Versioned for EscrowExtensionsHeader {
    const VERSION: u8 = 1;
}

impl AccountSerialize for EscrowExtensionsHeader {
    fn to_bytes_inner(&self) -> Vec<u8> {
        vec![self.bump, self.extension_count]
    }
}

impl EscrowExtensionsHeader {
    pub const DATA_LEN: usize = 1 + 1; // bump + extension_count
    pub const LEN: usize = 1 + 1 + Self::DATA_LEN; // discriminator + version + data

    pub fn new(bump: u8, extension_count: u8) -> Self {
        Self { bump, extension_count }
    }

    /// Parse header from account data (validates discriminator, skips version)
    pub fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        require_len!(data, Self::LEN);

        validate_discriminator!(data, Self::DISCRIMINATOR);

        // Skip discriminator (byte 0) and version (byte 1)
        Ok(Self { bump: data[2], extension_count: data[3] })
    }

    /// Get the byte offset where TLV data starts
    pub const fn tlv_offset() -> usize {
        Self::LEN
    }
}

/// PDA context for extensions - holds escrow address for seed derivation
///
/// Implements `PdaSeeds` trait for consistent PDA handling across codebase.
pub struct ExtensionsPda<'a> {
    pub escrow: &'a Address,
}

impl<'a> ExtensionsPda<'a> {
    pub fn new(escrow: &'a Address) -> Self {
        Self { escrow }
    }
}

impl PdaSeeds for ExtensionsPda<'_> {
    const PREFIX: &'static [u8] = b"extensions";

    fn seeds(&self) -> Vec<&[u8]> {
        vec![Self::PREFIX, self.escrow.as_ref()]
    }

    fn seeds_with_bump<'a>(&'a self, bump: &'a [u8; 1]) -> Vec<Seed<'a>> {
        vec![Seed::from(Self::PREFIX), Seed::from(self.escrow.as_ref()), Seed::from(bump.as_slice())]
    }
}

/// Calculate total account size for extensions PDA
pub fn calculate_extensions_account_size(has_timelock: bool) -> usize {
    let mut size = EscrowExtensionsHeader::LEN;
    if has_timelock {
        size += TLV_HEADER_SIZE + TimelockData::LEN;
    }
    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_type_try_from() {
        assert_eq!(ExtensionType::try_from(0u16).unwrap(), ExtensionType::Timelock);
        assert_eq!(ExtensionType::try_from(1u16).unwrap(), ExtensionType::Hook);
        assert!(ExtensionType::try_from(999u16).is_err());
    }

    #[test]
    fn test_header_new() {
        let header = EscrowExtensionsHeader::new(255, 1);

        assert_eq!(header.bump, 255);
        assert_eq!(header.extension_count, 1);
    }

    #[test]
    fn test_header_to_bytes() {
        let header = EscrowExtensionsHeader::new(100, 2);
        let bytes = header.to_bytes();

        assert_eq!(bytes.len(), EscrowExtensionsHeader::LEN);
        assert_eq!(bytes[0], EscrowExtensionsHeader::DISCRIMINATOR);
        assert_eq!(bytes[1], EscrowExtensionsHeader::VERSION); // version auto-prepended
        assert_eq!(bytes[2], 100); // bump
        assert_eq!(bytes[3], 2); // extension_count
    }

    #[test]
    fn test_header_from_bytes() {
        let header = EscrowExtensionsHeader::new(100, 2);
        let bytes = header.to_bytes();

        let parsed = EscrowExtensionsHeader::from_bytes(&bytes).unwrap();
        assert_eq!(parsed.bump, header.bump);
        assert_eq!(parsed.extension_count, header.extension_count);
    }

    #[test]
    fn test_calculate_extensions_account_size() {
        let no_extensions = calculate_extensions_account_size(false);
        assert_eq!(no_extensions, EscrowExtensionsHeader::LEN);

        let with_timelock = calculate_extensions_account_size(true);
        assert_eq!(with_timelock, EscrowExtensionsHeader::LEN + TLV_HEADER_SIZE + TimelockData::LEN);
    }

    #[test]
    fn test_extensions_pda_seeds() {
        let escrow = Address::new_from_array([1u8; 32]);
        let pda = ExtensionsPda::new(&escrow);

        let seeds = pda.seeds();
        assert_eq!(seeds.len(), 2);
        assert_eq!(seeds[0], b"extensions");
        assert_eq!(seeds[1], escrow.as_ref());
    }
}
