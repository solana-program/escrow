use pinocchio::{account::AccountView, error::ProgramError};

use crate::{
    traits::InstructionAccounts,
    utils::{
        verify_current_program, verify_current_program_account, verify_event_authority, verify_readonly, verify_signer,
        verify_token_program, verify_token_program_account, verify_writable,
    },
};

/// Accounts for the BlockMint instruction
///
/// Closes the AllowedMint PDA, blocking future deposits of that mint.
///
/// # Account Layout
/// 0. `[signer]` admin - Must match escrow.admin
/// 1. `[signer]` payer - Transaction fee payer
/// 2. `[writable]` rent_recipient - Receives rent refund from closed account
/// 3. `[]` escrow - Escrow PDA (validates admin)
/// 4. `[]` mint - Token mint being blocked
/// 5. `[writable]` allowed_mint - PDA to close `[b"allowed_mint", escrow, mint]`
/// 6. `[]` token_program - Token program (SPL Token or Token-2022)
/// 7. `[]` event_authority - Event authority PDA
/// 8. `[]` escrow_program - Current program (for event emission)
pub struct BlockMintAccounts<'a> {
    pub admin: &'a AccountView,
    pub payer: &'a AccountView,
    pub rent_recipient: &'a AccountView,
    pub escrow: &'a AccountView,
    pub mint: &'a AccountView,
    pub allowed_mint: &'a AccountView,
    pub token_program: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub escrow_program: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for BlockMintAccounts<'a> {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [admin, payer, rent_recipient, escrow, mint, allowed_mint, token_program, event_authority, escrow_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        verify_signer(admin, false)?;
        verify_signer(payer, false)?;

        verify_writable(rent_recipient, true)?;
        verify_writable(allowed_mint, true)?;

        verify_readonly(escrow)?;
        verify_readonly(mint)?;

        verify_current_program_account(escrow)?;
        verify_current_program_account(allowed_mint)?;

        verify_token_program(token_program)?;
        verify_token_program_account(mint)?;

        verify_current_program(escrow_program)?;
        verify_event_authority(event_authority)?;

        Ok(Self {
            admin,
            payer,
            rent_recipient,
            escrow,
            mint,
            allowed_mint,
            token_program,
            event_authority,
            escrow_program,
        })
    }
}

impl<'a> InstructionAccounts<'a> for BlockMintAccounts<'a> {}
