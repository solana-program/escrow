use alloc::vec::Vec;
use pinocchio::{
    account::AccountView,
    cpi::Seed,
    error::ProgramError,
    sysvars::{clock::Clock, Sysvar},
    Address, ProgramResult,
};
use pinocchio_token_2022::instructions::TransferChecked;

use crate::{
    errors::EscrowProgramError,
    events::DepositEvent,
    instructions::Deposit,
    state::{AllowedMint, AllowedMintPda, Escrow, HookPoint, Receipt},
    traits::{AccountSerialize, AccountSize, EventSerialize, PdaSeeds},
    utils::{create_pda_account, emit_event, get_and_validate_hook, get_mint_decimals, invoke_hook},
};

/// Processes the Deposit instruction.
///
/// Transfers tokens from depositor to escrow vault and creates a receipt PDA.
pub fn process_deposit(program_id: &Address, accounts: &[AccountView], instruction_data: &[u8]) -> ProgramResult {
    let ix = Deposit::try_from((instruction_data, accounts))?;

    // Verify escrow exists and is valid
    let escrow_data = ix.accounts.escrow.try_borrow()?;
    let _escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;

    // Verify allowed_mint PDA exists and matches expected derivation
    let allowed_mint_data = ix.accounts.allowed_mint.try_borrow()?;
    let allowed_mint = AllowedMint::from_account(&allowed_mint_data).map_err(|_| EscrowProgramError::MintNotAllowed)?;

    // Validate that the allowed_mint PDA is derived from the correct escrow + mint
    let pda_seeds = AllowedMintPda::new(ix.accounts.escrow.address(), ix.accounts.mint.address());
    pda_seeds
        .validate_pda(ix.accounts.allowed_mint, program_id, allowed_mint.bump)
        .map_err(|_| EscrowProgramError::MintNotAllowed)?;

    // Get current timestamp from Clock sysvar
    let clock = Clock::get()?;
    let deposited_at = clock.unix_timestamp;

    // Create Receipt state
    let receipt = Receipt::new(
        ix.data.amount,
        deposited_at,
        *ix.accounts.escrow.address(),
        *ix.accounts.depositor.address(),
        *ix.accounts.mint.address(),
        *ix.accounts.receipt_seed.address(),
        ix.data.bump,
    );

    // Validate deposit receipt PDA
    receipt.validate_pda(ix.accounts.receipt, program_id, ix.data.bump)?;

    // Get seeds for receipt account creation
    let receipt_bump_seed = [ix.data.bump];
    let receipt_seeds: Vec<Seed> = receipt.seeds_with_bump(&receipt_bump_seed);
    let receipt_seeds_array: [Seed; 6] = receipt_seeds.try_into().map_err(|_| ProgramError::InvalidArgument)?;

    // Create the deposit receipt PDA
    create_pda_account(ix.accounts.payer, Receipt::LEN, program_id, ix.accounts.receipt, receipt_seeds_array)?;

    // Write serialized receipt data to the account
    let mut receipt_data_slice = ix.accounts.receipt.try_borrow_mut()?;
    receipt.write_to_slice(&mut receipt_data_slice)?;
    drop(receipt_data_slice);

    // Check once if hook is configured
    let hook_data = get_and_validate_hook(ix.accounts.extensions, ix.accounts.remaining_accounts)?;

    // Invoke pre-deposit hook if configured
    if let Some(hook_data) = hook_data {
        invoke_hook(
            &hook_data,
            HookPoint::PreDeposit,
            ix.accounts.remaining_accounts,
            &[ix.accounts.escrow, ix.accounts.depositor, ix.accounts.mint, ix.accounts.receipt],
        )?;
    }

    // Transfer tokens from depositor to vault
    let decimals = get_mint_decimals(ix.accounts.mint)?;

    TransferChecked {
        from: ix.accounts.depositor_token_account,
        mint: ix.accounts.mint,
        to: ix.accounts.vault,
        authority: ix.accounts.depositor,
        amount: ix.data.amount,
        decimals,
        token_program: ix.accounts.token_program.address(),
    }
    .invoke()?;

    // Invoke post-deposit hook if configured
    if let Some(hook_data) = hook_data {
        invoke_hook(
            &hook_data,
            HookPoint::PostDeposit,
            ix.accounts.remaining_accounts,
            &[ix.accounts.escrow, ix.accounts.depositor, ix.accounts.mint, ix.accounts.receipt],
        )?;
    }

    // Emit event via CPI
    let event = DepositEvent::new(
        *ix.accounts.escrow.address(),
        *ix.accounts.depositor.address(),
        *ix.accounts.mint.address(),
        *ix.accounts.receipt_seed.address(),
        ix.data.amount,
    );
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
