use pinocchio::{account::AccountView, error::ProgramError};

use crate::{
    traits::InstructionAccounts,
    utils::{
        verify_current_program, verify_current_program_account, verify_event_authority, verify_readonly, verify_signer,
        verify_system_program, verify_token_program, verify_token_program_account, verify_writable,
    },
};

/// Accounts for the AllowMint instruction
///
/// Creates an AllowedMint PDA, enabling deposits of that mint into the escrow.
///
/// # Account Layout
/// 0. `[signer, writable]` payer - Pays for account creation
/// 1. `[signer]` admin - Must match escrow.admin
/// 2. `[]` escrow - Escrow PDA (validates admin)
/// 3. `[]` escrow_extensions - Extensions PDA `[b"extensions", escrow]` (may be empty/uninitialized)
/// 4. `[]` mint - Mint account to allow (must be owned by token_program)
/// 5. `[writable]` allowed_mint - PDA to create `[b"allowed_mint", escrow, mint]`
/// 6. `[]` token_program - Token program (validates mint ownership)
/// 7. `[]` system_program - System program for account creation
/// 8. `[]` event_authority - Event authority PDA
/// 9. `[]` escrow_program - Current program (for event emission)
pub struct AllowMintAccounts<'a> {
    pub payer: &'a AccountView,
    pub admin: &'a AccountView,
    pub escrow: &'a AccountView,
    pub escrow_extensions: &'a AccountView,
    pub mint: &'a AccountView,
    pub allowed_mint: &'a AccountView,
    pub token_program: &'a AccountView,
    pub system_program: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub escrow_program: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for AllowMintAccounts<'a> {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [payer, admin, escrow, escrow_extensions, mint, allowed_mint, token_program, system_program, event_authority, escrow_program] =
            accounts
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        verify_signer(payer, true)?;
        verify_signer(admin, false)?;

        verify_writable(allowed_mint, true)?;

        verify_readonly(escrow)?;
        verify_readonly(escrow_extensions)?;
        verify_readonly(mint)?;

        verify_current_program_account(escrow)?;

        verify_token_program(token_program)?;
        verify_token_program_account(mint)?;

        verify_system_program(system_program)?;
        verify_current_program(escrow_program)?;
        verify_event_authority(event_authority)?;

        Ok(Self {
            payer,
            admin,
            escrow,
            escrow_extensions,
            mint,
            allowed_mint,
            token_program,
            system_program,
            event_authority,
            escrow_program,
        })
    }
}

impl<'a> InstructionAccounts<'a> for AllowMintAccounts<'a> {}
