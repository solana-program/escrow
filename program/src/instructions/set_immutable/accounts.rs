use pinocchio::{account::AccountView, error::ProgramError};

use crate::{
    traits::InstructionAccounts,
    utils::{
        verify_current_program, verify_current_program_account, verify_event_authority, verify_signer, verify_writable,
    },
};

/// Accounts for the SetImmutable instruction
///
/// # Account Layout
/// 0. `[signer]` admin - Current admin, must match escrow.admin
/// 1. `[writable]` escrow - Escrow account to lock as immutable
/// 2. `[]` event_authority - Event authority PDA
/// 3. `[]` escrow_program - Current program
pub struct SetImmutableAccounts<'a> {
    pub admin: &'a AccountView,
    pub escrow: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub escrow_program: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for SetImmutableAccounts<'a> {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [admin, escrow, event_authority, escrow_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        // 1. Validate signers
        verify_signer(admin, false)?;

        // 2. Validate writable
        verify_writable(escrow, true)?;

        // 3. Validate program IDs
        verify_current_program(escrow_program)?;
        verify_event_authority(event_authority)?;

        // 4. Validate accounts owned by current program
        verify_current_program_account(escrow)?;

        Ok(Self { admin, escrow, event_authority, escrow_program })
    }
}

impl<'a> InstructionAccounts<'a> for SetImmutableAccounts<'a> {}
