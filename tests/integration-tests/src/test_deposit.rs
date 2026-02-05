use crate::{
    fixtures::{DepositFixture, DepositSetup, DEFAULT_DEPOSIT_AMOUNT},
    utils::{
        assert_custom_error, assert_escrow_error, assert_instruction_error, find_receipt_pda, test_empty_data,
        test_missing_signer, test_not_writable, test_wrong_account, test_wrong_current_program, test_wrong_owner,
        test_wrong_system_program, test_wrong_token_program, EscrowError, TestContext, TestInstruction,
        TEST_HOOK_ALLOW_ID, TEST_HOOK_DENY_ERROR, TEST_HOOK_DENY_ID,
    },
};
use escrow_program_client::instructions::DepositBuilder;
use solana_sdk::{instruction::InstructionError, pubkey::Pubkey, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_deposit_missing_depositor_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<DepositFixture>(&mut ctx, 1, 0);
}

#[test]
fn test_deposit_missing_receipt_seed_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<DepositFixture>(&mut ctx, 4, 1);
}

#[test]
fn test_receipt_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<DepositFixture>(&mut ctx, 5);
}

#[test]
fn test_deposit_vault_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<DepositFixture>(&mut ctx, 6);
}

#[test]
fn test_deposit_depositor_token_account_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<DepositFixture>(&mut ctx, 7);
}

#[test]
fn test_deposit_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<DepositFixture>(&mut ctx);
}

#[test]
fn test_deposit_wrong_current_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<DepositFixture>(&mut ctx);
}

#[test]
fn test_deposit_empty_data() {
    let mut ctx = TestContext::new();
    test_empty_data::<DepositFixture>(&mut ctx);
}

#[test]
fn test_deposit_invalid_bump() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new(&mut ctx);
    let valid_ix = setup.build_instruction(&ctx);
    let correct_bump = valid_ix.instruction.data[1];
    let invalid_bump = correct_bump.wrapping_add(1);

    let error = valid_ix.with_data_byte_at(1, invalid_bump).send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidSeeds);
}

#[test]
fn test_deposit_wrong_token_program() {
    let mut ctx = TestContext::new();
    test_wrong_token_program::<DepositFixture>(&mut ctx, 9);
}

#[test]
fn test_deposit_wrong_escrow_owner() {
    let mut ctx = TestContext::new();
    test_wrong_owner::<DepositFixture>(&mut ctx, 2);
}

#[test]
fn test_deposit_wrong_allowed_mint_owner() {
    let mut ctx = TestContext::new();
    test_wrong_owner::<DepositFixture>(&mut ctx, 3);
}

#[test]
fn test_deposit_zero_amount() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new(&mut ctx);

    // Build instruction with zero amount
    let instruction = DepositBuilder::new()
        .payer(ctx.payer.pubkey())
        .depositor(setup.depositor.pubkey())
        .escrow(setup.escrow_pda)
        .allowed_mint(setup.allowed_mint_pda)
        .receipt_seed(setup.receipt_seed.pubkey())
        .receipt(setup.receipt_pda)
        .vault(setup.vault)
        .depositor_token_account(setup.depositor_token_account)
        .mint(setup.mint.pubkey())
        .token_program(setup.token_program)
        .extensions(setup.extensions_pda)
        .bump(setup.bump)
        .amount(0) // Zero amount
        .instruction();

    let test_ix = TestInstruction {
        instruction,
        signers: vec![setup.depositor.insecure_clone(), setup.receipt_seed.insecure_clone()],
        name: "Deposit",
    };

    let error = test_ix.send_expect_error(&mut ctx);
    // ZeroDepositAmount = error code 13
    assert_escrow_error(error, EscrowError::ZeroDepositAmount);
}

#[test]
fn test_deposit_invalid_event_authority() {
    let mut ctx = TestContext::new();
    // event_authority is at index 11 in instruction accounts
    // Custom error 2 = InvalidEventAuthority
    test_wrong_account::<DepositFixture>(&mut ctx, 11, InstructionError::Custom(2));
}

