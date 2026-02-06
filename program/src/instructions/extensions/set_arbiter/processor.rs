use alloc::vec::Vec;
use pinocchio::{account::AccountView, cpi::Seed, error::ProgramError, Address, ProgramResult};

use crate::{
    events::ArbiterSetEvent,
    instructions::SetArbiter,
    state::{append_extension, ArbiterData, Escrow, ExtensionType, ExtensionsPda},
    traits::{EventSerialize, PdaSeeds},
    utils::{emit_event, TlvWriter},
};

/// Processes the SetArbiter instruction.
///
/// Sets the arbiter on an escrow. Creates extensions PDA if it doesn't exist.
/// The arbiter is immutable — this instruction will fail if an arbiter is already set.
pub fn process_set_arbiter(program_id: &Address, accounts: &[AccountView], instruction_data: &[u8]) -> ProgramResult {
    let ix = SetArbiter::try_from((instruction_data, accounts))?;

    // Reject zero-address arbiter — would make withdrawals permanently impossible
    if ix.data.arbiter == Address::default() {
        return Err(ProgramError::InvalidArgument.into());
    }

    // Read escrow and validate
    let escrow_data = ix.accounts.escrow.try_borrow()?;
    let escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;
    escrow.validate_admin(ix.accounts.admin.address())?;

    // Validate extensions PDA
    let extensions_pda = ExtensionsPda::new(ix.accounts.escrow.address());
    extensions_pda.validate_pda(ix.accounts.extensions, program_id, ix.data.extensions_bump)?;

    // Build TLV data
    let arbiter = ArbiterData::new(ix.data.arbiter);
    let mut tlv_writer = TlvWriter::new();
    tlv_writer.write_arbiter(&arbiter);

    // Get seeds and append extension (fails if arbiter already exists, enforcing immutability)
    let extensions_bump_seed = [ix.data.extensions_bump];
    let extensions_seeds: Vec<Seed> = extensions_pda.seeds_with_bump(&extensions_bump_seed);
    let extensions_seeds_array: [Seed; 3] = extensions_seeds.try_into().map_err(|_| ProgramError::InvalidArgument)?;

    append_extension(
        ix.accounts.payer,
        ix.accounts.extensions,
        program_id,
        ix.data.extensions_bump,
        ExtensionType::Arbiter,
        &tlv_writer.into_bytes(),
        extensions_seeds_array,
    )?;

    // Emit event
    let event = ArbiterSetEvent::new(*ix.accounts.escrow.address(), ix.data.arbiter);
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
