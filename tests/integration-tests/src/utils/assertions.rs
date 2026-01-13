use crate::utils::extensions_utils::{
    block_token_extensions_byte_len, find_extension, ESCROW_EXTENSIONS_DISCRIMINATOR,
    EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS, EXTENSION_TYPE_HOOK, EXTENSION_TYPE_TIMELOCK, HOOK_DATA_LEN,
    TIMELOCK_DATA_LEN,
};
use crate::utils::TestContext;
use escrow_program_client::{
    accounts::{AllowedMint, Escrow},
    ESCROW_PROGRAM_ID,
};
use solana_sdk::{instruction::InstructionError, pubkey::Pubkey, transaction::TransactionError};

pub fn assert_account_exists(context: &TestContext, pubkey: &Pubkey) {
    let account = context.get_account(pubkey).unwrap_or_else(|| panic!("Account {pubkey} should exist"));
    assert!(!account.data.is_empty(), "Account data should not be empty");
}

pub fn assert_account_not_exists(context: &TestContext, pubkey: &Pubkey) {
    assert!(context.get_account(pubkey).is_none(), "Account {pubkey} should not exist");
}

/// Assert that a transaction error contains the expected instruction error
pub fn assert_instruction_error(tx_error: TransactionError, expected: InstructionError) {
    match tx_error {
        TransactionError::InstructionError(_, err) => {
            assert_eq!(err, expected, "Expected {expected:?}, got {err:?}");
        }
        other => panic!("Expected InstructionError, got {other:?}"),
    }
}

/// Assert that a transaction error is a custom program error with the given code
pub fn assert_custom_error(tx_error: TransactionError, expected_code: u32) {
    assert_instruction_error(tx_error, InstructionError::Custom(expected_code));
}

pub fn assert_escrow_account(
    context: &TestContext,
    escrow_pda: &Pubkey,
    expected_admin: &Pubkey,
    expected_bump: u8,
    expected_escrow_seed: &Pubkey,
) {
    let account = context.get_account(escrow_pda).expect("Escrow account should exist");

    assert_eq!(account.owner, ESCROW_PROGRAM_ID);

    let escrow = Escrow::from_bytes(&account.data).expect("Should deserialize escrow account");

    assert_eq!(escrow.admin.as_ref(), expected_admin.as_ref());
    assert_eq!(escrow.bump, expected_bump);
    assert_eq!(escrow.escrow_seed.as_ref(), expected_escrow_seed.as_ref());
}

pub fn assert_extensions_header(
    ctx: &TestContext,
    extensions_pda: &Pubkey,
    expected_bump: u8,
    expected_extension_count: u8,
) {
    let account = ctx.get_account(extensions_pda).expect("Extensions account should exist");
    assert_eq!(account.owner, ESCROW_PROGRAM_ID);

    let data = &account.data;
    assert!(data.len() >= 4, "Extensions account too small");
    assert_eq!(data[0], ESCROW_EXTENSIONS_DISCRIMINATOR, "Wrong discriminator");
    // data[1] is version (auto-prepended)
    assert_eq!(data[2], expected_bump, "Wrong bump");
    assert_eq!(data[3], expected_extension_count, "Wrong extension count");
}

pub fn assert_timelock_extension(ctx: &TestContext, extensions_pda: &Pubkey, expected_lock_duration: u64) {
    let account = ctx.get_account(extensions_pda).expect("Extensions account should exist");
    let data = &account.data;

    let tlv_data = find_extension(data, EXTENSION_TYPE_TIMELOCK).expect("Timelock extension not found");
    assert_eq!(tlv_data.len(), TIMELOCK_DATA_LEN, "Wrong timelock data length");

    let lock_duration = u64::from_le_bytes(tlv_data[0..8].try_into().unwrap());
    assert_eq!(lock_duration, expected_lock_duration, "Wrong lock duration");
}

pub fn assert_hook_extension(ctx: &TestContext, extensions_pda: &Pubkey, expected_hook_program: &Pubkey) {
    let account = ctx.get_account(extensions_pda).expect("Extensions account should exist");
    let data = &account.data;

    let tlv_data = find_extension(data, EXTENSION_TYPE_HOOK).expect("Hook extension not found");
    assert_eq!(tlv_data.len(), HOOK_DATA_LEN, "Wrong hook data length");

    let hook_program = Pubkey::new_from_array(tlv_data[0..32].try_into().unwrap());
    assert_eq!(hook_program, *expected_hook_program, "Wrong hook program");
}

pub fn assert_block_token_extensions_extension(
    ctx: &TestContext,
    extensions_pda: &Pubkey,
    expected_blocked_extensions: &[u16],
) {
    let account = ctx.get_account(extensions_pda).expect("Extensions account should exist");
    let data = &account.data;

    let tlv_data =
        find_extension(data, EXTENSION_TYPE_BLOCK_TOKEN_EXTENSIONS).expect("BlockTokenExtensions extension not found");

    let count = tlv_data[0];
    assert_eq!(count as usize, expected_blocked_extensions.len(), "Wrong count");

    let expected_len = block_token_extensions_byte_len(expected_blocked_extensions.len());
    assert_eq!(tlv_data.len(), expected_len, "Wrong block token extensions data length");

    for (i, &expected_ext) in expected_blocked_extensions.iter().enumerate() {
        let offset = 1 + (i * 2);
        let ext = u16::from_le_bytes([tlv_data[offset], tlv_data[offset + 1]]);
        assert_eq!(ext, expected_ext, "Wrong blocked extension at index {i}");
    }
}

pub fn assert_allowed_mint_account(ctx: &TestContext, allowed_mint_pda: &Pubkey, expected_bump: u8) {
    let account = ctx.get_account(allowed_mint_pda).expect("AllowedMint account should exist");

    assert_eq!(account.owner, ESCROW_PROGRAM_ID);

    let allowed_mint = AllowedMint::from_bytes(&account.data).expect("Should deserialize AllowedMint account");

    assert_eq!(allowed_mint.bump, expected_bump);
}
