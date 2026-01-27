use pinocchio::{account::AccountView, Address, ProgramResult};
use pinocchio_token_2022::instructions::TransferChecked;

use crate::{
    events::WithdrawEvent,
    instructions::Withdraw,
    state::{Escrow, ExtensionType, HookPoint, Receipt},
    traits::{AccountDeserialize, EventSerialize},
    utils::{
        close_pda_account, emit_event, get_and_validate_hook, get_mint_decimals, invoke_hook, validate_extensions,
        ValidationContext,
    },
};

/// Processes the Withdraw instruction.
///
/// Transfers tokens from escrow vault back to the withdrawer and closes the receipt PDA.
pub fn process_withdraw(program_id: &Address, accounts: &[AccountView], instruction_data: &[u8]) -> ProgramResult {
    let ix = Withdraw::try_from((instruction_data, accounts))?;

    // Validate escrow PDA
    {
        let escrow_data = ix.accounts.escrow.try_borrow()?;
        let _escrow = Escrow::from_account(&escrow_data, ix.accounts.escrow, program_id)?;
    }

    // Read and validate receipt
    let (amount, receipt_seed, mint, deposited_at) = {
        let receipt_data = ix.accounts.receipt.try_borrow()?;
        let receipt = Receipt::from_account(&receipt_data, ix.accounts.receipt, program_id)?;

        // Verify withdrawer matches the original depositor and the receipt is for this escrow
        receipt.validate_depositor(ix.accounts.escrow.address(), ix.accounts.withdrawer.address())?;

        (receipt.amount, receipt.receipt_seed, receipt.mint, receipt.deposited_at)
    };

    // Validate extensions
    let ctx = ValidationContext { deposited_at };
    validate_extensions(ix.accounts.escrow, ix.accounts.extensions, program_id, &[ExtensionType::Timelock], &ctx)?;

    // Check once if hook is configured
    let hook_data = get_and_validate_hook(ix.accounts.extensions, ix.accounts.remaining_accounts)?;

    // Invoke pre-withdraw hook if configured
    if let Some(hook_data) = hook_data {
        invoke_hook(
            &hook_data,
            HookPoint::PreWithdraw,
            ix.accounts.remaining_accounts,
            &[ix.accounts.escrow, ix.accounts.withdrawer, ix.accounts.mint, ix.accounts.receipt],
        )?;
    }

    // Transfer tokens from vault to withdrawer using escrow PDA as signer
    let decimals = get_mint_decimals(ix.accounts.mint)?;

    {
        let escrow_data = ix.accounts.escrow.try_borrow()?;
        let escrow = Escrow::from_bytes(&escrow_data)?;
        escrow.with_signer(|signers| {
            TransferChecked {
                from: ix.accounts.vault,
                mint: ix.accounts.mint,
                to: ix.accounts.withdrawer_token_account,
                authority: ix.accounts.escrow,
                amount,
                decimals,
                token_program: ix.accounts.token_program.address(),
            }
            .invoke_signed(signers)
        })?;
    }

    // Close receipt account and return lamports to rent_recipient
    close_pda_account(ix.accounts.receipt, ix.accounts.rent_recipient)?;

    // Invoke post-withdraw hook if configured (receipt is closed, don't pass it)
    if let Some(hook_data) = hook_data {
        invoke_hook(
            &hook_data,
            HookPoint::PostWithdraw,
            ix.accounts.remaining_accounts,
            &[ix.accounts.escrow, ix.accounts.withdrawer, ix.accounts.mint],
        )?;
    }

    // Emit event
    let event = WithdrawEvent::new(
        *ix.accounts.escrow.address(),
        *ix.accounts.withdrawer.address(),
        mint,
        receipt_seed,
        amount,
    );
    emit_event(program_id, ix.accounts.event_authority, ix.accounts.escrow_program, &event.to_bytes())?;

    Ok(())
}
