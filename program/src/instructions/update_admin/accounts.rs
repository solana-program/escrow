use pinocchio::{account::AccountView, error::ProgramError};

use crate::{
    traits::InstructionAccounts,
    utils::{
        verify_current_program, verify_current_program_account, verify_event_authority, verify_signer, verify_writable,
    },
};

/// Accounts for the UpdateAdmin instruction
///
/// # Account Layout
/// 0. `[signer]` admin - Current admin, must match escrow.admin
/// 1. `[signer]` new_admin - New admin pubkey
/// 2. `[writable]` escrow - Escrow account to update
/// 3. `[]` event_authority - Event authority PDA
/// 4. `[]` escrow_program - Current program
pub struct UpdateAdminAccounts<'a> {
    pub admin: &'a AccountView,
    pub new_admin: &'a AccountView,
    pub escrow: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub escrow_program: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for UpdateAdminAccounts<'a> {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [admin, new_admin, escrow, event_authority, escrow_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        verify_signer(admin, false)?;
        verify_signer(new_admin, false)?;

        verify_writable(escrow, true)?;

        verify_current_program(escrow_program)?;

        verify_event_authority(event_authority)?;

        verify_current_program_account(escrow)?;

        Ok(Self { admin, new_admin, escrow, event_authority, escrow_program })
    }
}

impl<'a> InstructionAccounts<'a> for UpdateAdminAccounts<'a> {}
