use alloc::vec::Vec;
use pinocchio::{account::AccountView, cpi::Seed, error::ProgramError, Address, ProgramResult};

use crate::{
    events::TokenExtensionBlocked,
    instructions::BlockTokenExtension,
    state::{BlockTokenExtensionsData, Escrow, ExtensionType, ExtensionsPda},
    traits::{EventSerialize, PdaSeeds},
    utils::{emit_event, update_or_append_extension, TlvReader},
};

/// Processes the BlockTokenExtension instruction.
///
/// Blocks a single token extension for an escrow. Creates extensions PDA if it doesn't exist.
pub fn process_block_token_extension(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = BlockTokenExtension::try_from((instruction_data, accounts))?;

    // Read escrow and validate
    let escrow_data = ix.accounts.escrow.try_borrow()?;
    let escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;
    escrow.validate_admin(ix.accounts.admin.address())?;

    // Validate extensions PDA
    let extensions_pda = ExtensionsPda::new(ix.accounts.escrow.address());
    extensions_pda.validate_pda(ix.accounts.extensions, program_id, ix.data.extensions_bump)?;

    // Get seeds for PDA operations
    let extensions_bump_seed = [ix.data.extensions_bump];
    let extensions_seeds: Vec<Seed> = extensions_pda.seeds_with_bump(&extensions_bump_seed);
    let extensions_seeds_array: [Seed; 3] = extensions_seeds.try_into().map_err(|_| ProgramError::InvalidArgument)?;

    // Read existing BlockedTokenExtensions data if it exists
    // Scoped to ensure borrow is released before calling update_or_append_extension
    let mut blocked_token_extensions = {
        if ix.accounts.extensions.data_len() > 0 {
            let data = ix.accounts.extensions.try_borrow()?;
            let reader = TlvReader::new(&data);
            match reader.read_blocked_token_extensions() {
                Some(data) => data,
                None => BlockTokenExtensionsData::new(&[])?,
            }
        } else {
            BlockTokenExtensionsData::new(&[])?
        }
    };

    // Add the new extension (checks for duplicates)
    blocked_token_extensions.add_extension(ix.data.blocked_extension)?;

    // Serialize and update or append the extension
    update_or_append_extension(
        ix.accounts.payer,
        ix.accounts.extensions,
        program_id,
        ix.data.extensions_bump,
        ExtensionType::BlockedTokenExtensions,
        &blocked_token_extensions.to_bytes(),
        extensions_seeds_array,
    )?;

    // Emit event
    let event = TokenExtensionBlocked::new(*ix.accounts.escrow.address(), ix.data.blocked_extension);
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
