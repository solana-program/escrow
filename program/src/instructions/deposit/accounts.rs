use pinocchio::{account::AccountView, error::ProgramError};

use crate::{
    traits::InstructionAccounts,
    utils::{
        validate_associated_token_account, verify_current_program, verify_current_program_account,
        verify_event_authority, verify_readonly, verify_signer, verify_system_program, verify_token_program,
        verify_writable,
    },
};

/// Accounts for the Deposit instruction
///
/// # Account Layout
/// 0. `[signer, writable]` payer - Pays for account creation
/// 1. `[signer]` depositor - Token authority for the deposit
/// 2. `[]` escrow - Escrow account (must exist)
/// 3. `[]` allowed_mint - AllowedMint PDA `[b"allowed_mint", escrow, mint]` (validates mint is allowed)
/// 4. `[signer]` receipt_seed - Receipt seed signer for PDA uniqueness
/// 5. `[writable]` receipt - Deposit receipt PDA to be created
/// 6. `[writable]` vault - Escrow's vault token account (destination)
/// 7. `[writable]` depositor_token_account - Depositor's token account (source)
/// 8. `[]` mint - Token mint
/// 9. `[]` token_program - SPL Token program
/// 10. `[]` system_program - System program for account creation
/// 11. `[]` event_authority - Event authority PDA
/// 12. `[]` escrow_program - Current program
/// 13. `[]` extensions - Extensions PDA (may be empty/uninitialized)
///
/// # Remaining Accounts (if hook configured)
/// 0. `[]` hook_program - The hook program to invoke
/// 1. ..N. `[]` extra accounts - Additional accounts to pass to the hook (all read-only)
pub struct DepositAccounts<'a> {
    pub payer: &'a AccountView,
    pub depositor: &'a AccountView,
    pub escrow: &'a AccountView,
    pub allowed_mint: &'a AccountView,
    pub receipt_seed: &'a AccountView,
    pub receipt: &'a AccountView,
    pub vault: &'a AccountView,
    pub depositor_token_account: &'a AccountView,
    pub mint: &'a AccountView,
    pub token_program: &'a AccountView,
    pub system_program: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub escrow_program: &'a AccountView,
    pub extensions: &'a AccountView,
    pub remaining_accounts: &'a [AccountView],
}

impl<'a> TryFrom<&'a [AccountView]> for DepositAccounts<'a> {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [payer, depositor, escrow, allowed_mint, receipt_seed, receipt, vault, depositor_token_account, mint, token_program, system_program, event_authority, escrow_program, extensions, remaining_accounts @ ..] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // 1. Validate signers
        verify_signer(payer, true)?;
        verify_signer(depositor, false)?;
        verify_signer(receipt_seed, false)?;

        // 2. Validate writable
        verify_writable(receipt, true)?;
        verify_writable(vault, true)?;
        verify_writable(depositor_token_account, true)?;

        // 3. Validate readonly
        verify_readonly(escrow)?;
        verify_readonly(allowed_mint)?;
        verify_readonly(mint)?;
        verify_readonly(extensions)?;

        // 4. Validate program IDs
        verify_token_program(token_program)?;
        verify_system_program(system_program)?;
        verify_current_program(escrow_program)?;
        verify_event_authority(event_authority)?;

        // 5. Validate accounts owned by current program
        verify_current_program_account(escrow)?;
        verify_current_program_account(allowed_mint)?;

        // 6. Validate ATA
        validate_associated_token_account(vault, escrow.address(), mint, token_program)?;
        validate_associated_token_account(depositor_token_account, depositor.address(), mint, token_program)?;

        Ok(Self {
            payer,
            depositor,
            escrow,
            allowed_mint,
            receipt_seed,
            receipt,
            vault,
            depositor_token_account,
            mint,
            token_program,
            system_program,
            event_authority,
            escrow_program,
            extensions,
            remaining_accounts,
        })
    }
}

impl<'a> InstructionAccounts<'a> for DepositAccounts<'a> {}
