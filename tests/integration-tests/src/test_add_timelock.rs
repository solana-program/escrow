use crate::{
    fixtures::{AddTimelockFixture, CreateEscrowFixture},
    utils::{
        assert_extensions_header, assert_instruction_error, assert_timelock_extension, find_escrow_pda,
        find_extensions_pda, test_empty_data, test_missing_signer, test_not_writable, test_truncated_data,
        test_wrong_account, test_wrong_current_program, test_wrong_system_program, InstructionTestFixture, TestContext,
        RANDOM_PUBKEY,
    },
};
use solana_sdk::{instruction::InstructionError, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_add_timelock_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<AddTimelockFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_add_timelock_extensions_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<AddTimelockFixture>(&mut ctx, 3);
}

#[test]
fn test_add_timelock_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<AddTimelockFixture>(&mut ctx);
}

#[test]
fn test_add_timelock_wrong_escrow_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<AddTimelockFixture>(&mut ctx);
}

#[test]
fn test_add_timelock_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<AddTimelockFixture>(&mut ctx, 5, InstructionError::Custom(2));
}

#[test]
fn test_add_timelock_invalid_extensions_bump() {
    let mut ctx = TestContext::new();
    let test_ix = AddTimelockFixture::build_valid(&mut ctx);
    let correct_bump = test_ix.instruction.data[1];
    let invalid_bump = correct_bump.wrapping_add(1);
    let error = test_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_add_timelock_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<AddTimelockFixture>(&mut ctx);
}

#[test]
fn test_add_timelock_truncated_data() {
    let mut ctx = TestContext::new();
    test_truncated_data::<AddTimelockFixture>(&mut ctx);
}

// ============================================================================
// Custom Error Tests
// ============================================================================

#[test]
fn test_add_timelock_wrong_admin() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let wrong_admin = ctx.create_funded_keypair();
    let test_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, wrong_admin, 3600);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(1));
}

#[test]
fn test_add_timelock_escrow_not_owned_by_program() {
    let mut ctx = TestContext::new();
    let test_ix = AddTimelockFixture::build_valid(&mut ctx);

    let error = test_ix.with_account_at(2, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_add_timelock_duplicate_extension() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    let first_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 3600);
    first_ix.send_expect_success(&mut ctx);

    let second_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 7200);
    let error = second_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::AccountAlreadyInitialized);
}

// ============================================================================
// Success Tests
// ============================================================================

#[test]
fn test_add_timelock_success() {
    let mut ctx = TestContext::new();
    let test_ix = AddTimelockFixture::build_valid(&mut ctx);

    let extensions_pda = test_ix.instruction.accounts[3].pubkey;
    let extensions_bump = test_ix.instruction.data[1];
    let lock_duration = u64::from_le_bytes(test_ix.instruction.data[2..10].try_into().unwrap());

    test_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_timelock_extension(&ctx, &extensions_pda, lock_duration);
}

#[test]
fn test_add_timelock_success_lock_duration_values() {
    for lock_duration in [0u64, 1, 60, 3600, 86400, i64::MAX as u64] {
        let mut ctx = TestContext::new();

        let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
        let admin = escrow_ix.signers[0].insecure_clone();
        let escrow_seed = escrow_ix.signers[1].pubkey();
        escrow_ix.send_expect_success(&mut ctx);

        let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
        let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

        let test_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin, lock_duration);
        test_ix.send_expect_success(&mut ctx);

        assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
        assert_timelock_extension(&ctx, &extensions_pda, lock_duration);
    }
}

#[test]
fn test_add_timelock_rejects_lock_duration_exceeding_i64_max() {
    for lock_duration in [i64::MAX as u64 + 1, u64::MAX] {
        let mut ctx = TestContext::new();

        let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
        let admin = escrow_ix.signers[0].insecure_clone();
        let escrow_seed = escrow_ix.signers[1].pubkey();
        escrow_ix.send_expect_success(&mut ctx);

        let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

        let test_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin, lock_duration);
        let error = test_ix.send_expect_error(&mut ctx);
        assert_instruction_error(error, InstructionError::InvalidInstructionData);
    }
}
