use crate::{
    fixtures::CreateEscrowFixture,
    utils::{
        assert_escrow_account, assert_instruction_error, test_empty_data, test_missing_signer, test_not_writable,
        test_wrong_account, test_wrong_current_program, test_wrong_system_program, InstructionTestFixture, TestContext,
    },
};
use escrow_program_client::instructions::CreatesEscrowBuilder;
use solana_sdk::{instruction::InstructionError, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_create_escrow_missing_admin_signer() {
    let mut ctx = TestContext::new();
    // admin is at account index 1, signer vec index 0 (payer is handled separately)
    test_missing_signer::<CreateEscrowFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_create_escrow_missing_escrow_seed_signer() {
    let mut ctx = TestContext::new();
    // escrow_seed is at account index 2, signer vec index 1
    test_missing_signer::<CreateEscrowFixture>(&mut ctx, 2, 1);
}

#[test]
fn test_create_escrow_escrow_not_writable() {
    let mut ctx = TestContext::new();
    // escrow is at index 3 in instruction accounts
    test_not_writable::<CreateEscrowFixture>(&mut ctx, 3);
}

#[test]
fn test_create_escrow_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<CreateEscrowFixture>(&mut ctx);
}

#[test]
fn test_create_escrow_wrong_current_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<CreateEscrowFixture>(&mut ctx);
}

#[test]
fn test_create_escrow_invalid_event_authority() {
    let mut ctx = TestContext::new();
    // event_authority is at index 5 in instruction accounts
    // Custom error 2 = InvalidEventAuthority
    test_wrong_account::<CreateEscrowFixture>(&mut ctx, 5, InstructionError::Custom(2));
}

#[test]
fn test_create_escrow_invalid_bump() {
    let mut ctx = TestContext::new();

    // Build a valid instruction first to get the correct bump
    let valid_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let correct_bump = valid_ix.instruction.data[1];

    // Use incorrect bump (correct_bump + 1, wrapping)
    let invalid_bump = correct_bump.wrapping_add(1);

    let error = valid_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);

    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_create_escrow_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<CreateEscrowFixture>(&mut ctx);
}

// ============================================================================
// Happy Path Test
// ============================================================================

#[test]
fn test_create_escrow_success() {
    let mut ctx = TestContext::new();
    let test_ix = CreateEscrowFixture::build_valid(&mut ctx);

    let admin_pubkey = test_ix.signers[0].pubkey();
    let escrow_seed_pubkey = test_ix.signers[1].pubkey();
    let escrow_pda = test_ix.instruction.accounts[3].pubkey;
    let bump = test_ix.instruction.data[1];

    test_ix.send_expect_success(&mut ctx);

    assert_escrow_account(&ctx, &escrow_pda, &admin_pubkey, bump, &escrow_seed_pubkey);
}

// ============================================================================
// Re-initialization Protection Tests
// ============================================================================

#[test]
fn test_create_escrow_reinitialization_fails() {
    let mut ctx = TestContext::new();
    let test_ix = CreateEscrowFixture::build_valid(&mut ctx);

    let admin = test_ix.signers[0].insecure_clone();
    let escrow_seed = test_ix.signers[1].insecure_clone();
    let escrow_pda = test_ix.instruction.accounts[3].pubkey;
    let bump = test_ix.instruction.data[1];

    test_ix.send_expect_success(&mut ctx);

    assert_escrow_account(&ctx, &escrow_pda, &admin.pubkey(), bump, &escrow_seed.pubkey());

    let attacker = ctx.create_funded_keypair();
    let reinit_ix = CreatesEscrowBuilder::new()
        .payer(ctx.payer.pubkey())
        .admin(attacker.pubkey())
        .escrow_seed(escrow_seed.pubkey())
        .escrow(escrow_pda)
        .bump(bump)
        .instruction();

    let error = ctx.send_transaction_expect_error(reinit_ix, &[&attacker, &escrow_seed]);

    assert_instruction_error(error, InstructionError::AccountAlreadyInitialized);

    assert_escrow_account(&ctx, &escrow_pda, &admin.pubkey(), bump, &escrow_seed.pubkey());
}
