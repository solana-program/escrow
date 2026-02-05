use crate::{
    fixtures::{CreateEscrowFixture, UpdateAdminFixture},
    utils::{
        assert_escrow_account, assert_instruction_error, find_escrow_pda, test_missing_signer, test_not_writable,
        test_wrong_account, test_wrong_current_program, InstructionTestFixture, TestContext, TestInstruction,
        RANDOM_PUBKEY,
    },
};
use escrow_program_client::instructions::UpdateAdminBuilder;
use solana_sdk::{
    instruction::InstructionError,
    signature::{Keypair, Signer},
};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_update_admin_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<UpdateAdminFixture>(&mut ctx, 0, 0);
}

#[test]
fn test_update_admin_missing_new_admin_signer() {
    let mut ctx = TestContext::new();
    // new_admin is at account index 1, signer vec index 1
    test_missing_signer::<UpdateAdminFixture>(&mut ctx, 1, 1);
}

#[test]
fn test_update_admin_escrow_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<UpdateAdminFixture>(&mut ctx, 2);
}

#[test]
fn test_update_admin_wrong_escrow_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<UpdateAdminFixture>(&mut ctx);
}

#[test]
fn test_update_admin_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<UpdateAdminFixture>(&mut ctx, 3, InstructionError::Custom(2));
}

// ============================================================================
// Custom Error Tests
// ============================================================================

#[test]
fn test_update_admin_wrong_admin() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let wrong_admin = ctx.create_funded_keypair();
    let new_admin = Keypair::new();

    let test_ix = UpdateAdminFixture::build_with_escrow(&mut ctx, escrow_pda, wrong_admin, new_admin);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(1)); // InvalidAdmin
}

#[test]
fn test_update_admin_escrow_not_owned_by_program() {
    let mut ctx = TestContext::new();
    let test_ix = UpdateAdminFixture::build_valid(&mut ctx);

    let error = test_ix.with_account_at(2, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

// ============================================================================
// Success Tests
// ============================================================================

#[test]
fn test_update_admin_success() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    let bump = escrow_ix.instruction.data[1];
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    // Verify initial admin
    assert_escrow_account(&ctx, &escrow_pda, &admin.pubkey(), bump, &escrow_seed);

    // Update to new admin
    let new_admin = Keypair::new();
    let new_admin_pubkey = new_admin.pubkey();
    let update_ix = UpdateAdminFixture::build_with_escrow(&mut ctx, escrow_pda, admin, new_admin);
    update_ix.send_expect_success(&mut ctx);

    // Verify admin was updated
    assert_escrow_account(&ctx, &escrow_pda, &new_admin_pubkey, bump, &escrow_seed);
}

#[test]
fn test_update_admin_can_update_again() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    let bump = escrow_ix.instruction.data[1];
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    // First update
    let new_admin1 = ctx.create_funded_keypair();
    let new_admin1_clone = new_admin1.insecure_clone();
    let update_ix1 = UpdateAdminFixture::build_with_escrow(&mut ctx, escrow_pda, admin, new_admin1);
    update_ix1.send_expect_success(&mut ctx);

    assert_escrow_account(&ctx, &escrow_pda, &new_admin1_clone.pubkey(), bump, &escrow_seed);

    // Second update (using new admin)
    let new_admin2 = Keypair::new();
    let new_admin2_pubkey = new_admin2.pubkey();
    let update_ix2 = UpdateAdminFixture::build_with_escrow(&mut ctx, escrow_pda, new_admin1_clone, new_admin2);
    update_ix2.send_expect_success(&mut ctx);

    assert_escrow_account(&ctx, &escrow_pda, &new_admin2_pubkey, bump, &escrow_seed);
}

#[test]
fn test_update_admin_old_admin_cannot_update() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let original_admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    // Update to new admin
    let new_admin = ctx.create_funded_keypair();
    let update_ix =
        UpdateAdminFixture::build_with_escrow(&mut ctx, escrow_pda, original_admin.insecure_clone(), new_admin);
    update_ix.send_expect_success(&mut ctx);

    // Try to update using old admin - should fail
    let another_admin = Keypair::new();
    let bad_update_ix = UpdateAdminFixture::build_with_escrow(&mut ctx, escrow_pda, original_admin, another_admin);
    let error = bad_update_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(1)); // InvalidAdmin
}

// ============================================================================
// Additional Tests
// ============================================================================

#[test]
fn test_update_admin_idempotent() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    let bump = escrow_ix.instruction.data[1];
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    // Update admin to the same admin (idempotent operation)
    let same_admin = admin.insecure_clone();
    let instruction =
        UpdateAdminBuilder::new().admin(admin.pubkey()).new_admin(same_admin.pubkey()).escrow(escrow_pda).instruction();

    let test_ix =
        TestInstruction { instruction, signers: vec![admin.insecure_clone(), same_admin], name: "UpdateAdmin" };
    test_ix.send_expect_success(&mut ctx);

    // Verify admin is still the same
    assert_escrow_account(&ctx, &escrow_pda, &admin.pubkey(), bump, &escrow_seed);
}