#[test]
fn test_deposit_wrong_vault_ata() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new(&mut ctx);

    // Create a token account at a wrong address (not the escrow's ATA)
    let wrong_vault = Pubkey::new_unique();
    ctx.create_token_account_at_address(&wrong_vault, &setup.escrow_pda, &setup.mint.pubkey(), 0);

    let instruction = DepositBuilder::new()
        .payer(ctx.payer.pubkey())
        .depositor(setup.depositor.pubkey())
        .escrow(setup.escrow_pda)
        .allowed_mint(setup.allowed_mint_pda)
        .receipt_seed(setup.receipt_seed.pubkey())
        .receipt(setup.receipt_pda)
        .vault(wrong_vault) // Wrong vault address
        .depositor_token_account(setup.depositor_token_account)
        .mint(setup.mint.pubkey())
        .token_program(setup.token_program)
        .extensions(setup.extensions_pda)
        .bump(setup.bump)
        .amount(DEFAULT_DEPOSIT_AMOUNT)
        .instruction();

    let test_ix = TestInstruction {
        instruction,
        signers: vec![setup.depositor.insecure_clone(), setup.receipt_seed.insecure_clone()],
        name: "Deposit",
    };

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountData);
}

#[test]
fn test_deposit_wrong_depositor_token_account_owner() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new(&mut ctx);

    // Create a token account owned by a different owner at a random address
    let wrong_owner = Pubkey::new_unique();
    let wrong_token_account = Pubkey::new_unique();
    ctx.create_token_account_at_address(
        &wrong_token_account,
        &wrong_owner,
        &setup.mint.pubkey(),
        DEFAULT_DEPOSIT_AMOUNT,
    );

    let instruction = DepositBuilder::new()
        .payer(ctx.payer.pubkey())
        .depositor(setup.depositor.pubkey())
        .escrow(setup.escrow_pda)
        .allowed_mint(setup.allowed_mint_pda)
        .receipt_seed(setup.receipt_seed.pubkey())
        .receipt(setup.receipt_pda)
        .vault(setup.vault)
        .depositor_token_account(wrong_token_account) // Wrong depositor token account
        .mint(setup.mint.pubkey())
        .token_program(setup.token_program)
        .extensions(setup.extensions_pda)
        .bump(setup.bump)
        .amount(DEFAULT_DEPOSIT_AMOUNT)
        .instruction();

    let test_ix = TestInstruction {
        instruction,
        signers: vec![setup.depositor.insecure_clone(), setup.receipt_seed.insecure_clone()],
        name: "Deposit",
    };

    let error = test_ix.send_expect_error(&mut ctx);
    assert_instruction_error(error, InstructionError::InvalidAccountData);
}

// ============================================================================
// Happy Path Test
// ============================================================================

#[test]
fn test_deposit_success() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new(&mut ctx);

    let initial_depositor_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let initial_vault_balance = ctx.get_token_balance(&setup.vault);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let final_depositor_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let final_vault_balance = ctx.get_token_balance(&setup.vault);

    assert_eq!(final_depositor_balance, initial_depositor_balance - DEFAULT_DEPOSIT_AMOUNT);
    assert_eq!(final_vault_balance, initial_vault_balance + DEFAULT_DEPOSIT_AMOUNT);

    let receipt_account = ctx.get_account(&setup.receipt_pda).expect("Deposit receipt should exist");
    assert!(!receipt_account.data.is_empty());
}

#[test]
fn test_deposit_multiple_deposits() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new(&mut ctx);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let new_receipt_seed = solana_sdk::signature::Keypair::new();
    let (new_receipt_pda, new_bump) = find_receipt_pda(
        &setup.escrow_pda,
        &setup.depositor.pubkey(),
        &setup.mint.pubkey(),
        &new_receipt_seed.pubkey(),
    );

    let second_deposit_ix = DepositBuilder::new()
        .payer(ctx.payer.pubkey())
        .depositor(setup.depositor.pubkey())
        .escrow(setup.escrow_pda)
        .allowed_mint(setup.allowed_mint_pda)
        .receipt_seed(new_receipt_seed.pubkey())
        .receipt(new_receipt_pda)
        .vault(setup.vault)
        .depositor_token_account(setup.depositor_token_account)
        .mint(setup.mint.pubkey())
        .extensions(setup.extensions_pda)
        .bump(new_bump)
        .amount(DEFAULT_DEPOSIT_AMOUNT / 2)
        .instruction();

    ctx.send_transaction(second_deposit_ix, &[&setup.depositor, &new_receipt_seed]).unwrap();

    let first_receipt = ctx.get_account(&setup.receipt_pda).expect("First receipt should exist");
    let second_receipt = ctx.get_account(&new_receipt_pda).expect("Second receipt should exist");

    assert!(!first_receipt.data.is_empty());
    assert!(!second_receipt.data.is_empty());
}

