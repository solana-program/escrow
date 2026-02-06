use crate::{
    fixtures::{AddTimelockFixture, CreateEscrowFixture, SetArbiterFixture, SetHookFixture},
    utils::{
        assert_arbiter_extension, assert_extensions_header, assert_hook_extension, assert_instruction_error,
        assert_timelock_extension, find_escrow_pda, find_extensions_pda, test_empty_data, test_missing_signer,
        test_not_writable, test_truncated_data, test_wrong_account, test_wrong_current_program,
        test_wrong_system_program, InstructionTestFixture, TestContext, RANDOM_PUBKEY,
    },
};
use solana_sdk::{
    instruction::InstructionError,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_set_arbiter_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<SetArbiterFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_set_arbiter_missing_arbiter_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<SetArbiterFixture>(&mut ctx, 2, 1);
}

#[test]
fn test_set_arbiter_extensions_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<SetArbiterFixture>(&mut ctx, 4);
}

#[test]
fn test_set_arbiter_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<SetArbiterFixture>(&mut ctx);
}

#[test]
fn test_set_arbiter_wrong_escrow_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<SetArbiterFixture>(&mut ctx);
}

#[test]
fn test_set_arbiter_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<SetArbiterFixture>(&mut ctx, 6, InstructionError::Custom(2));
}

#[test]
fn test_set_arbiter_invalid_extensions_bump() {
    let mut ctx = TestContext::new();
    let test_ix = SetArbiterFixture::build_valid(&mut ctx);
    let correct_bump = test_ix.instruction.data[1];
    let invalid_bump = correct_bump.wrapping_add(1);
    let error = test_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_set_arbiter_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<SetArbiterFixture>(&mut ctx);
}

#[test]
fn test_set_arbiter_truncated_data() {
    let mut ctx = TestContext::new();
    test_truncated_data::<SetArbiterFixture>(&mut ctx);
}

// ============================================================================
// Custom Error Tests
// ============================================================================

#[test]
fn test_set_arbiter_wrong_admin() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let wrong_admin = ctx.create_funded_keypair();
    let arbiter = Keypair::new();
    let test_ix = SetArbiterFixture::build_with_escrow(&mut ctx, escrow_pda, wrong_admin, arbiter);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(1));
}

#[test]
fn test_set_arbiter_escrow_not_owned_by_program() {
    let mut ctx = TestContext::new();
    let test_ix = SetArbiterFixture::build_valid(&mut ctx);

    let error = test_ix.with_account_at(3, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_set_arbiter_duplicate_extension() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let arbiter = Keypair::new();

    let first_ix = SetArbiterFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), arbiter);
    first_ix.send_expect_success(&mut ctx);

    // Second attempt should fail — arbiter is immutable
    let second_ix = SetArbiterFixture::build_with_escrow(&mut ctx, escrow_pda, admin, Keypair::new());
    let error = second_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::AccountAlreadyInitialized);
}

// ============================================================================
// Success Tests
// ============================================================================

#[test]
fn test_set_arbiter_success() {
    let mut ctx = TestContext::new();
    let test_ix = SetArbiterFixture::build_valid(&mut ctx);

    let extensions_pda = test_ix.instruction.accounts[4].pubkey;
    let extensions_bump = test_ix.instruction.data[1];
    let arbiter = test_ix.instruction.accounts[2].pubkey;

    test_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_arbiter_extension(&ctx, &extensions_pda, &arbiter);
}

// ============================================================================
// Combined Extension Tests
// ============================================================================

#[test]
fn test_add_timelock_then_set_arbiter() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    let timelock_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 3600);
    timelock_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_timelock_extension(&ctx, &extensions_pda, 3600);

    let arbiter = Keypair::new();
    let arbiter_pubkey = arbiter.pubkey();
    let arbiter_ix = SetArbiterFixture::build_with_escrow(&mut ctx, escrow_pda, admin, arbiter);
    arbiter_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 2);
    assert_timelock_extension(&ctx, &extensions_pda, 3600);
    assert_arbiter_extension(&ctx, &extensions_pda, &arbiter_pubkey);
}

#[test]
fn test_set_arbiter_then_set_hook() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    let arbiter = Keypair::new();
    let arbiter_pubkey = arbiter.pubkey();
    let arbiter_ix = SetArbiterFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), arbiter);
    arbiter_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_arbiter_extension(&ctx, &extensions_pda, &arbiter_pubkey);

    let hook_program = Pubkey::new_unique();
    let hook_ix = SetHookFixture::build_with_escrow(&mut ctx, escrow_pda, admin, hook_program);
    hook_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 2);
    assert_arbiter_extension(&ctx, &extensions_pda, &arbiter_pubkey);
    assert_hook_extension(&ctx, &extensions_pda, &hook_program);
}

#[test]
fn test_all_three_extensions() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    let timelock_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 7200);
    timelock_ix.send_expect_success(&mut ctx);

    let hook_program = Pubkey::new_unique();
    let hook_ix = SetHookFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), hook_program);
    hook_ix.send_expect_success(&mut ctx);

    let arbiter = Keypair::new();
    let arbiter_pubkey = arbiter.pubkey();
    let arbiter_ix = SetArbiterFixture::build_with_escrow(&mut ctx, escrow_pda, admin, arbiter);
    arbiter_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 3);
    assert_timelock_extension(&ctx, &extensions_pda, 7200);
    assert_hook_extension(&ctx, &extensions_pda, &hook_program);
    assert_arbiter_extension(&ctx, &extensions_pda, &arbiter_pubkey);
}
