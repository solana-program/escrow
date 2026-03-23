use crate::{
    fixtures::{CreateEscrowFixture, SetImmutableFixture},
    utils::{
        assert_escrow_error, assert_escrow_mutability, find_escrow_pda, test_empty_data, test_missing_signer,
        test_not_writable, test_wrong_account, test_wrong_current_program, InstructionTestFixture, TestContext,
    },
};
use solana_sdk::{instruction::InstructionError, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_set_immutable_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<SetImmutableFixture>(&mut ctx, 0, 0);
}

#[test]
fn test_set_immutable_escrow_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<SetImmutableFixture>(&mut ctx, 1);
}

#[test]
fn test_set_immutable_wrong_current_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<SetImmutableFixture>(&mut ctx);
}

#[test]
fn test_set_immutable_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<SetImmutableFixture>(&mut ctx, 2, InstructionError::Custom(2));
}

#[test]
fn test_set_immutable_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<SetImmutableFixture>(&mut ctx);
}

#[test]
fn test_set_immutable_wrong_admin() {
    let mut ctx = TestContext::new();
    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let wrong_admin = ctx.create_funded_keypair();
    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let test_ix = SetImmutableFixture::build_with_escrow(&mut ctx, escrow_pda, wrong_admin);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_escrow_error(error, escrow_program_client::errors::EscrowProgramError::InvalidAdmin);
}

// ============================================================================
// Happy Path Tests
// ============================================================================

#[test]
fn test_set_immutable_success() {
    let mut ctx = TestContext::new();
    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    assert_escrow_mutability(&ctx, &escrow_pda, false);

    let test_ix = SetImmutableFixture::build_with_escrow(&mut ctx, escrow_pda, admin);
    test_ix.send_expect_success(&mut ctx);

    assert_escrow_mutability(&ctx, &escrow_pda, true);
}

#[test]
fn test_set_immutable_idempotent() {
    let mut ctx = TestContext::new();
    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    let first_ix = SetImmutableFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone());
    first_ix.send_expect_success(&mut ctx);

    ctx.warp_to_slot(2);

    let second_ix = SetImmutableFixture::build_with_escrow(&mut ctx, escrow_pda, admin);
    second_ix.send_expect_success(&mut ctx);

    assert_escrow_mutability(&ctx, &escrow_pda, true);
}
