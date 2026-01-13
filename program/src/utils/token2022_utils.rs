//! Token2022 extension validation utilities.

use alloc::vec::Vec;
use pinocchio::{account::AccountView, error::ProgramError};
use pinocchio_token_2022::ID as TOKEN_2022_PROGRAM_ID;
use spl_token_2022::{
    extension::{BaseStateWithExtensions, ExtensionType, StateWithExtensions},
    state::Mint,
};

use crate::{errors::EscrowProgramError, utils::TlvReader};

/// Validates that a Token2022 mint does not have any dangerous extensions.
///
/// This function only checks mints owned by the Token-2022 program.
/// Regular SPL Token mints are allowed without extension checks.
///
/// # Blocked Extensions (Global)
/// - `PermanentDelegate`: Authority can transfer/burn tokens from ANY account
/// - `NonTransferable`: Tokens cannot be transferred
/// - `Pausable`: Authority can pause all transfers
///
/// # Escrow-Specific Blocklist
/// If `extensions` account is provided and contains a `BlockTokenExtensions` extension,
/// those extension types are also checked (union with global blocklist).
///
/// # Arguments
/// * `mint` - The mint account to validate
/// * `extensions` - Optional extensions account to check for escrow-specific blocklist
///
/// # Returns
/// * `Ok(())` if the mint is safe to use
/// * `Err(EscrowProgramError::*)` if the mint has a blocked extension
#[inline(always)]
pub fn validate_mint_extensions(mint: &AccountView, extensions: &AccountView) -> Result<(), ProgramError> {
    if !mint.owned_by(&TOKEN_2022_PROGRAM_ID) {
        return Ok(());
    }

    let mint_data = mint.try_borrow()?;

    // Parse the mint with extensions
    let mint_state = StateWithExtensions::<Mint>::unpack(&mint_data)?;

    // Get all extension types present on this mint
    let extension_types = mint_state.get_extension_types()?;

    // Build combined blocklist: global + escrow-specific (as u16 values)
    let mut blocked_types_u16 = Vec::new();

    // Add global blocklist (convert ExtensionType enum to u16)
    blocked_types_u16.push(ExtensionType::PermanentDelegate as u16);
    blocked_types_u16.push(ExtensionType::NonTransferable as u16);
    blocked_types_u16.push(ExtensionType::Pausable as u16);

    // Add escrow-specific blocklist if extensions account has data
    if extensions.data_len() > 0 {
        let extensions_data = extensions.try_borrow()?;
        let reader = TlvReader::new(&extensions_data);
        if let Some(blocked_token_extensions) = reader.read_blocked_token_extensions() {
            for &ext_type_u16 in blocked_token_extensions.blocked_extensions() {
                blocked_types_u16.push(ext_type_u16);
            }
        }
    }

    // Check each extension type against combined blocklist
    for ext_type in extension_types {
        let ext_type_u16 = ext_type as u16;
        if blocked_types_u16.contains(&ext_type_u16) {
            match ext_type {
                ExtensionType::PermanentDelegate => {
                    return Err(EscrowProgramError::PermanentDelegateNotAllowed.into());
                }
                ExtensionType::NonTransferable => {
                    return Err(EscrowProgramError::NonTransferableNotAllowed.into());
                }
                ExtensionType::Pausable => {
                    return Err(EscrowProgramError::PausableNotAllowed.into());
                }
                _ => {
                    // Escrow-specific blocked extension
                    return Err(EscrowProgramError::MintNotAllowed.into());
                }
            }
        }
    }

    Ok(())
}
