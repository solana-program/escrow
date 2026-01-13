use pinocchio::{account::AccountView, Address, ProgramResult};

use crate::{
    events::BlockMintEvent,
    instructions::BlockMint,
    state::{AllowedMint, AllowedMintPda, Escrow},
    traits::{EventSerialize, PdaSeeds},
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

    // Verify allowed_mint exists and is valid
    let allowed_mint_data = ix.accounts.allowed_mint.try_borrow()?;
    let allowed_mint = AllowedMint::from_account(&allowed_mint_data)?;

    // Validate that allowed_mint PDA matches the escrow + mint combination
    let pda_seeds = AllowedMintPda::new(ix.accounts.escrow.address(), ix.accounts.mint.address());
    pda_seeds.validate_pda(ix.accounts.allowed_mint, program_id, allowed_mint.bump)?;
    drop(allowed_mint_data);

    // Close the AllowedMint account and return lamports to payer
    close_pda_account(ix.accounts.allowed_mint, ix.accounts.payer)?;

    // Emit event via CPI
    let event = BlockMintEvent::new(*ix.accounts.escrow.address(), *ix.accounts.mint.address());
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
