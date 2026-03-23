use alloc::vec::Vec;
use pinocchio::{account::AccountView, cpi::Seed, error::ProgramError, Address, ProgramResult};

use crate::{
    errors::EscrowProgramError,
    events::TokenExtensionUnblocked,
    instructions::UnblockTokenExtension,
    state::{remove_extension, update_extension, Escrow, ExtensionType, ExtensionsPda},
    traits::{EventSerialize, ExtensionData, PdaSeeds},
    utils::{emit_event, TlvReader},
};

/// Processes the UnblockTokenExtension instruction.
///
/// Removes a single blocked token extension value from the escrow's blocked list.
/// If the list becomes empty, removes the entire BlockedTokenExtensions TLV entry.
pub fn process_unblock_token_extension(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = UnblockTokenExtension::try_from((instruction_data, accounts))?;

    // Read escrow and validate
    let escrow_data = ix.accounts.escrow.try_borrow()?;
    let escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;
    escrow.validate_admin(ix.accounts.admin.address())?;
    escrow.require_mutable()?;

    // Validate extensions PDA
    let extensions_pda = ExtensionsPda::new(ix.accounts.escrow.address());
    extensions_pda.validate_pda(ix.accounts.extensions, program_id, ix.data.extensions_bump)?;

    // Get seeds for PDA operations
    let extensions_bump_seed = [ix.data.extensions_bump];
    let extensions_seeds: Vec<Seed> = extensions_pda.seeds_with_bump(&extensions_bump_seed);
    let extensions_seeds_array: [Seed; 3] = extensions_seeds.try_into().map_err(|_| ProgramError::InvalidArgument)?;

    // Read existing BlockedTokenExtensions data if present
    let mut blocked_token_extensions = {
        if ix.accounts.extensions.data_len() == 0 {
            return Err(EscrowProgramError::TokenExtensionNotBlocked.into());
        }

        let data = ix.accounts.extensions.try_borrow()?;
        let reader = TlvReader::new(&data);
        reader.read_blocked_token_extensions().ok_or(EscrowProgramError::TokenExtensionNotBlocked)?
    };

    blocked_token_extensions.remove_extension(ix.data.blocked_extension)?;

    if blocked_token_extensions.count == 0 {
        remove_extension(ix.accounts.extensions, ExtensionType::BlockedTokenExtensions)?;
    } else {
        update_extension(
            ix.accounts.payer,
            ix.accounts.extensions,
            ExtensionType::BlockedTokenExtensions,
            &blocked_token_extensions.to_bytes(),
            extensions_seeds_array,
        )?;
    }

    // Emit event
    let event = TokenExtensionUnblocked::new(*ix.accounts.escrow.address(), ix.data.blocked_extension);
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
