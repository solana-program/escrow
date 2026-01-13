use crate::{
    fixtures::{AddBlockTokenExtensionsFixture, CreateEscrowFixture},
    utils::{
        assert_block_token_extensions_extension, assert_extensions_header, assert_instruction_error, find_escrow_pda,
        find_extensions_pda, test_empty_data, test_missing_signer, test_not_writable, test_truncated_data,
        test_wrong_account, test_wrong_current_program, test_wrong_system_program, InstructionTestFixture, TestContext,
        RANDOM_PUBKEY,
    },
};
use solana_sdk::{instruction::InstructionError, signature::Signer};

#[test]
fn test_block_token_extension_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<AddBlockTokenExtensionsFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_block_token_extension_extensions_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<AddBlockTokenExtensionsFixture>(&mut ctx, 3);
}

#[test]
fn test_block_token_extension_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<AddBlockTokenExtensionsFixture>(&mut ctx);
}

#[test]
fn test_block_token_extension_wrong_escrow_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<AddBlockTokenExtensionsFixture>(&mut ctx);
}

#[test]
fn test_block_token_extension_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<AddBlockTokenExtensionsFixture>(&mut ctx, 5, InstructionError::Custom(2));
}

#[test]
fn test_block_token_extension_invalid_extensions_bump() {
    let mut ctx = TestContext::new();
    let test_ix = AddBlockTokenExtensionsFixture::build_valid(&mut ctx);
    // Instruction data includes discriminator at [0], so extensions_bump is at [1]
    let correct_bump = test_ix.instruction.data[1];
    let invalid_bump = correct_bump.wrapping_add(1);
    let error = test_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_block_token_extension_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<AddBlockTokenExtensionsFixture>(&mut ctx);
}

#[test]
fn test_block_token_extension_truncated_data() {
    let mut ctx = TestContext::new();
    test_truncated_data::<AddBlockTokenExtensionsFixture>(&mut ctx);
}

#[test]
fn test_block_token_extension_wrong_admin() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let wrong_admin = ctx.create_funded_keypair();
    let test_ix = AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, wrong_admin, 1u16);

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(1));
}

#[test]
fn test_block_token_extension_escrow_not_owned_by_program() {
    let mut ctx = TestContext::new();
    let test_ix = AddBlockTokenExtensionsFixture::build_valid(&mut ctx);

    let error = test_ix.with_account_at(2, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_block_token_extension_duplicate_extension() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    let first_ix =
        AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 1u16);
    first_ix.send_expect_success(&mut ctx);

    // Advance slot to get a new blockhash
    ctx.warp_to_slot(2);

    // Try to add the same extension again - should fail with TokenExtensionAlreadyBlocked (error code 12)
    let second_ix = AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 1u16);
    let error = second_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::Custom(12));
}

#[test]
fn test_block_token_extension_success() {
    let mut ctx = TestContext::new();
    let test_ix = AddBlockTokenExtensionsFixture::build_valid(&mut ctx);

    let extensions_pda = test_ix.instruction.accounts[3].pubkey;
    // Instruction data includes discriminator at [0], so extensions_bump is at [1]
    let extensions_bump = test_ix.instruction.data[1];
    // blocked_extension is at [2..4]
    let blocked_extension = u16::from_le_bytes([test_ix.instruction.data[2], test_ix.instruction.data[3]]);

    test_ix.send_expect_success(&mut ctx);

    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &[blocked_extension]);
}

#[test]
fn test_block_token_extension_success_single() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let test_ix = AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin, 42u16);
    test_ix.send_expect_success(&mut ctx);

    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &[42u16]);
}

#[test]
fn test_block_token_extension_success_many_extensions() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

    // Add 20 extensions one at a time (no max limit anymore)
    for i in 1..=20 {
        let test_ix =
            AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), i);
        test_ix.send_expect_success(&mut ctx);
    }

    let expected_extensions: Vec<u16> = (1..=20).collect();
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 1);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &expected_extensions);
}

#[test]
fn test_block_token_extension_success_multiple_extensions() {
    let mut ctx = TestContext::new();

    let escrow_ix = CreateEscrowFixture::build_valid(&mut ctx);
    let admin = escrow_ix.signers[0].insecure_clone();
    let escrow_seed = escrow_ix.signers[1].pubkey();
    escrow_ix.send_expect_success(&mut ctx);

    let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

    // Add timelock first
    use crate::fixtures::AddTimelockFixture;
    let timelock_ix = AddTimelockFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), 3600);
    timelock_ix.send_expect_success(&mut ctx);

    // Then add block token extensions one at a time
    let blocked_extensions = [1u16, 2u16, 3u16];
    for &ext in &blocked_extensions {
        let block_ext_ix =
            AddBlockTokenExtensionsFixture::build_with_escrow(&mut ctx, escrow_pda, admin.insecure_clone(), ext);
        block_ext_ix.send_expect_success(&mut ctx);
    }

    let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);
    assert_extensions_header(&ctx, &extensions_pda, extensions_bump, 2);
    assert_block_token_extensions_extension(&ctx, &extensions_pda, &blocked_extensions);
}
