use crate::{
    fixtures::{AllowMintFixture, AllowMintSetup},
    utils::{
        assert_account_exists, assert_allowed_mint_account, assert_escrow_error, assert_instruction_error,
        find_allowed_mint_pda, test_missing_signer, test_not_writable, test_wrong_current_program,
        test_wrong_system_program, EscrowError, InstructionTestFixture, TestContext, RANDOM_PUBKEY,
    },
};
use escrow_program_client::instructions::AllowMintBuilder;
use solana_sdk::{instruction::InstructionError, signature::Signer};
use spl_associated_token_account::get_associated_token_address;
use spl_token_2022::extension::ExtensionType;

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_allow_mint_missing_admin_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<AllowMintFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_allow_mint_allowed_mint_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<AllowMintFixture>(&mut ctx, 5);
}

#[test]
fn test_allow_mint_vault_not_writable() {
    let mut ctx = TestContext::new();
    // vault is at index 6 in instruction accounts
    test_not_writable::<AllowMintFixture>(&mut ctx, 6);
}

#[test]
fn test_allow_mint_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<AllowMintFixture>(&mut ctx);
}

#[test]
fn test_allow_mint_wrong_current_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<AllowMintFixture>(&mut ctx);
}

#[test]
fn test_allow_mint_invalid_event_authority() {
    let mut ctx = TestContext::new();
    let error = AllowMintFixture::build_valid(&mut ctx).with_account_at(10, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_escrow_error(error, EscrowError::InvalidEventAuthority);
}

#[test]
fn test_allow_mint_invalid_bump() {
    let mut ctx = TestContext::new();
    let valid_ix = AllowMintFixture::build_valid(&mut ctx);
    let correct_bump = valid_ix.instruction.data[1];
    let invalid_bump = correct_bump.wrapping_add(1);

    let error = valid_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_allow_mint_wrong_admin() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new(&mut ctx);

    let wrong_admin = ctx.create_funded_keypair();

    let instruction = AllowMintBuilder::new()
        .payer(ctx.payer.pubkey())
        .admin(wrong_admin.pubkey())
        .escrow(setup.escrow_pda)
        .escrow_extensions(setup.escrow_extensions_pda)
        .mint(setup.mint_pubkey)
        .allowed_mint(setup.allowed_mint_pda)
        .vault(setup.vault)
        .token_program(setup.token_program)
        .bump(setup.allowed_mint_bump)
        .instruction();

    let test_ix = crate::utils::TestInstruction { instruction, signers: vec![wrong_admin], name: "AllowMint" };

    let error = test_ix.send_expect_error(&mut ctx);
    assert_escrow_error(error, EscrowError::InvalidAdmin);
}

#[test]
fn test_allow_mint_wrong_escrow() {
    let mut ctx = TestContext::new();
    let error = AllowMintFixture::build_valid(&mut ctx).with_account_at(2, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_allow_mint_wrong_mint() {
    let mut ctx = TestContext::new();
    let error = AllowMintFixture::build_valid(&mut ctx).with_account_at(4, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}

#[test]
fn test_allow_mint_wrong_token_program() {
    let mut ctx = TestContext::new();
    let error = AllowMintFixture::build_valid(&mut ctx).with_account_at(7, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::IncorrectProgramId);
}

#[test]
fn test_allow_mint_wrong_associated_token_program() {
    let mut ctx = TestContext::new();
    // associated_token_program is at index 8 in instruction accounts
    let error = AllowMintFixture::build_valid(&mut ctx).with_account_at(8, RANDOM_PUBKEY).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::IncorrectProgramId);
}

#[test]
fn test_allow_mint_duplicate() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new(&mut ctx);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let duplicate_ix = setup.build_instruction(&ctx);
    let error = duplicate_ix.send_expect_error(&mut ctx);
    assert!(matches!(error, solana_sdk::transaction::TransactionError::AlreadyProcessed));
}

// ============================================================================
// Happy Path Test
// ============================================================================

#[test]
fn test_allow_mint_success() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new(&mut ctx);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert_account_exists(&ctx, &setup.allowed_mint_pda);
    assert_allowed_mint_account(&ctx, &setup.allowed_mint_pda, setup.allowed_mint_bump);
}

#[test]
fn test_allow_mint_multiple_mints() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new(&mut ctx);

    let first_ix = setup.build_instruction(&ctx);
    first_ix.send_expect_success(&mut ctx);

    let second_mint = solana_sdk::signature::Keypair::new();
    ctx.create_mint(&second_mint, &ctx.payer.pubkey(), 9);

    let (second_allowed_mint_pda, second_bump) = find_allowed_mint_pda(&setup.escrow_pda, &second_mint.pubkey());
    let second_vault = get_associated_token_address(&setup.escrow_pda, &second_mint.pubkey());

    let instruction = AllowMintBuilder::new()
        .payer(ctx.payer.pubkey())
        .admin(setup.admin.pubkey())
        .escrow(setup.escrow_pda)
        .escrow_extensions(setup.escrow_extensions_pda)
        .mint(second_mint.pubkey())
        .allowed_mint(second_allowed_mint_pda)
        .vault(second_vault)
        .token_program(setup.token_program)
        .bump(second_bump)
        .instruction();

    let second_ix =
        crate::utils::TestInstruction { instruction, signers: vec![setup.admin.insecure_clone()], name: "AllowMint" };
    second_ix.send_expect_success(&mut ctx);

    assert_account_exists(&ctx, &setup.allowed_mint_pda);
    assert_account_exists(&ctx, &second_allowed_mint_pda);
}

