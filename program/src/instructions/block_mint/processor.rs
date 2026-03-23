use pinocchio::{account::AccountView, Address, ProgramResult};

use crate::{
    events::BlockMintEvent,
    instructions::BlockMint,
    state::{AllowedMint, Escrow},
    traits::EventSerialize,
    utils::{close_pda_account, emit_event},
};

/// Processes the BlockMint instruction.
///
/// Closes the AllowedMint PDA, blocking future deposits of that mint.
pub fn process_block_mint(program_id: &Address, accounts: &[AccountView], instruction_data: &[u8]) -> ProgramResult {
    let ix = BlockMint::try_from((instruction_data, accounts))?;

    // Verify escrow exists and validate admin
    let escrow_data = ix.accounts.escrow.try_borrow()?;
    let escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;
    escrow.validate_admin(ix.accounts.admin.address())?;
    escrow.require_mutable()?;

    // Verify allowed_mint account exists and self-validates against escrow + mint PDA derivation
    let allowed_mint_data = ix.accounts.allowed_mint.try_borrow()?;
    let _allowed_mint = AllowedMint::from_account(
        &allowed_mint_data,
        ix.accounts.allowed_mint,
        program_id,
        ix.accounts.escrow.address(),
        ix.accounts.mint.address(),
    )?;
    drop(allowed_mint_data);

    // Close the AllowedMint account and return lamports to rent_recipient
    close_pda_account(ix.accounts.allowed_mint, ix.accounts.rent_recipient)?;

    // Emit event via CPI
    let event = BlockMintEvent::new(*ix.accounts.escrow.address(), *ix.accounts.mint.address());
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
