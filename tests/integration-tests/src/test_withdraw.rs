use crate::{
    fixtures::{AllowMintSetup, WithdrawFixture, WithdrawSetup, DEFAULT_DEPOSIT_AMOUNT},
    utils::{
        assert_custom_error, assert_escrow_error, test_missing_signer, test_not_writable, test_wrong_account,
        test_wrong_current_program, test_wrong_owner, test_wrong_system_program, test_wrong_token_program, EscrowError,
        TestContext, TEST_HOOK_ALLOW_ID, TEST_HOOK_DENY_ERROR, TEST_HOOK_DENY_ID,
    },
};
use escrow_program_client::instructions::WithdrawBuilder;
use solana_sdk::{account::Account, instruction::InstructionError, pubkey::Pubkey, signature::Signer};

// ============================================================================
// Error Tests - Using Generic Test Helpers
// ============================================================================

#[test]
fn test_withdraw_missing_withdrawer_signer() {
    let mut ctx = TestContext::new();
    test_missing_signer::<WithdrawFixture>(&mut ctx, 2, 0);
}

#[test]
fn test_withdraw_receipt_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<WithdrawFixture>(&mut ctx, 5);
}

#[test]
fn test_withdraw_vault_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<WithdrawFixture>(&mut ctx, 6);
}

#[test]
fn test_withdraw_withdrawer_token_account_not_writable() {
    let mut ctx = TestContext::new();
    test_not_writable::<WithdrawFixture>(&mut ctx, 7);
}

#[test]
fn test_withdraw_wrong_system_program() {
    let mut ctx = TestContext::new();
    test_wrong_system_program::<WithdrawFixture>(&mut ctx);
}

#[test]
fn test_withdraw_wrong_current_program() {
    let mut ctx = TestContext::new();
    test_wrong_current_program::<WithdrawFixture>(&mut ctx);
}

#[test]
fn test_withdraw_invalid_event_authority() {
    let mut ctx = TestContext::new();
    test_wrong_account::<WithdrawFixture>(&mut ctx, 11, InstructionError::Custom(2));
}

#[test]
fn test_withdraw_wrong_token_program() {
    let mut ctx = TestContext::new();
    test_wrong_token_program::<WithdrawFixture>(&mut ctx, 9);
}

#[test]
fn test_withdraw_wrong_escrow_owner() {
    let mut ctx = TestContext::new();
    test_wrong_owner::<WithdrawFixture>(&mut ctx, 3);
}

#[test]
fn test_withdraw_wrong_receipt_owner() {
    let mut ctx = TestContext::new();
    test_wrong_owner::<WithdrawFixture>(&mut ctx, 5);
}

// ============================================================================
// Custom Error Tests
// ============================================================================

#[test]
fn test_withdraw_wrong_withdrawer() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let wrong_withdrawer = ctx.create_funded_keypair();
    let wrong_withdrawer_token_account = ctx.create_token_account(&wrong_withdrawer.pubkey(), &setup.mint.pubkey());

    let instruction = WithdrawBuilder::new()
        .payer(ctx.payer.pubkey())
        .withdrawer(wrong_withdrawer.pubkey())
        .escrow(setup.escrow_pda)
        .extensions(setup.extensions_pda)
        .receipt(setup.receipt_pda)
        .vault(setup.vault)
        .withdrawer_token_account(wrong_withdrawer_token_account)
        .mint(setup.mint.pubkey())
        .token_program(setup.token_program)
        .rent_recipient(ctx.payer.pubkey())
        .instruction();

    let error = ctx.send_transaction_expect_error(instruction, &[&wrong_withdrawer]);
    assert_escrow_error(error, EscrowError::InvalidWithdrawer);
}

// ============================================================================
// Timelock Tests
// ============================================================================

#[test]
fn test_withdraw_timelock_not_expired() {
    let mut ctx = TestContext::new();
    let lock_duration = 3600;
    let setup = WithdrawSetup::new_with_timelock(&mut ctx, lock_duration);

    let test_ix = setup.build_instruction(&ctx);
    let error = test_ix.send_expect_error(&mut ctx);
    assert_escrow_error(error, EscrowError::TimelockNotExpired);
}

#[test]
fn test_withdraw_timelock_expired_success() {
    let mut ctx = TestContext::new();
    let lock_duration = 3600;
    let setup = WithdrawSetup::new_with_timelock(&mut ctx, lock_duration);

    let current_time = ctx.get_current_timestamp();
    ctx.warp_to_timestamp(current_time + lock_duration as i64 + 1);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed");
}

#[test]
fn test_withdraw_no_timelock_success() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed");
}

