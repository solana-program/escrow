use alloc::vec::Vec;
use pinocchio::error::ProgramError;

use crate::{errors::EscrowProgramError, require_len, traits::ExtensionData};

/// Block token extensions data (stored in TLV format)
///
/// Stores a list of Token-2022 ExtensionType values that should be blocked
/// for mints used with this escrow.
///
/// Uses dynamic sizing - serialized format is:
/// - 1 byte: count
/// - 2 bytes × count: each blocked extension
#[derive(Clone, Debug, PartialEq)]
pub struct BlockTokenExtensionsData {
    pub count: u8,
    pub blocked_extensions: Vec<u16>,
}

impl BlockTokenExtensionsData {
    pub fn new(blocked_extensions: &[u16]) -> Result<Self, ProgramError> {
        if blocked_extensions.len() > u8::MAX as usize {
            return Err(ProgramError::InvalidArgument);
        }

        Ok(Self { count: blocked_extensions.len() as u8, blocked_extensions: blocked_extensions.to_vec() })
    }

    /// Returns the serialized byte length: 1 (count) + 2 * count (extensions)
    pub fn byte_len(&self) -> usize {
        1 + (self.count as usize * 2)
    }

    /// Check if a token extension type is blocked
    pub fn is_blocked(&self, extension_type: u16) -> bool {
        self.blocked_extensions.contains(&extension_type)
    }

    /// Get the list of blocked extensions
    pub fn blocked_extensions(&self) -> &[u16] {
        &self.blocked_extensions
    }

    /// Add a single extension to the list
    ///
    /// Returns an error if the extension already exists
    pub fn add_extension(&mut self, extension: u16) -> Result<(), ProgramError> {
        if self.is_blocked(extension) {
            return Err(EscrowProgramError::TokenExtensionAlreadyBlocked.into());
        }

        if self.count == u8::MAX {
            return Err(ProgramError::InvalidArgument);
        }

        self.blocked_extensions.push(extension);
        self.count += 1;

        Ok(())
    }
}

impl ExtensionData for BlockTokenExtensionsData {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.byte_len());
        bytes.push(self.count);
        for ext in &self.blocked_extensions[..self.count as usize] {
            bytes.extend_from_slice(&ext.to_le_bytes());
        }
        bytes
    }

    fn from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        require_len!(data, 1);

        let count = data[0];
        let expected_len = 1 + (count as usize * 2);

        require_len!(data, expected_len);

        let mut blocked_extensions = Vec::with_capacity(count as usize);
        for i in 0..count as usize {
            let offset = 1 + (i * 2);
            blocked_extensions.push(u16::from_le_bytes([data[offset], data[offset + 1]]));
        }

        Ok(Self { count, blocked_extensions })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_token_extensions_data_new() {
        let extensions = [1u16, 2u16, 3u16];
        let data = BlockTokenExtensionsData::new(&extensions).unwrap();
        assert_eq!(data.count, 3);
        assert_eq!(data.blocked_extensions[0], 1);
        assert_eq!(data.blocked_extensions[1], 2);
        assert_eq!(data.blocked_extensions[2], 3);
    }

    #[test]
    fn test_block_token_extensions_data_new_empty() {
        let data = BlockTokenExtensionsData::new(&[]).unwrap();
        assert_eq!(data.count, 0);
    }

    #[test]
    fn test_block_token_extensions_data_large_count() {
        let extensions: Vec<u16> = (0..100).collect();
        let data = BlockTokenExtensionsData::new(&extensions).unwrap();
        assert_eq!(data.count, 100);
        assert_eq!(data.blocked_extensions.len(), 100);
    }

    #[test]
    fn test_block_token_extensions_data_roundtrip() {
        let extensions = [5u16, 10u16, 15u16];
        let data = BlockTokenExtensionsData::new(&extensions).unwrap();
        let bytes = data.to_bytes();
        let parsed = BlockTokenExtensionsData::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, data);
    }

    #[test]
    fn test_block_token_extensions_data_roundtrip_large() {
        let extensions: Vec<u16> = (0..50).collect();
        let data = BlockTokenExtensionsData::new(&extensions).unwrap();
        let bytes = data.to_bytes();
        assert_eq!(bytes.len(), 1 + 50 * 2);
        let parsed = BlockTokenExtensionsData::from_bytes(&bytes).unwrap();
        assert_eq!(parsed, data);
    }

    #[test]
    fn test_block_token_extensions_byte_len() {
        let data = BlockTokenExtensionsData::new(&[1, 2, 3]).unwrap();
        assert_eq!(data.byte_len(), 7);

        let empty = BlockTokenExtensionsData::new(&[]).unwrap();
        assert_eq!(empty.byte_len(), 1);
    }

    #[test]
    fn test_block_token_extensions_is_blocked() {
        let extensions = [1u16, 2u16, 3u16];
        let data = BlockTokenExtensionsData::new(&extensions).unwrap();
        assert!(data.is_blocked(1));
        assert!(data.is_blocked(2));
        assert!(data.is_blocked(3));
        assert!(!data.is_blocked(4));
    }

    #[test]
    fn test_block_token_extensions_blocked_extensions() {
        let extensions = [1u16, 2u16, 3u16];
        let data = BlockTokenExtensionsData::new(&extensions).unwrap();
        let blocked = data.blocked_extensions();
        assert_eq!(blocked, &[1u16, 2u16, 3u16]);
    }

    #[test]
    fn test_block_token_extensions_add_extension() {
        let mut data = BlockTokenExtensionsData::new(&[]).unwrap();
        assert_eq!(data.count, 0);

        data.add_extension(1u16).unwrap();
        assert_eq!(data.count, 1);
        assert_eq!(data.blocked_extensions[0], 1);

        data.add_extension(2u16).unwrap();
        assert_eq!(data.count, 2);
        assert_eq!(data.blocked_extensions[1], 2);
    }

    #[test]
    fn test_block_token_extensions_add_extension_duplicate() {
        let mut data = BlockTokenExtensionsData::new(&[1u16]).unwrap();
        let result = data.add_extension(1u16);
        assert!(result.is_err());
        assert_eq!(data.count, 1);
    }

    #[test]
    fn test_block_token_extensions_from_bytes_empty_fails() {
        let result = BlockTokenExtensionsData::from_bytes(&[]);
        assert!(result.is_err());
    }

    #[test]
    fn test_block_token_extensions_from_bytes_truncated_fails() {
        let result = BlockTokenExtensionsData::from_bytes(&[3, 0, 0]);
        assert!(result.is_err());
    }
}
