use alloc::vec::Vec;
use pinocchio::{account::AccountView, cpi::Seed, error::ProgramError, Address, ProgramResult};

use crate::ID as ESCROW_PROGRAM_ID;
use crate::{
    state::{EscrowExtensionsHeader, ExtensionType, ExtensionsPda, TimelockData, TLV_HEADER_SIZE},
    traits::{AccountSerialize, PdaSeeds},
    utils::{create_pda_account_idempotent, TlvReader},
};

/// Context for extension validation
pub struct ValidationContext {
    pub deposited_at: i64,
}

/// Appends a TLV extension to the extensions PDA.
///
/// Creates account if new, or resizes if existing (via `create_pda_account_idempotent`).
/// Returns error if the extension type already exists (no duplicates allowed).
pub fn append_extension<const N: usize>(
    payer: &AccountView,
    extensions: &AccountView,
    program_id: &Address,
    bump: u8,
    ext_type: ExtensionType,
    new_tlv: &[u8],
    pda_signer_seeds: [Seed; N],
) -> ProgramResult {
    // 1. Read current data (if any)
    let current_data_len = extensions.data_len();
    let (extension_count, existing_tlv) = if current_data_len > 0 {
        let data = extensions.try_borrow()?;
        let header = EscrowExtensionsHeader::from_bytes(&data)?;

        // Check for duplicate extension
        let reader = TlvReader::new(&data);
        if reader.find_extension(ext_type)?.is_some() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let tlv_slice = data[EscrowExtensionsHeader::LEN..].to_vec();
        (header.extension_count, tlv_slice)
    } else {
        (0, Vec::new())
    };

    // 2. Build new data: existing TLV + new TLV
    let mut all_tlv = existing_tlv;
    all_tlv.extend_from_slice(new_tlv);

    // 3. Calculate required size
    let required_size = EscrowExtensionsHeader::LEN + all_tlv.len();

    // 4. Create/resize account
    create_pda_account_idempotent(payer, required_size, program_id, extensions, pda_signer_seeds)?;

    // 5. Write data
    let mut data = extensions.try_borrow_mut()?;
    let new_header = EscrowExtensionsHeader::new(bump, extension_count + 1);
    let header_bytes = new_header.to_bytes();
    data[..EscrowExtensionsHeader::LEN].copy_from_slice(&header_bytes);
    data[EscrowExtensionsHeader::LEN..required_size].copy_from_slice(&all_tlv);

    Ok(())
}

/// Updates an existing TLV extension entry in the extensions PDA.
///
/// Finds the extension by type, replaces its data, and resizes account if needed.
/// Returns error if the extension type doesn't exist.
pub fn update_extension<const N: usize>(
    payer: &AccountView,
    extensions: &AccountView,
    ext_type: ExtensionType,
    new_tlv: &[u8],
    pda_signer_seeds: [Seed; N],
) -> ProgramResult {
    let current_data_len = extensions.data_len();
    if current_data_len == 0 {
        return Err(ProgramError::UninitializedAccount);
    }

    let data = extensions.try_borrow()?;
    let header = EscrowExtensionsHeader::from_bytes(&data)?;

    // Find the extension entry
    let mut offset = EscrowExtensionsHeader::LEN;
    let mut found_offset = None;
    let mut old_tlv_len = 0;

    while offset + TLV_HEADER_SIZE <= data.len() {
        let type_bytes = u16::from_le_bytes([data[offset], data[offset + 1]]);
        let length = u16::from_le_bytes([data[offset + 2], data[offset + 3]]) as usize;

        if offset + TLV_HEADER_SIZE + length > data.len() {
            break;
        }

        if type_bytes == ext_type as u16 {
            found_offset = Some(offset);
            old_tlv_len = TLV_HEADER_SIZE + length;
            break;
        }

        offset += TLV_HEADER_SIZE + length;
    }

    let found_offset = found_offset.ok_or(ProgramError::UninitializedAccount)?;

    // Build new TLV data: before + new entry + after
    let before = data[EscrowExtensionsHeader::LEN..found_offset].to_vec();
    let after_start = found_offset + old_tlv_len;
    let after = data[after_start..].to_vec();
    let header_bytes = header.to_bytes();

    drop(data);

    let mut new_tlv_data = Vec::new();
    new_tlv_data.extend_from_slice(&before);
    new_tlv_data.extend_from_slice(&(ext_type as u16).to_le_bytes());
    new_tlv_data.extend_from_slice(&(new_tlv.len() as u16).to_le_bytes());
    new_tlv_data.extend_from_slice(new_tlv);
    new_tlv_data.extend_from_slice(&after);

    // Calculate required size
    let required_size = EscrowExtensionsHeader::LEN + new_tlv_data.len();

    // Resize account if needed
    create_pda_account_idempotent(payer, required_size, &ESCROW_PROGRAM_ID, extensions, pda_signer_seeds)?;

    // Write data
    let mut data = extensions.try_borrow_mut()?;
    data[..EscrowExtensionsHeader::LEN].copy_from_slice(&header_bytes);
    data[EscrowExtensionsHeader::LEN..required_size].copy_from_slice(&new_tlv_data);

    Ok(())
}