#[test]
fn test_withdraw_timelock_zero_duration() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new_with_timelock(&mut ctx, 0);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed");
}

// ============================================================================
// Happy Path Tests
// ============================================================================

#[test]
fn test_withdraw_success() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let initial_withdrawer_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let initial_vault_balance = ctx.get_token_balance(&setup.vault);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let final_withdrawer_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let final_vault_balance = ctx.get_token_balance(&setup.vault);

    assert_eq!(final_withdrawer_balance, initial_withdrawer_balance + DEFAULT_DEPOSIT_AMOUNT);
    assert_eq!(final_vault_balance, initial_vault_balance - DEFAULT_DEPOSIT_AMOUNT);
}

#[test]
fn test_withdraw_closes_receipt() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    assert!(ctx.get_account(&setup.receipt_pda).is_some(), "Receipt should exist before withdraw");

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed after withdraw");
}

#[test]
fn test_withdraw_transfers_tokens() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let vault_balance_before = ctx.get_token_balance(&setup.vault);
    let withdrawer_balance_before = ctx.get_token_balance(&setup.depositor_token_account);

    assert_eq!(vault_balance_before, DEFAULT_DEPOSIT_AMOUNT);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let vault_balance_after = ctx.get_token_balance(&setup.vault);
    let withdrawer_balance_after = ctx.get_token_balance(&setup.depositor_token_account);

    assert_eq!(vault_balance_after, 0);
    assert_eq!(withdrawer_balance_after, withdrawer_balance_before + DEFAULT_DEPOSIT_AMOUNT);
}

#[test]
fn test_withdraw_returns_rent_to_payer() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let payer_balance_before = ctx.get_account(&ctx.payer.pubkey()).unwrap().lamports;
    let receipt_rent = ctx.get_account(&setup.receipt_pda).unwrap().lamports;

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let payer_balance_after = ctx.get_account(&ctx.payer.pubkey()).unwrap().lamports;

    assert!(payer_balance_after > payer_balance_before, "Payer should receive rent back (after accounting for tx fee)");
    assert!(
        payer_balance_after >= payer_balance_before + receipt_rent - 10000,
        "Payer balance should increase by approximately receipt rent minus tx fee"
    );
}

#[test]
fn test_withdraw_returns_rent_to_custom_recipient() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let custom_recipient = Pubkey::new_unique();
    ctx.svm.set_account(custom_recipient, Account { lamports: 1_000_000, ..Account::default() }).unwrap();

    let recipient_balance_before = ctx.get_account(&custom_recipient).unwrap().lamports;
    let receipt_rent = ctx.get_account(&setup.receipt_pda).unwrap().lamports;

    let test_ix = setup.build_instruction_with_rent_recipient(&ctx, custom_recipient);
    test_ix.send_expect_success(&mut ctx);

    let recipient_balance_after = ctx.get_account(&custom_recipient).unwrap().lamports;

    assert_eq!(
        recipient_balance_after,
        recipient_balance_before + receipt_rent,
        "Custom rent recipient should receive exact receipt rent"
    );
}

// ============================================================================
// Token 2022 Happy Path Tests
// ============================================================================

#[test]
fn test_withdraw_token_2022_success() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new_token_2022(&mut ctx);

    let initial_withdrawer_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let initial_vault_balance = ctx.get_token_balance(&setup.vault);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let final_withdrawer_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let final_vault_balance = ctx.get_token_balance(&setup.vault);

    assert_eq!(final_withdrawer_balance, initial_withdrawer_balance + DEFAULT_DEPOSIT_AMOUNT);
    assert_eq!(final_vault_balance, initial_vault_balance - DEFAULT_DEPOSIT_AMOUNT);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed after withdraw");
}

// ============================================================================
// Hook Program Tests
// ============================================================================

#[test]
fn test_withdraw_with_hook_success() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new_with_hook(&mut ctx, TEST_HOOK_ALLOW_ID);

    let initial_withdrawer_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let initial_vault_balance = ctx.get_token_balance(&setup.vault);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    let final_withdrawer_balance = ctx.get_token_balance(&setup.depositor_token_account);
    let final_vault_balance = ctx.get_token_balance(&setup.vault);

    assert_eq!(final_withdrawer_balance, initial_withdrawer_balance + DEFAULT_DEPOSIT_AMOUNT);
    assert_eq!(final_vault_balance, initial_vault_balance - DEFAULT_DEPOSIT_AMOUNT);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed after withdraw");
}

