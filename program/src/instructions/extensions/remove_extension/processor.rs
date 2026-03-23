use pinocchio::{account::AccountView, Address, ProgramResult};

use crate::{
    events::ExtensionRemovedEvent,
    instructions::RemoveExtension,
    state::{remove_extension, Escrow, ExtensionType, ExtensionsPda},
    traits::{EventSerialize, PdaSeeds},
    utils::emit_event,
};

/// Processes the RemoveExtension instruction.
///
/// Removes an existing extension entry from the escrow extensions account.
pub fn process_remove_extension(
    program_id: &Address,
    accounts: &[AccountView],
    instruction_data: &[u8],
) -> ProgramResult {
    let ix = RemoveExtension::try_from((instruction_data, accounts))?;

    // Read escrow and validate
    let escrow_data = ix.accounts.escrow.try_borrow()?;
    let escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;
    escrow.validate_admin(ix.accounts.admin.address())?;

    // Validate extensions PDA
    let extensions_pda = ExtensionsPda::new(ix.accounts.escrow.address());
    extensions_pda.validate_pda(ix.accounts.extensions, program_id, ix.data.extensions_bump)?;

    // Parse extension type and remove matching extension
    let extension_type = ExtensionType::try_from(ix.data.extension_type)?;
    remove_extension(ix.accounts.extensions, extension_type)?;

    // Emit event
    let event = ExtensionRemovedEvent::new(*ix.accounts.escrow.address(), ix.data.extension_type);
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
