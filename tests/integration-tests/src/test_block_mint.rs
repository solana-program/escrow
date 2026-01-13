use crate::{
    fixtures::{AllowMintSetup, BlockMintFixture, BlockMintSetup},
    utils::{
        assert_account_exists, assert_account_not_exists, assert_custom_error, assert_instruction_error,
        find_allowed_mint_pda, test_missing_signer, test_not_writable, test_wrong_current_program,
        InstructionTestFixture, TestContext, TestInstruction, RANDOM_PUBKEY,
    },
};
use escrow_program_client::instructions::{AllowMintBuilder, BlockMintBuilder};
use solana_sdk::{instruction::InstructionError, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_block_mint_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<BlockMintFixture>(&mut ctx, 0, 0);
}

#[test]
fn test_block_mint_allowed_mint_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<BlockMintFixture>(&mut ctx, 4);
}

#[test]
fn test_block_mint_wrong_current_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<BlockMintFixture>(&mut ctx);
}

#[test]
fn test_block_mint_invalid_event_authority() {
    let mut ctx = TestContext::new();
    let error = BlockMintFixture::build_valid(&mut ctx).with_account_at(6, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_custom_error(error, 2);
}

#[test]
fn test_block_mint_wrong_admin() {
    let mut ctx = TestContext::new();
    let setup = BlockMintSetup::new(&mut ctx);

    let wrong_admin = ctx.create_funded_keypair();

    let instruction = BlockMintBuilder::new()
        .admin(wrong_admin.pubkey())
        .payer(ctx.payer.pubkey())
        .escrow(setup.escrow_pda)
        .mint(setup.mint.pubkey())
        .allowed_mint(setup.allowed_mint_pda)
        .instruction();

    let test_ix = TestInstruction { instruction, signers: vec![wrong_admin], name: "BlockMint" };

    let error = test_ix.send_expect_error(&mut ctx);
    assert_custom_error(error, 1);
}

#[test]
fn test_block_mint_wrong_escrow() {
    let mut ctx = TestContext::new();
    let error = BlockMintFixture::build_valid(&mut ctx).with_account_at(2, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_block_mint_wrong_mint() {
    let mut ctx = TestContext::new();
    let error = BlockMintFixture::build_valid(&mut ctx).with_account_at(3, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_block_mint_wrong_allowed_mint() {
    let mut ctx = TestContext::new();
    let error = BlockMintFixture::build_valid(&mut ctx).with_account_at(4, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_block_mint_wrong_token_program() {
    let mut ctx = TestContext::new();
    let error = BlockMintFixture::build_valid(&mut ctx).with_account_at(5, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::IncorrectProgramId);
}

#[test]
fn test_block_mint_allowed_mint_escrow_mismatch() {
    let mut ctx = TestContext::new();

    let first_setup = AllowMintSetup::new(&mut ctx);
    first_setup.build_instruction(&ctx).send_expect_success(&mut ctx);

    let second_setup = AllowMintSetup::new(&mut ctx);
    second_setup.build_instruction(&ctx).send_expect_success(&mut ctx);

    let instruction = BlockMintBuilder::new()
        .admin(first_setup.admin.pubkey())
        .payer(ctx.payer.pubkey())
        .escrow(first_setup.escrow_pda)
        .mint(first_setup.mint.pubkey())
        .allowed_mint(second_setup.allowed_mint_pda)
        .instruction();

    let test_ix = TestInstruction { instruction, signers: vec![first_setup.admin.insecure_clone()], name: "BlockMint" };

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

// ============================================================================
// Happy Path Tests
// ============================================================================

#[test]
fn test_block_mint_success() {
    let mut ctx = TestContext::new();
    let setup = BlockMintSetup::new(&mut ctx);

    assert_account_exists(&ctx, &setup.allowed_mint_pda);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert_account_not_exists(&ctx, &setup.allowed_mint_pda);
}

#[test]
fn test_block_mint_rent_returned_to_payer() {
    let mut ctx = TestContext::new();
    let setup = BlockMintSetup::new(&mut ctx);

    let payer_balance_before = ctx.get_account(&ctx.payer.pubkey()).unwrap().lamports;

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let payer_balance_after = ctx.get_account(&ctx.payer.pubkey()).unwrap().lamports;
    assert!(payer_balance_after > payer_balance_before, "Payer should receive rent refund");
}

#[test]
fn test_block_multiple_mints_same_escrow() {
    let mut ctx = TestContext::new();

    let first_setup = AllowMintSetup::new(&mut ctx);
    first_setup.build_instruction(&ctx).send_expect_success(&mut ctx);

    let second_mint = solana_sdk::signature::Keypair::new();
    ctx.create_mint(&second_mint, &ctx.payer.pubkey(), 9);

    let (second_allowed_mint_pda, second_bump) = find_allowed_mint_pda(&first_setup.escrow_pda, &second_mint.pubkey());

    let allow_second_ix = AllowMintBuilder::new()
        .payer(ctx.payer.pubkey())
        .admin(first_setup.admin.pubkey())
        .escrow(first_setup.escrow_pda)
        .escrow_extensions(first_setup.escrow_extensions_pda)
        .mint(second_mint.pubkey())
        .allowed_mint(second_allowed_mint_pda)
        .bump(second_bump)
        .instruction();

    let allow_second_test_ix = TestInstruction {
        instruction: allow_second_ix,
        signers: vec![first_setup.admin.insecure_clone()],
        name: "AllowMint",
    };
    allow_second_test_ix.send_expect_success(&mut ctx);

    assert_account_exists(&ctx, &first_setup.allowed_mint_pda);
    assert_account_exists(&ctx, &second_allowed_mint_pda);

    let block_first_ix = BlockMintBuilder::new()
        .admin(first_setup.admin.pubkey())
        .payer(ctx.payer.pubkey())
        .escrow(first_setup.escrow_pda)
        .mint(first_setup.mint.pubkey())
        .allowed_mint(first_setup.allowed_mint_pda)
        .instruction();

    let block_first_test_ix = TestInstruction {
        instruction: block_first_ix,
        signers: vec![first_setup.admin.insecure_clone()],
        name: "BlockMint",
    };
    block_first_test_ix.send_expect_success(&mut ctx);

    assert_account_not_exists(&ctx, &first_setup.allowed_mint_pda);
    assert_account_exists(&ctx, &second_allowed_mint_pda);

    let block_second_ix = BlockMintBuilder::new()
        .admin(first_setup.admin.pubkey())
        .payer(ctx.payer.pubkey())
        .escrow(first_setup.escrow_pda)
        .mint(second_mint.pubkey())
        .allowed_mint(second_allowed_mint_pda)
        .instruction();

    let block_second_test_ix = TestInstruction {
        instruction: block_second_ix,
        signers: vec![first_setup.admin.insecure_clone()],
        name: "BlockMint",
    };
    block_second_test_ix.send_expect_success(&mut ctx);

    assert_account_not_exists(&ctx, &second_allowed_mint_pda);
}

// ============================================================================
// Token 2022 Happy Path Tests
// ============================================================================

#[test]
fn test_block_mint_token_2022_success() {
    let mut ctx = TestContext::new();
    let setup = BlockMintSetup::new_token_2022(&mut ctx);

    assert_account_exists(&ctx, &setup.allowed_mint_pda);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert_account_not_exists(&ctx, &setup.allowed_mint_pda);
}
