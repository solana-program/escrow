use alloc::vec::Vec;
use pinocchio::{account::AccountView, cpi::Seed, error::ProgramError, Address, ProgramResult};

use crate::{
    events::TimelockAddedEvent,
    instructions::AddTimelock,
    state::{Escrow, ExtensionType, ExtensionsPda, TimelockData},
    traits::{EventSerialize, PdaSeeds},
    utils::{append_extension, emit_event, TlvWriter},
};

/// Processes the AddTimelock instruction.
///
/// Adds a timelock extension to an escrow. Creates extensions PDA if it doesn't exist.
pub fn process_add_timelock(program_id: &Address, accounts: &[AccountView], instruction_data: &[u8]) -> ProgramResult {
    let ix = AddTimelock::try_from((instruction_data, accounts))?;

    // Read escrow and validate
    let escrow_data = ix.accounts.escrow.try_borrow()?;
    let escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;
    escrow.validate_admin(ix.accounts.admin.address())?;

    // Validate extensions PDA
    let extensions_pda = ExtensionsPda::new(ix.accounts.escrow.address());
    extensions_pda.validate_pda(ix.accounts.extensions, program_id, ix.data.extensions_bump)?;

    // Build TLV data
    let timelock = TimelockData::new(ix.data.lock_duration);
    let mut tlv_writer = TlvWriter::new();
    tlv_writer.write_timelock(&timelock);

    // Get seeds and append extension
    let extensions_bump_seed = [ix.data.extensions_bump];
    let extensions_seeds: Vec<Seed> = extensions_pda.seeds_with_bump(&extensions_bump_seed);
    let extensions_seeds_array: [Seed; 3] = extensions_seeds.try_into().map_err(|_| ProgramError::InvalidArgument)?;

    append_extension(
        ix.accounts.payer,
        ix.accounts.extensions,
        program_id,
        ix.data.extensions_bump,
        ExtensionType::Timelock,
        &tlv_writer.into_bytes(),
        extensions_seeds_array,
    )?;

    // Emit event
    let event = TimelockAddedEvent::new(*ix.accounts.escrow.address(), ix.data.lock_duration);
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