// ============================================================================
// Token 2022 Happy Path Tests
// ============================================================================

#[test]
fn test_deposit_token_2022_success() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new_token_2022(&mut ctx);

    let initial_depositor_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let initial_vault_balance = ctx.get_token_balance(&setup.vault);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let final_depositor_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let final_vault_balance = ctx.get_token_balance(&setup.vault);

    assert_eq!(final_depositor_balance, initial_depositor_balance - DEFAULT_DEPOSIT_AMOUNT);
    assert_eq!(final_vault_balance, initial_vault_balance + DEFAULT_DEPOSIT_AMOUNT);

    let receipt_account = ctx.get_account(&setup.receipt_pda).expect("Deposit receipt should exist");
    assert!(!receipt_account.data.is_empty());
}

// ============================================================================
// Hook Program Tests
// ============================================================================

/// Happy path: Deposit with hook program that allows
#[test]
fn test_deposit_with_hook_success() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new_with_hook(&mut ctx, TEST_HOOK_ALLOW_ID);

    let initial_depositor_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let initial_vault_balance = ctx.get_token_balance(&setup.vault);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let final_depositor_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let final_vault_balance = ctx.get_token_balance(&setup.vault);

    assert_eq!(final_depositor_balance, initial_depositor_balance - DEFAULT_DEPOSIT_AMOUNT);
    assert_eq!(final_vault_balance, initial_vault_balance + DEFAULT_DEPOSIT_AMOUNT);

    let receipt_account = ctx.get_account(&setup.receipt_pda).expect("Deposit receipt should exist");
    assert!(!receipt_account.data.is_empty());
}

/// Sad path: Deposit with hook program that rejects
#[test]
fn test_deposit_with_hook_rejected() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new_with_hook(&mut ctx, TEST_HOOK_DENY_ID);

    let test_ix = setup.build_instruction(&ctx);
    let error = test_ix.send_expect_error(&mut ctx);

    assert_custom_error(error, TEST_HOOK_DENY_ERROR);
}

// ============================================================================
// Additional Tests
// ============================================================================

#[test]
fn test_deposit_allowed_mint_not_owned_by_program() {
    let mut ctx = TestContext::new();
    let setup = DepositSetup::new(&mut ctx);

    // Use a wrong AllowedMint PDA (random pubkey, not owned by program)
    let wrong_allowed_mint = Pubkey::new_unique();

    let instruction = DepositBuilder::new()
        .payer(ctx.payer.pubkey())
        .depositor(setup.depositor.pubkey())
        .escrow(setup.escrow_pda)
        .allowed_mint(wrong_allowed_mint) // Invalid AllowedMint PDA
        .receipt_seed(setup.receipt_seed.pubkey())
        .receipt(setup.receipt_pda)
        .vault(setup.vault)
        .depositor_token_account(setup.depositor_token_account)
        .mint(setup.mint.pubkey())
        .token_program(setup.token_program)
        .extensions(setup.extensions_pda)
        .bump(setup.bump)
        .amount(DEFAULT_DEPOSIT_AMOUNT)
        .instruction();

    let test_ix = TestInstruction {
        instruction,
        signers: vec![setup.depositor.insecure_clone(), setup.receipt_seed.insecure_clone()],
        name: "Deposit",
    };

    let error = test_ix.send_expect_error(&mut ctx);
    // Wrong AllowedMint address (not owned by program) should fail with InvalidAccountOwner
    assert_instruction_error(error, InstructionError::InvalidAccountOwner);
}
