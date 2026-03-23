use crate::{
    fixtures::{
        AddBlockTokenExtensionsFixture, AddTimelockFixture, CreateEscrowFixture, RemoveExtensionFixture,
        SetArbiterFixture, SetHookFixture, SetImmutableFixture,
    },
    utils::extensions_utils::{
        EXTENSION_TYPE_ARBITER, EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS, EXTENSION_TYPE_HOOK, EXTENSION_TYPE_TIMELOCK,
    },
    utils::{
        assert_arbiter_extension, assert_block_token_extensions_extension, assert_escrow_error,
        assert_extension_missing, assert_extensions_header, assert_instruction_error, find_escrow_pda,
        find_extensions_pda, test_empty_data, test_missing_signer, test_not_writable, test_truncated_data,
        test_wrong_account, test_wrong_current_program, test_wrong_system_program, EscrowError, InstructionTestFixture,
        TestContext, RANDOM_PUBKEY,
    },
};
use solana_sdk::{instruction::InstructionError, pubkey::Pubkey, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_remove_extension_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<RemoveExtensionFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_remove_extension_extensions_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<RemoveExtensionFixture>(&mut ctx, 3);
}

#[test]
fn test_remove_extension_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<RemoveExtensionFixture>(&mut ctx);
}

#[test]
fn test_remove_extension_wrong_escrow_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<RemoveExtensionFixture>(&mut ctx);
}

#[test]
fn test_remove_extension_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<RemoveExtensionFixture>(&mut ctx, 5, InstructionError::Custom(2));
}

#[test]
fn test_remove_extension_invalid_extensions_bump() {
    let mut ctx = TestContext::new();
    let test_ix = RemoveExtensionFixture::build_valid(&mut ctx);
    let correct_bump = test_ix.instruction.data[1];
    let invalid_bump = correct_bump.wrapping_add(1);
    let error = test_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_remove_extension_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<RemoveExtensionFixture>(&mut ctx);
}

#[test]
fn test_remove_extension_truncated_data() {
    let mut ctx = TestContext::new();
    test_truncated_data::<RemoveExtensionFixture>(&mut ctx);
}

// ============================================================================
// Custom Error Tests
// ============================================================================

#[test]
fn test_remove_extension_wrong_admin() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    SetHookFixture::build_with_escrow(&mut ctx, escrow_pda, admin, Pubkey::new_unique()).send_expect_success(&mut ctx);

    let wrong_admin = ctx.create_funded_keypair();
    let test_ix = RemoveExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, wrong_admin, EXTENSION_TYPE_HOOK);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(1));
}

#[test]
fn test_remove_extension_escrow_not_owned_by_program() {
    let mut ctx = TestContext::new();
    let test_ix = RemoveExtensionFixture::build_valid(&mut ctx);

    let error = test_ix.with_account_at(2, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_remove_extension_fails_when_escrow_is_immutable() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    SetHookFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), Pubkey::new_unique())
        .send_expect_success(&mut ctx);

    let set_immutable_ix = SetImmutableFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone());
    set_immutable_ix.send_expect_success(&mut ctx);

    let remove_ix = RemoveExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, EXTENSION_TYPE_HOOK);
    let error = remove_ix.send_expect_error(&mut ctx);
    assert_escrow_error(error, EscrowError::EscrowImmutable);
}

#[test]
fn test_remove_extension_invalid_type() {
    let mut ctx = TestContext::new();
    let test_ix = RemoveExtensionFixture::build_valid(&mut ctx);

    // extension_type starts at data[2..4]
    let error = test_ix.with_data_byte_at(2, 250).with_data_byte_at(3, 0).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountData);
}

#[test]
fn test_remove_extension_not_found() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    SetHookFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), Pubkey::new_unique())
        .send_expect_success(&mut ctx);

    // Timelock was never set.
    let test_ix = RemoveExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, EXTENSION_TYPE_TIMELOCK);
    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::UninitializedAccount);
}

// ============================================================================
// Success Tests
// ============================================================================

#[test]
fn test_remove_extension_success_remove_hook() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    SetHookFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), Pubkey::new_unique())
        .send_expect_success(&mut ctx);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);

    let remove_ix = RemoveExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, EXTENSION_TYPE_HOOK);
    remove_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 0);
    assert_extension_missing(&ctx, &extensions_pda, EXTENSION_TYPE_HOOK);
}

#[test]
fn test_remove_extension_success_remove_timelock_keeps_other_extensions() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 3600)
        .send_expect_success(&mut ctx);
    let hook_program = Pubkey::new_unique();
    SetHookFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), hook_program)
        .send_expect_success(&mut ctx);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 2);

    let remove_ix = RemoveExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, EXTENSION_TYPE_TIMELOCK);
    remove_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_extension_missing(&ctx, &extensions_pda, EXTENSION_TYPE_TIMELOCK);
    crate::utils::assert_hook_extension(&ctx, &extensions_pda, &hook_program);
}

#[test]
fn test_remove_extension_success_remove_arbiter() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    let arbiter = ctx.create_funded_keypair();
    SetArbiterFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), arbiter.insecure_clone())
        .send_expect_success(&mut ctx);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_arbiter_extension(&ctx, &extensions_pda, &arbiter.pubkey());

    let remove_ix = RemoveExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, EXTENSION_TYPE_ARBITER);
    remove_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 0);
    assert_extension_missing(&ctx, &extensions_pda, EXTENSION_TYPE_ARBITER);
}

#[test]
fn test_remove_extension_success_remove_blocked_token_extensions() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 99u16)
        .send_expect_success(&mut ctx);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &[99u16]);

    let remove_ix =
        RemoveExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS);
    remove_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 0);
    assert_extension_missing(&ctx, &extensions_pda, EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS);
}