// ============================================================================
// Token 2022 Happy Path Tests
// ============================================================================

#[test]
fn test_allow_mint_token_2022_success() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new_token_2022(&mut ctx);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert_account_exists(&ctx, &setup.allowed_mint_pda);
    assert_allowed_mint_account(&ctx, &setup.allowed_mint_pda, setup.allowed_mint_bump);
}

// ============================================================================
// Token 2022 Blocked Extension Tests
// ============================================================================

#[test]
fn test_allow_mint_rejects_permanent_delegate() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new_with_extension(&mut ctx, ExtensionType::PermanentDelegate);

    let test_ix = setup.build_instruction(&ctx);
    let error = test_ix.send_expect_error(&mut ctx);

    assert_escrow_error(error, EscrowError::PermanentDelegateNotAllowed);
}

#[test]
fn test_allow_mint_rejects_non_transferable() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new_with_extension(&mut ctx, ExtensionType::NonTransferable);

    let test_ix = setup.build_instruction(&ctx);
    let error = test_ix.send_expect_error(&mut ctx);

    assert_escrow_error(error, EscrowError::NonTransferableNotAllowed);
}

#[test]
fn test_allow_mint_rejects_pausable() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new_with_extension(&mut ctx, ExtensionType::Pausable);

    let test_ix = setup.build_instruction(&ctx);
    let error = test_ix.send_expect_error(&mut ctx);

    assert_escrow_error(error, EscrowError::PausableNotAllowed);
}

// ============================================================================
// Escrow-Specific Blocked Extension Tests
// ============================================================================

#[test]
fn test_allow_mint_rejects_escrow_blocked_extension() {
    let mut ctx = TestContext::new();

    // Create escrow with TransferFeeConfig blocked, then try to allow a mint with that extension
    let setup = AllowMintSetup::new_with_escrow_blocked_extension(&mut ctx, ExtensionType::TransferFeeConfig);

    let test_ix = setup.build_instruction(&ctx);
    let error = test_ix.send_expect_error(&mut ctx);

    assert_escrow_error(error, EscrowError::MintNotAllowed);
}

#[test]
fn test_allow_mint_accepts_mint_without_escrow_blocked_extension() {
    let mut ctx = TestContext::new();

    // Block TransferFeeConfig, but create a mint with MintCloseAuthority (different extension)
    let setup = AllowMintSetup::new_with_different_extension_blocked(
        &mut ctx,
        ExtensionType::TransferFeeConfig,
        ExtensionType::MintCloseAuthority,
    );

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);
}

// ============================================================================
// Vault ATA Creation Tests
// ============================================================================

#[test]
fn test_allow_mint_creates_vault_ata() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new(&mut ctx);

    // Verify vault ATA does not exist before allow_mint
    assert!(ctx.svm.get_account(&setup.vault).is_none());

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    // Verify vault ATA exists after allow_mint
    assert_account_exists(&ctx, &setup.vault);

    // Verify vault is owned by the token program
    let vault_account = ctx.svm.get_account(&setup.vault).unwrap();
    assert_eq!(vault_account.owner, setup.token_program);
}

#[test]
fn test_allow_mint_creates_vault_ata_token_2022() {
    let mut ctx = TestContext::new();
    let setup = AllowMintSetup::new_token_2022(&mut ctx);

    // Verify vault ATA does not exist before allow_mint
    assert!(ctx.svm.get_account(&setup.vault).is_none());

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    // Verify vault ATA exists after allow_mint
    assert_account_exists(&ctx, &setup.vault);

    // Verify vault is owned by Token-2022 program
    let vault_account = ctx.svm.get_account(&setup.vault).unwrap();
    assert_eq!(vault_account.owner, spl_token_2022::ID);
}
