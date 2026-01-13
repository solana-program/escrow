//! Account validation utilities.
//!
//! # Security Note: Closed Account Discriminator Pattern
//!
//! This program does NOT implement the closed account discriminator pattern (setting first byte
//! to 0xff before closing). This is intentional because:
//!
//! 1. All account access validates ownership via `verify_owned_by()`.
//!    After closing, accounts are assigned to system program, so any attempt to use a "revived"
//!    account fails the ownership check.
//!
use crate::ID as ESCROW_PROGRAM_ID;
use pinocchio::{account::AccountView, address::Address, error::ProgramError};
use pinocchio_token::ID as TOKEN_PROGRAM_ID;
use pinocchio_token_2022::ID as TOKEN_2022_PROGRAM_ID;

/// Verify account as writable, returning an error if it is not or if it is not writable while
/// expected to be.
///
/// # Arguments
/// * `account` - The account to verify.
/// * `expect_writable` - Whether the account should be writable
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_writable(account: &AccountView, expect_writable: bool) -> Result<(), ProgramError> {
    if expect_writable && !account.is_writable() {
        return Err(ProgramError::Immutable);
    }
    Ok(())
}

/// Verify account is read-only, returning an error if it is writable.
///
/// Enforces that accounts declared as read-only in the IDL are actually passed as read-only,
/// preventing accidental mutations even if the processor has bugs.
///
/// # Arguments
/// * `account` - The account to verify.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_readonly(account: &AccountView) -> Result<(), ProgramError> {
    if account.is_writable() {
        return Err(ProgramError::AccountBorrowFailed);
    }
    Ok(())
}

/// Verify account as a signer, returning an error if it is not or if it is not writable while
/// expected to be.
///
/// # Arguments
/// * `account` - The account to verify.
/// * `expect_writable` - Whether the account should be writable
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_signer(account: &AccountView, expect_writable: bool) -> Result<(), ProgramError> {
    if !account.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    verify_writable(account, expect_writable)
}

/// Verify account's owner, returning an error if it is not the expected owner.
///
/// # Arguments
/// * `account` - The account to verify.
/// * `owner` - The expected owner.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_owned_by(account: &AccountView, owner: &Address) -> Result<(), ProgramError> {
    if !account.owned_by(owner) {
        return Err(ProgramError::InvalidAccountOwner);
    }

    Ok(())
}

/// Verify account is a system account, returning an error if it is not.
///
/// # Arguments
/// * `account` - The account to verify.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_system_account(account: &AccountView) -> Result<(), ProgramError> {
    verify_owned_by(account, &pinocchio_system::ID)
}

/// Verify account is a token program account (owned by SPL Token or Token-2022), returning an error if it is not.
///
/// # Arguments
/// * `account` - The account to verify.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_token_program_account(account: &AccountView) -> Result<(), ProgramError> {
    if !account.owned_by(&TOKEN_PROGRAM_ID) && !account.owned_by(&TOKEN_2022_PROGRAM_ID) {
        return Err(ProgramError::InvalidAccountOwner);
    }
    Ok(())
}

/// Verify account is the current program, returning an error if it is not.
///
/// # Arguments
/// * `account` - The account to verify.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_current_program_account(account: &AccountView) -> Result<(), ProgramError> {
    verify_owned_by(account, &ESCROW_PROGRAM_ID)
}
