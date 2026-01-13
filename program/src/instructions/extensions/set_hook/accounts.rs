use pinocchio::{account::AccountView, error::ProgramError};

use crate::{
    traits::InstructionAccounts,
    utils::{
        verify_current_program, verify_current_program_account, verify_event_authority, verify_readonly, verify_signer,
        verify_system_program, verify_writable,
    },
};

/// Accounts for the SetHook instruction
///
/// # Account Layout
/// 0. `[signer, writable]` payer - Pays for account creation
/// 1. `[signer]` admin - Must match escrow.admin
/// 2. `[]` escrow - Escrow account to set hook on
/// 3. `[writable]` extensions - Extensions PDA (created if doesn't exist)
/// 4. `[]` system_program - System program for account creation
/// 5. `[]` event_authority - Event authority PDA
/// 6. `[]` escrow_program - Current program
pub struct SetHookAccounts<'a> {
    pub payer: &'a AccountView,
    pub admin: &'a AccountView,
    pub escrow: &'a AccountView,
    pub extensions: &'a AccountView,
    pub system_program: &'a AccountView,
    pub event_authority: &'a AccountView,
    pub escrow_program: &'a AccountView,
}

impl<'a> TryFrom<&'a [AccountView]> for SetHookAccounts<'a> {
    type Error = ProgramError;

    #[inline(always)]
    fn try_from(accounts: &'a [AccountView]) -> Result<Self, Self::Error> {
        let [payer, admin, escrow, extensions, system_program, event_authority, escrow_program] = accounts else {
            return Err(ProgramError::NotEnoughAccountKeys);
        };

        verify_signer(payer, true)?;
        verify_signer(admin, false)?;

        verify_writable(extensions, true)?;

        verify_readonly(escrow)?;

        verify_system_program(system_program)?;
        verify_current_program(escrow_program)?;

        verify_event_authority(event_authority)?;

        verify_current_program_account(escrow)?;

        Ok(Self { payer, admin, escrow, extensions, system_program, event_authority, escrow_program })
    }
}

impl<'a> InstructionAccounts<'a> for SetHookAccounts<'a> {}
