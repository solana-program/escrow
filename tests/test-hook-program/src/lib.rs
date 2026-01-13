//! Test Hook Program for LiteSVM Integration Tests
//!
//! Two variants via feature flags:
//! - allow: Accepts all operations
//! - deny: Rejects all operations

#![no_std]

extern crate alloc;

use pinocchio::{account::AccountView, Address, ProgramResult};

pinocchio::program_entrypoint!(process_instruction);
pinocchio::default_allocator!();
pinocchio::nostd_panic_handler!();

#[cfg(feature = "allow")]
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}

#[cfg(feature = "deny")]
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    use pinocchio::error::ProgramError;
    Err(ProgramError::Custom(1))
}

#[cfg(not(any(feature = "allow", feature = "deny")))]
pub fn process_instruction(
    _program_id: &Address,
    _accounts: &[AccountView],
    _instruction_data: &[u8],
) -> ProgramResult {
    Ok(())
}