#[test]
fn test_withdraw_with_hook_rejected() {
    let mut ctx = TestContext::new();

    let mut setup = WithdrawSetup::new(&mut ctx);
    setup.set_hook(&mut ctx, TEST_HOOK_DENY_ID);

    let initial_vault_balance = ctx.get_token_balance(&setup.vault);

    let test_ix = setup.build_instruction(&ctx);
    let error = test_ix.send_expect_error(&mut ctx);

    assert_custom_error(error, TEST_HOOK_DENY_ERROR);

    assert!(ctx.get_account(&setup.receipt_pda).is_some(), "Receipt should still exist after rejected withdraw");

    let final_vault_balance = ctx.get_token_balance(&setup.vault);
    assert_eq!(final_vault_balance, initial_vault_balance, "Vault balance should be unchanged");
}

// ============================================================================
// Cross-Escrow Protection Tests
// ============================================================================

#[test]
fn test_withdraw_receipt_for_different_escrow_fails() {
    let mut ctx = TestContext::new();

    let setup_a = WithdrawSetup::new(&mut ctx);

    let setup_b =
        AllowMintSetup::builder(&mut ctx).with_existing_mint(setup_a.mint.pubkey(), setup_a.token_program).build();
    setup_b.build_instruction(&ctx).send_expect_success(&mut ctx);

    let vault_b = ctx.create_token_account(&setup_b.escrow_pda, &setup_a.mint.pubkey());

    let instruction = WithdrawBuilder::new()
        .payer(ctx.payer.pubkey())
        .withdrawer(setup_a.depositor.pubkey())
        .escrow(setup_b.escrow_pda)
        .extensions(setup_b.escrow_extensions_pda)
        .receipt(setup_a.receipt_pda)
        .vault(vault_b)
        .withdrawer_token_account(setup_a.depositor_token_account)
        .mint(setup_a.mint.pubkey())
        .token_program(setup_a.token_program)
        .rent_recipient(ctx.payer.pubkey())
        .instruction();

    let error = ctx.send_transaction_expect_error(instruction, &[&setup_a.depositor]);

    assert_escrow_error(error, EscrowError::InvalidReceiptEscrow);
}

// ============================================================================
// Closed Account Protection Tests
// ============================================================================

#[test]
fn test_withdraw_double_withdraw_fails() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed");

    ctx.warp_to_slot(2);

    let instruction = WithdrawBuilder::new()
        .payer(ctx.payer.pubkey())
        .withdrawer(setup.depositor.pubkey())
        .escrow(setup.escrow_pda)
        .extensions(setup.extensions_pda)
        .receipt(setup.receipt_pda)
        .vault(setup.vault)
        .withdrawer_token_account(setup.depositor_token_account)
        .mint(setup.mint.pubkey())
        .token_program(setup.token_program)
        .rent_recipient(ctx.payer.pubkey())
        .instruction();

    let error = ctx.send_transaction_expect_error(instruction, &[&setup.depositor]);
    match error {
        solana_sdk::transaction::TransactionError::InstructionError(_, InstructionError::InvalidAccountOwner) => {}
        other => panic!("Expected InvalidAccountOwner, got {:?}", other),
    }
}

#[test]
fn test_withdraw_rejects_reactivated_account_wrong_owner() {
    let mut ctx = TestContext::new();
    let setup = WithdrawSetup::new(&mut ctx);

    let test_ix = setup.build_instruction(&ctx);
    test_ix.send_expect_success(&mut ctx);

    assert!(ctx.get_account(&setup.receipt_pda).is_none(), "Receipt should be closed");

    ctx.svm
        .set_account(
            setup.receipt_pda,
            Account {
                lamports: 1_000_000,
                data: vec![0u8; 128],
                owner: Pubkey::new_unique(),
                executable: false,
                rent_epoch: 0,
            },
        )
        .unwrap();

    ctx.warp_to_slot(2);

    let instruction = WithdrawBuilder::new()
        .payer(ctx.payer.pubkey())
        .withdrawer(setup.depositor.pubkey())
        .escrow(setup.escrow_pda)
        .extensions(setup.extensions_pda)
        .receipt(setup.receipt_pda)
        .vault(setup.vault)
        .withdrawer_token_account(setup.depositor_token_account)
        .mint(setup.mint.pubkey())
        .token_program(setup.token_program)
        .rent_recipient(ctx.payer.pubkey())
        .instruction();

    let error = ctx.send_transaction_expect_error(instruction, &[&setup.depositor]);
    assert!(
        matches!(
            error,
            solana_sdk::transaction::TransactionError::InstructionError(_, InstructionError::InvalidAccountOwner)
        ),
        "Expected InvalidAccountOwner, got {:?}",
        error
    );
}
