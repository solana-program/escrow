use alloc::vec::Vec;
use pinocchio::{
    account::AccountView,
    cpi::invoke_with_bounds,
    instruction::{InstructionAccount, InstructionView},
    ProgramResult,
};

use crate::{
    errors::EscrowProgramError,
    state::{EscrowExtensionsHeader, HookData, HookPoint},
    utils::TlvReader,
};

/// Checks if a hook is configured in the extensions account.
///
/// Returns `Some(HookData)` if the extensions account has data and contains a hook extension, `None` otherwise.
pub fn get_and_validate_hook(
    extensions: &AccountView,
    remaining_accounts: &[AccountView],
) -> Result<Option<HookData>, pinocchio::error::ProgramError> {
    if extensions.data_len() == 0 {
        return Ok(None);
    }

    let extensions_data = extensions.try_borrow()?;
    let _header = EscrowExtensionsHeader::from_bytes(&extensions_data)?;

    let reader = TlvReader::new(&extensions_data);

    let hook_data = reader.read_hook();

    if let Some(hook_data) = hook_data {
        // Verify hook_program account is provided and matches the stored address
        let hook_program = remaining_accounts.first().ok_or(EscrowProgramError::HookProgramMismatch)?;
        if hook_program.address() != &hook_data.hook_program {
            return Err(EscrowProgramError::HookProgramMismatch.into());
        }
    }

    Ok(hook_data)
}

/// Invokes a hook program. Call `is_hook_present` first to check if hook exists.
///
/// # Arguments
/// * `hook_data` - Hook data containing the hook program address
/// * `hook_point` - The hook point discriminator
/// * `remaining_accounts` - Remaining accounts slice: [hook_program, extra_accounts...]
/// * `core_accounts` - Core accounts to pass to hook (escrow, actor, mint, receipt, vault)
///
/// # Returns
/// * `Ok(())` if hook succeeds
/// * `Err(HookRejected)` if hook returns error or remaining_accounts is invalid
pub fn invoke_hook(
    hook_data: &HookData,
    hook_point: HookPoint,
    remaining_accounts: &[AccountView],
    core_accounts: &[&AccountView],
) -> ProgramResult {
    let extra_accounts = remaining_accounts.get(1..).unwrap_or(&[]);
    let all_accounts: Vec<&AccountView> = core_accounts.iter().copied().chain(extra_accounts.iter()).collect();

    // Build instruction accounts - ALL accounts are read-only
    let instruction_accounts: Vec<InstructionAccount> = all_accounts.iter().map(|acc| (*acc).into()).collect();

    // Instruction data is just the 1-byte hook point discriminator
    let instruction_data = [hook_point as u8];

    let instruction = InstructionView {
        program_id: &hook_data.hook_program,
        accounts: &instruction_accounts,
        data: &instruction_data,
    };

    invoke_with_bounds::<16>(&instruction, &all_accounts).map_err(|_| EscrowProgramError::HookRejected.into())
}
