use crate::{
    fixtures::{
        AddBlockTokenExtensionsFixture, AddTimelockFixture, CreateEscrowFixture, SetImmutableFixture,
        UnblockTokenExtensionFixture,
    },
    utils::extensions_utils::EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS,
    utils::{
        assert_block_token_extensions_extension, assert_escrow_error, assert_extension_missing,
        assert_extensions_header, assert_instruction_error, assert_timelock_extension, find_escrow_pda,
        find_extensions_pda, test_empty_data, test_missing_signer, test_not_writable, test_truncated_data,
        test_wrong_account, test_wrong_current_program, test_wrong_system_program, EscrowError, InstructionTestFixture,
        TestContext, RANDOM_PUBKEY,
    },
};
use solana_sdk::{instruction::InstructionError, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_unblock_token_extension_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<UnblockTokenExtensionFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_unblock_token_extension_extensions_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<UnblockTokenExtensionFixture>(&mut ctx, 3);
}

#[test]
fn test_unblock_token_extension_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<UnblockTokenExtensionFixture>(&mut ctx);
}

#[test]
fn test_unblock_token_extension_wrong_escrow_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<UnblockTokenExtensionFixture>(&mut ctx);
}

#[test]
fn test_unblock_token_extension_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<UnblockTokenExtensionFixture>(&mut ctx, 5, InstructionError::Custom(2));
}

#[test]
fn test_unblock_token_extension_invalid_extensions_bump() {
    let mut ctx = TestContext::new();
    let test_ix = UnblockTokenExtensionFixture::build_valid(&mut ctx);
    let correct_bump = test_ix.instruction.data[1];
    let invalid_bump = correct_bump.wrapping_add(1);
    let error = test_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_unblock_token_extension_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<UnblockTokenExtensionFixture>(&mut ctx);
}

#[test]
fn test_unblock_token_extension_truncated_data() {
    let mut ctx = TestContext::new();
    test_truncated_data::<UnblockTokenExtensionFixture>(&mut ctx);
}

// ============================================================================
// Custom Error Tests
// ============================================================================

#[test]
fn test_unblock_token_extension_wrong_admin() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 1u16).send_expect_success(&mut ctx);

    let wrong_admin = ctx.create_funded_keypair();
    let test_ix = UnblockTokenExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, wrong_admin, 1u16);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(1));
}

#[test]
fn test_unblock_token_extension_escrow_not_owned_by_program() {
    let mut ctx = TestContext::new();
    let test_ix = UnblockTokenExtensionFixture::build_valid(&mut ctx);

    let error = test_ix.with_account_at(2, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_unblock_token_extension_fails_when_escrow_is_immutable() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 1u16)
        .send_expect_success(&mut ctx);

    let set_immutable_ix = SetImmutableFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone());
    set_immutable_ix.send_expect_success(&mut ctx);

    let unblock_ix = UnblockTokenExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 1u16);
    let error = unblock_ix.send_expect_error(&mut ctx);
    assert_escrow_error(error, EscrowError::EscrowImmutable);
}

#[test]
fn test_unblock_token_extension_not_blocked_when_extensions_missing() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let test_ix = UnblockTokenExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 1u16);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_escrow_error(error, EscrowError::TokenExtensionNotBlocked);
}

#[test]
fn test_unblock_token_extension_not_blocked_when_target_missing() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 1u16)
        .send_expect_success(&mut ctx);

    let test_ix = UnblockTokenExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 2u16);
    let error = test_ix.send_expect_error(&mut ctx);
    assert_escrow_error(error, EscrowError::TokenExtensionNotBlocked);
}

// ============================================================================
// Success Tests
// ============================================================================

#[test]
fn test_unblock_token_extension_success_remove_single() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 1u16)
        .send_expect_success(&mut ctx);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &[1u16]);

    let unblock_ix = UnblockTokenExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 1u16);
    unblock_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 0);
    assert_extension_missing(&ctx, &extensions_pda, EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS);
}

#[test]
fn test_unblock_token_extension_success_remove_middle() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 1u16)
        .send_expect_success(&mut ctx);
    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 2u16)
        .send_expect_success(&mut ctx);
    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 3u16)
        .send_expect_success(&mut ctx);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &[1u16, 2u16, 3u16]);

    let unblock_ix = UnblockTokenExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 2u16);
    unblock_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &[1u16, 3u16]);
}

#[test]
fn test_unblock_token_extension_success_remove_last_keeps_other_extensions() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 3600)
        .send_expect_success(&mut ctx);
    AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 16u16)
        .send_expect_success(&mut ctx);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 2);

    let unblock_ix = UnblockTokenExtensionFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 16u16);
    unblock_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_extension_missing(&ctx, &extensions_pda, EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS);
    assert_timelock_extension(&ctx, &extensions_pda, 3600);
}
