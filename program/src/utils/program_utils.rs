use crate::ID as ESCROW_PROGRAM_ID;
use pinocchio::{account::AccountView, error::ProgramError};
use pinocchio_token::ID as TOKEN_PROGRAM_ID;
use pinocchio_token_2022::ID as TOKEN_2022_PROGRAM_ID;

/// Verify the account is a system program, returning an error if it is not.
///
/// # Arguments
/// * `account` - The account to verify.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_system_program(account: &AccountView) -> Result<(), ProgramError> {
    if account.address() != &pinocchio_system::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

/// Verify the account is a token program (SPL Token or Token-2022), returning an error if it is not.
///
/// # Arguments
/// * `account` - The account to verify.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_token_program(account: &AccountView) -> Result<(), ProgramError> {
    if account.address() != &TOKEN_PROGRAM_ID && account.address() != &TOKEN_2022_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}

/// Verify the account is the current program, returning an error if it is not.
///
/// # Arguments
/// * `account` - The account to verify.
///
/// # Returns
/// * `Result<(), ProgramError>` - The result of the operation
#[inline(always)]
pub fn verify_current_program(account: &AccountView) -> Result<(), ProgramError> {
    if account.address() != &ESCROW_PROGRAM_ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    Ok(())
}
