use pinocchio::{account::AccountView, error::ProgramError};

use crate::{
    traits::InstructionAccounts,
    utils::{
        validate_associated_token_account, verify_current_program, verify_current_program_account,
        verify_event_authority, verify_readonly, verify_signer, verify_system_program, verify_token_program,
        verify_writable,
    },
};

/// Accounts for the Withdraw instruction
///
/// # Account Layout
/// 0. `[signer]` payer - Transaction fee payer
/// 1. `[writable]` rent_recipient - Receives rent from closed receipt
/// 2. `[signer]` withdrawer - Must match receipt.depositor
/// 3. `[]` escrow - Escrow PDA (signing authority for vault transfer)
/// 4. `[]` extensions - Extensions PDA (optional, may be system-owned)
/// 5. `[writable]` receipt - Deposit receipt to verify and close
/// 6. `[writable]` vault - Escrow's vault token account (source)
/// 7. `[writable]` withdrawer_token_account - Withdrawer's token account (destination)
/// 8. `[]` mint - Token mint
/// 9. `[]` token_program - SPL Token program
/// 10. `[]` system_program - System program
/// 11. `[]` event_authority - Event authority PDA
/// 12. `[]` escrow_program - Current program
///
/// # Remaining Accounts (if hook configured)
/// 0. `[]` hook_program - The hook program to invoke
/// 1. ..N. `[]` extra accounts - Additional accounts to pass to the hook (all read-only)
pub struct WithdrawAccounts<'a> {
    pub payer: &'a AccountView,
    pub rent_recipient: &'a AccountView,
    pub withdrawer: &'a AccountView,
    pub escrow: &'a AccountView,
    pub extensions: &'a AccountView,
    pub receipt: &'a AccountView,
    pub vault: &'a AccountView,
    pub withdrawer_token_account: &'a AccountView,
    pub mint: &'a AccountView,
    pub token_program: &'a AccountView,
    pub system_program: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub escrow_program: &'a AccountView,
    pub remaining_accounts: &'a [AccountView],
}

impl<'a> TryFrom<&'a [AccountView]> for WithdrawAccounts<'a> {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [payer, rent_recipient, withdrawer, escrow, extensions, receipt, vault, withdrawer_token_account, mint, token_program, system_program, event_authority, escrow_program, remaining_accounts @ ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // 1. Validate signers
        verify_signer(payer, false)?;
        verify_signer(withdrawer, false)?;

        // 2. Validate writable
        verify_writable(rent_recipient, true)?;
        verify_writable(receipt, true)?;
        verify_writable(vault, true)?;
        verify_writable(withdrawer_token_account, true)?;

        // 3. Validate readonly
        verify_readonly(escrow)?;
        verify_readonly(extensions)?;
        verify_readonly(mint)?;

        // 4. Validate program IDs
        verify_token_program(token_program)?;
        verify_system_program(system_program)?;
        verify_current_program(escrow_program)?;
        verify_event_authority(event_authority)?;

        // 5. Validate accounts owned by current program
        verify_current_program_account(escrow)?;
        verify_current_program_account(receipt)?;

        // 6. Validate ATA
        validate_associated_token_account(vault, escrow.address(), mint, token_program)?;
        validate_associated_token_account(withdrawer_token_account, withdrawer.address(), mint, token_program)?;

        Ok(Self {
            payer,
            rent_recipient,
            withdrawer,
            escrow,
            extensions,
            receipt,
            vault,
            withdrawer_token_account,
            mint,
            token_program,
            system_program,
            event_authority,
            escrow_program,
            remaining_accounts,
        })
    }
}

impl<'a> InstructionAccounts<'a> for WithdrawAccounts<'a> {}
