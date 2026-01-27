use alloc::vec::Vec;
use pinocchio::error::ProgramError;

use crate::{
    state::{BlockTokenExtensionsData, EscrowExtensionsHeader, ExtensionType, HookData, TimelockData, TLV_HEADER_SIZE},
    traits::ExtensionData,
};

/// Helper to read TLV extensions from account data
pub struct TlvReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> TlvReader<'a> {
    /// Create a reader starting after the header
    pub fn new(account_data: &'a [u8]) -> Self {
        Self { data: account_data, offset: EscrowExtensionsHeader::LEN }
    }

    /// Find and read a specific extension type
    pub fn find_extension(&self, ext_type: ExtensionType) -> Result<Option<&'a [u8]>, ProgramError> {
        let mut offset = self.offset;
        while offset + TLV_HEADER_SIZE <= self.data.len() {
            let type_bytes = u16::from_le_bytes(
                self.data[offset..offset + 2].try_into().map_err(|_| ProgramError::InvalidAccountData)?,
            );
            let length = u16::from_le_bytes(
                self.data[offset + 2..offset + 4].try_into().map_err(|_| ProgramError::InvalidAccountData)?,
            ) as usize;

            if offset + TLV_HEADER_SIZE + length > self.data.len() {
                break;
            }

            if type_bytes == ext_type as u16 {
                return Ok(Some(&self.data[offset + TLV_HEADER_SIZE..offset + TLV_HEADER_SIZE + length]));
            }

            offset += TLV_HEADER_SIZE + length;
        }
        Ok(None)
    }

    /// Read timelock extension if present
    pub fn read_timelock(&self) -> Option<TimelockData> {
        self.find_extension(ExtensionType::Timelock).ok().flatten().and_then(|data| TimelockData::from_bytes(data).ok())
    }

    /// Read hook extension if present
    pub fn read_hook(&self) -> Option<HookData> {
        self.find_extension(ExtensionType::Hook).ok().flatten().and_then(|data| HookData::from_bytes(data).ok())
    }

    /// Read blocked token extensions if present
    pub fn read_blocked_token_extensions(&self) -> Option<BlockTokenExtensionsData> {
        self.find_extension(ExtensionType::BlockedTokenExtensions)
            .ok()
            .flatten()
            .and_then(|data| BlockTokenExtensionsData::from_bytes(data).ok())
    }
}

/// Helper to write TLV extensions to account data
pub struct TlvWriter {
    data: Vec<u8>,
}

impl TlvWriter {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Write a TLV entry
    pub fn write_extension(&mut self, ext_type: ExtensionType, data: &[u8]) {
        self.data.extend_from_slice(&(ext_type as u16).to_le_bytes());
        self.data.extend_from_slice(&(data.len() as u16).to_le_bytes());
        self.data.extend_from_slice(data);
    }

    /// Write timelock extension
    pub fn write_timelock(&mut self, timelock: &TimelockData) {
        self.write_extension(ExtensionType::Timelock, &timelock.to_bytes());
    }

    /// Write hook extension
    pub fn write_hook(&mut self, hook: &HookData) {
        self.write_extension(ExtensionType::Hook, &hook.to_bytes());
    }

    /// Write blocked token extensions
    pub fn write_block_token_extensions(&mut self, block_token_extensions: &BlockTokenExtensionsData) {
        self.write_extension(ExtensionType::BlockedTokenExtensions, &block_token_extensions.to_bytes());
    }

    /// Get the total TLV data
    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    /// Get current length
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl Default for TlvWriter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::EscrowExtensionsHeader;
    use crate::traits::AccountSerialize;
    use pinocchio::Address;

    #[test]
    fn test_tlv_writer_timelock() {
        let mut writer = TlvWriter::new();
        let timelock = TimelockData::new(3600);
        writer.write_timelock(&timelock);

        let bytes = writer.into_bytes();
        assert_eq!(bytes.len(), TLV_HEADER_SIZE + TimelockData::LEN);

        // Check type
        let ext_type = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        assert_eq!(ext_type, ExtensionType::Timelock as u16);

        // Check length
        let length = u16::from_le_bytes(bytes[2..4].try_into().unwrap());
        assert_eq!(length as usize, TimelockData::LEN);
    }

    #[test]
    fn test_tlv_reader_find_timelock() {
        let header = EscrowExtensionsHeader::new(255, 1);

        let mut writer = TlvWriter::new();
        let timelock = TimelockData::new(7200);
        writer.write_timelock(&timelock);

        let mut account_data = header.to_bytes();
        account_data.extend_from_slice(&writer.into_bytes());

        // Read it back
        let reader = TlvReader::new(&account_data);
        let read_timelock = reader.read_timelock().unwrap();
        assert_eq!(read_timelock.lock_duration, 7200);
    }

    #[test]
    fn test_tlv_writer_hook() {
        let mut writer = TlvWriter::new();
        let hook_program = Address::new_from_array([42u8; 32]);
        let hook = HookData::new(hook_program);
        writer.write_hook(&hook);

        let bytes = writer.into_bytes();
        assert_eq!(bytes.len(), TLV_HEADER_SIZE + HookData::LEN);

        // Check type
        let ext_type = u16::from_le_bytes(bytes[0..2].try_into().unwrap());
        assert_eq!(ext_type, ExtensionType::Hook as u16);

        // Check length
        let length = u16::from_le_bytes(bytes[2..4].try_into().unwrap());
        assert_eq!(length as usize, HookData::LEN);
    }

    #[test]
    fn test_tlv_reader_find_hook() {
        let header = EscrowExtensionsHeader::new(255, 1);

        let mut writer = TlvWriter::new();
        let hook_program = Address::new_from_array([99u8; 32]);
        let hook = HookData::new(hook_program);
        writer.write_hook(&hook);

        let mut account_data = header.to_bytes();
        account_data.extend_from_slice(&writer.into_bytes());

        // Read it back
        let reader = TlvReader::new(&account_data);
        let read_hook = reader.read_hook().unwrap();
        assert_eq!(read_hook.hook_program, hook_program);
    }

    #[test]
    fn test_tlv_reader_multiple_extensions() {
        let header = EscrowExtensionsHeader::new(255, 2);

        // Write both timelock and hook
        let mut writer = TlvWriter::new();
        let timelock = TimelockData::new(3600);
        writer.write_timelock(&timelock);

        let hook_program = Address::new_from_array([77u8; 32]);
        let hook = HookData::new(hook_program);
        writer.write_hook(&hook);

        let mut account_data = header.to_bytes();
        account_data.extend_from_slice(&writer.into_bytes());

        // Read both back - should find correct ones
        let reader = TlvReader::new(&account_data);

        let read_timelock = reader.read_timelock().unwrap();
        assert_eq!(read_timelock.lock_duration, 3600);

        let read_hook = reader.read_hook().unwrap();
        assert_eq!(read_hook.hook_program, hook_program);
    }

    #[test]
    fn test_tlv_writer_is_empty() {
        let writer = TlvWriter::new();
        assert!(writer.is_empty());
        assert_eq!(writer.len(), 0);

        let mut writer = TlvWriter::new();
        let timelock = TimelockData::new(100);
        writer.write_timelock(&timelock);
        assert!(!writer.is_empty());
        assert_eq!(writer.len(), TLV_HEADER_SIZE + TimelockData::LEN);
    }
}
