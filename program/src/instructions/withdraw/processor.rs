use pinocchio::{account::AccountView, Address, ProgramResult};
use pinocchio_token_2022::instructions::TransferChecked;

use crate::{
    events::WithdrawEvent,
    instructions::Withdraw,
    state::{
        get_extensions_from_account, validate_extensions_pda, Escrow, ExtensionType, HookData, HookPoint, Receipt,
        TimelockData,
    },
    traits::{AccountDeserialize, EventSerialize, ExtensionData},
    utils::{close_pda_account, emit_event, get_mint_decimals},
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

    // Validate extensions PDA
    validate_extensions_pda(ix.accounts.escrow, ix.accounts.extensions, program_id)?;

    // Get timelock and hook extensions in single pass
    let exts = get_extensions_from_account(ix.accounts.extensions, &[ExtensionType::Timelock, ExtensionType::Hook])?;

    // Validate timelock if present
    if let Some(ref timelock_bytes) = exts[0] {
        let timelock = TimelockData::from_bytes(timelock_bytes)?;
        timelock.validate(deposited_at)?;
    }

    // Parse hook if present
    let hook_data = exts[1].as_ref().map(|b| HookData::from_bytes(b)).transpose()?;

    // Invoke pre-withdraw hook if configured
    if let Some(ref hook) = hook_data {
        hook.invoke(
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
    if let Some(ref hook) = hook_data {
        hook.invoke(
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