/// Updates or appends a TLV extension to the extensions PDA.
///
/// Simplifies the common pattern of checking if extension exists and either updating or appending:
/// - If no data exists → creates account with new extension
/// - If data exists but extension not found → appends new extension
/// - If data exists and extension found → updates existing extension
///
/// Note: `ext_data` should be the raw extension data (not TLV-wrapped). This function
/// handles TLV wrapping internally for append operations.
pub fn update_or_append_extension<const N: usize>(
    payer: &AccountView,
    extensions: &AccountView,
    program_id: &Address,
    bump: u8,
    ext_type: ExtensionType,
    ext_data: &[u8],
    pda_signer_seeds: [Seed; N],
) -> ProgramResult {
    // Build TLV-wrapped data for append operations
    let build_tlv = || {
        let mut tlv = Vec::with_capacity(TLV_HEADER_SIZE + ext_data.len());
        tlv.extend_from_slice(&(ext_type as u16).to_le_bytes());
        tlv.extend_from_slice(&(ext_data.len() as u16).to_le_bytes());
        tlv.extend_from_slice(ext_data);
        tlv
    };

    if extensions.data_len() == 0 {
        let tlv = build_tlv();
        append_extension(payer, extensions, program_id, bump, ext_type, &tlv, pda_signer_seeds)
    } else {
        let data = extensions.try_borrow()?;
        let reader = TlvReader::new(&data);
        let extension_exists = reader.find_extension(ext_type)?.is_some();
        drop(data);

        if extension_exists {
            update_extension(payer, extensions, ext_type, ext_data, pda_signer_seeds)
        } else {
            let tlv = build_tlv();
            append_extension(payer, extensions, program_id, bump, ext_type, &tlv, pda_signer_seeds)
        }
    }
}

/// Validates specified extensions if initialized.
///
/// Always validates that extensions account is the correct PDA for the escrow.
/// Then checks if data exists to determine if extensions are initialized.
/// Iterates through TLV entries and validates only the extension types specified in `to_validate`.
pub fn validate_extensions(
    escrow: &AccountView,
    extensions: &AccountView,
    program_id: &Address,
    to_validate: &[ExtensionType],
    ctx: &ValidationContext,
) -> ProgramResult {
    // Validate this is the correct extensions PDA for this escrow
    let extensions_pda = ExtensionsPda::new(escrow.address());
    let expected_bump = extensions_pda.validate_pda_address(extensions, program_id)?;

    // No data = extensions not initialized, nothing to check
    if extensions.data_len() == 0 {
        return Ok(());
    }

    // Extensions exist - parse and validate
    let data = extensions.try_borrow()?;
    let header = EscrowExtensionsHeader::from_bytes(&data)?;

    // Verify stored bump matches canonical bump
    if header.bump != expected_bump {
        return Err(ProgramError::InvalidSeeds);
    }

    // Iterate through TLV entries
    let mut offset = EscrowExtensionsHeader::LEN;
    while offset.checked_add(TLV_HEADER_SIZE).ok_or(ProgramError::ArithmeticOverflow)? <= data.len() {
        let ext_type_raw = u16::from_le_bytes([data[offset], data[offset + 1]]);
        let ext_len = u16::from_le_bytes([data[offset + 2], data[offset + 3]]) as usize;

        let data_end = offset
            .checked_add(TLV_HEADER_SIZE)
            .and_then(|v| v.checked_add(ext_len))
            .ok_or(ProgramError::ArithmeticOverflow)?;

        if data_end > data.len() {
            break;
        }

        let ext_data = &data[offset + TLV_HEADER_SIZE..data_end];
        let ext_type = ExtensionType::try_from(ext_type_raw)?;

        // Dispatch to extension's validate method if in to_validate list
        if to_validate.contains(&ext_type) {
            match ext_type {
                ExtensionType::Timelock => TimelockData::validate(ext_data, ctx)?,
                ExtensionType::Hook => {}
                ExtensionType::BlockedTokenExtensions => {}
            }
        }

        offset = data_end;
    }

    Ok(())
}
