# Test Coverage

This document provides a comprehensive overview of all integration tests for the Solana Escrow Program.

## Summary

| Instruction            | File                            | Test Count |
| ---------------------- | ------------------------------- | ---------- |
| CreateEscrow           | `test_create_escrow.rs`         | 11         |
| AddTimelock            | `test_add_timelock.rs`          | 14         |
| AllowMint              | `test_allow_mint.rs`            | 23         |
| BlockMint              | `test_block_mint.rs`            | 14         |
| AddBlockTokenExtension | `test_block_token_extension.rs` | 15         |
| SetHook                | `test_set_hook.rs`              | 16         |
| UpdateAdmin            | `test_update_admin.rs`          | 12         |
| Deposit                | `test_deposit.rs`               | 22         |
| Withdraw               | `test_withdraw.rs`              | 30         |
| **Total**              |                                 | **157**    |

## Error Codes Validated

### System Errors (InstructionError)

- `MissingRequiredSignature` - Signer validation
- `Immutable` / `ReadonlyDataModified` - Writable account validation
- `IncorrectProgramId` - Program validation (system, token, escrow)
- `InvalidSeeds` - PDA bump validation
- `InvalidAccountOwner` - Account ownership validation
- `InvalidInstructionData` - Instruction data validation
- `AccountAlreadyInitialized` - Re-initialization protection

### Custom Errors (EscrowError)

- `InvalidAdmin` (1) - Admin authorization failed
- `InvalidEventAuthority` (2) - Event authority PDA mismatch
- `TokenExtensionAlreadyBlocked` (12) - Duplicate blocked extension
- `PermanentDelegateNotAllowed` - Token-2022 extension blocked
- `NonTransferableNotAllowed` - Token-2022 extension blocked
- `PausableNotAllowed` - Token-2022 extension blocked
- `MintNotAllowed` - Mint has escrow-blocked extension
- `TimelockNotExpired` - Withdrawal before lock expiry
- `InvalidWithdrawer` - Withdrawer doesn't match receipt
- `InvalidReceiptEscrow` - Receipt belongs to different escrow

---

## CreateEscrow

**File:** `test_create_escrow.rs`

### Error Tests

| Test                                            | Description                 | Expected Error             |
| ----------------------------------------------- | --------------------------- | -------------------------- |
| `test_create_escrow_missing_admin_signer`       | Admin not signed            | `MissingRequiredSignature` |
| `test_create_escrow_missing_escrow_seed_signer` | Escrow seed not signed      | `MissingRequiredSignature` |
| `test_create_escrow_escrow_not_writable`        | Escrow account not writable | `ReadonlyDataModified`     |
| `test_create_escrow_wrong_system_program`       | Invalid system program      | `IncorrectProgramId`       |
| `test_create_escrow_wrong_current_program`      | Invalid escrow program      | `IncorrectProgramId`       |
| `test_create_escrow_invalid_event_authority`    | Wrong event authority PDA   | `Custom(2)`                |
| `test_create_escrow_invalid_bump`               | Incorrect PDA bump          | `InvalidSeeds`             |
| `test_create_escrow_empty_data`                 | Empty instruction data      | `InvalidInstructionData`   |
| `test_create_escrow_invalid_escrow_address`     | Wrong escrow PDA address    | `InvalidSeeds`             |

### Happy Path Tests

| Test                         | Description                                    |
| ---------------------------- | ---------------------------------------------- |
| `test_create_escrow_success` | Successfully creates escrow with correct state |

### Security Tests

| Test                                        | Description                          | Expected Error              |
| ------------------------------------------- | ------------------------------------ | --------------------------- |
| `test_create_escrow_reinitialization_fails` | Prevents overwriting existing escrow | `AccountAlreadyInitialized` |

---

## AddTimelock

**File:** `test_add_timelock.rs`

### Error Tests

| Test                                        | Description                     | Expected Error             |
| ------------------------------------------- | ------------------------------- | -------------------------- |
| `test_add_timelock_missing_admin_signer`    | Admin not signed                | `MissingRequiredSignature` |
| `test_add_timelock_extensions_not_writable` | Extensions account not writable | `ReadonlyDataModified`     |
| `test_add_timelock_wrong_system_program`    | Invalid system program          | `IncorrectProgramId`       |
| `test_add_timelock_wrong_escrow_program`    | Invalid escrow program          | `IncorrectProgramId`       |
| `test_add_timelock_invalid_event_authority` | Wrong event authority PDA       | `Custom(2)`                |
| `test_add_timelock_invalid_extensions_bump` | Incorrect extensions PDA bump   | `InvalidSeeds`             |
| `test_add_timelock_empty_data`              | Empty instruction data          | `InvalidInstructionData`   |
| `test_add_timelock_truncated_data`          | Truncated instruction data      | `InvalidInstructionData`   |

### Custom Error Tests

| Test                                                   | Description                     | Expected Error              |
| ------------------------------------------------------ | ------------------------------- | --------------------------- |
| `test_add_timelock_wrong_admin`                        | Non-admin tries to add timelock | `Custom(1)`                 |
| `test_add_timelock_escrow_not_owned_by_program`        | Escrow owned by wrong program   | `InvalidAccountOwner`       |
| `test_add_timelock_duplicate_extension`                | Adding timelock twice           | `AccountAlreadyInitialized` |
| `test_add_timelock_extensions_wrong_owner_before_init` | Wrong extensions address        | `InvalidSeeds`              |

### Happy Path Tests

| Test                                             | Description                                              |
| ------------------------------------------------ | -------------------------------------------------------- |
| `test_add_timelock_success`                      | Successfully adds timelock extension                     |
| `test_add_timelock_success_lock_duration_values` | Tests various durations: 0, 1, 60, 3600, 86400, u64::MAX |

---

## AllowMint

**File:** `test_allow_mint.rs`

### Error Tests

| Test                                             | Description                      | Expected Error             |
| ------------------------------------------------ | -------------------------------- | -------------------------- |
| `test_allow_mint_missing_admin_signer`           | Admin not signed                 | `MissingRequiredSignature` |
| `test_allow_mint_allowed_mint_not_writable`      | AllowedMint account not writable | `ReadonlyDataModified`     |
| `test_allow_mint_vault_not_writable`             | Vault account not writable       | `Immutable`                |
| `test_allow_mint_wrong_system_program`           | Invalid system program           | `IncorrectProgramId`       |
| `test_allow_mint_wrong_current_program`          | Invalid escrow program           | `IncorrectProgramId`       |
| `test_allow_mint_invalid_event_authority`        | Wrong event authority PDA        | `InvalidEventAuthority`    |
| `test_allow_mint_invalid_bump`                   | Incorrect PDA bump               | `InvalidSeeds`             |
| `test_allow_mint_wrong_admin`                    | Non-admin tries to allow mint    | `InvalidAdmin`             |
| `test_allow_mint_wrong_escrow`                   | Invalid escrow account           | `InvalidAccountOwner`      |
| `test_allow_mint_wrong_mint`                     | Invalid mint account             | `InvalidAccountOwner`      |
| `test_allow_mint_wrong_token_program`            | Invalid token program            | `IncorrectProgramId`       |
| `test_allow_mint_wrong_associated_token_program` | Invalid associated token program | `IncorrectProgramId`       |
| `test_allow_mint_duplicate`                      | Allowing same mint twice         | `AlreadyProcessed`         |

### Happy Path Tests

| Test                                           | Description                      |
| ---------------------------------------------- | -------------------------------- |
| `test_allow_mint_success`                      | Successfully allows mint         |
| `test_allow_mint_multiple_mints`               | Allows multiple different mints  |
| `test_allow_mint_token_2022_success`           | Works with Token-2022 mints      |
| `test_allow_mint_creates_vault_ata`            | Creates vault ATA for SPL Token  |
| `test_allow_mint_creates_vault_ata_token_2022` | Creates vault ATA for Token-2022 |

### Token-2022 Extension Tests

| Test                                                            | Description                               | Expected Error                |
| --------------------------------------------------------------- | ----------------------------------------- | ----------------------------- |
| `test_allow_mint_rejects_permanent_delegate`                    | Rejects PermanentDelegate extension       | `PermanentDelegateNotAllowed` |
| `test_allow_mint_rejects_non_transferable`                      | Rejects NonTransferable extension         | `NonTransferableNotAllowed`   |
| `test_allow_mint_rejects_pausable`                              | Rejects Pausable extension                | `PausableNotAllowed`          |
| `test_allow_mint_rejects_escrow_blocked_extension`              | Rejects escrow-specific blocked extension | `MintNotAllowed`              |
| `test_allow_mint_accepts_mint_without_escrow_blocked_extension` | Accepts mint with different extension     | Success                       |

---

## BlockMint

**File:** `test_block_mint.rs`

### Error Tests

| Test                                           | Description                       | Expected Error             |
| ---------------------------------------------- | --------------------------------- | -------------------------- |
| `test_block_mint_missing_admin_signer`         | Admin not signed                  | `MissingRequiredSignature` |
| `test_block_mint_allowed_mint_not_writable`    | AllowedMint account not writable  | `ReadonlyDataModified`     |
| `test_block_mint_wrong_current_program`        | Invalid escrow program            | `IncorrectProgramId`       |
| `test_block_mint_invalid_event_authority`      | Wrong event authority PDA         | `InvalidEventAuthority`    |
| `test_block_mint_wrong_admin`                  | Non-admin tries to block mint     | `InvalidAdmin`             |
| `test_block_mint_wrong_escrow`                 | Invalid escrow account            | `InvalidAccountOwner`      |
| `test_block_mint_wrong_mint`                   | Invalid mint account              | `InvalidAccountOwner`      |
| `test_block_mint_wrong_allowed_mint`           | Invalid allowed_mint account      | `InvalidAccountOwner`      |
| `test_block_mint_wrong_token_program`          | Invalid token program             | `IncorrectProgramId`       |
| `test_block_mint_allowed_mint_escrow_mismatch` | AllowedMint from different escrow | `InvalidSeeds`             |

### Happy Path Tests

| Test                                     | Description                                 |
| ---------------------------------------- | ------------------------------------------- |
| `test_block_mint_success`                | Successfully blocks mint and closes account |
| `test_block_mint_rent_returned_to_payer` | Returns rent to payer                       |
| `test_block_multiple_mints_same_escrow`  | Blocks multiple mints from same escrow      |
| `test_block_mint_token_2022_success`     | Works with Token-2022 mints                 |

---

## AddBlockTokenExtension

**File:** `test_block_token_extension.rs`

### Error Tests

| Test                                                     | Description                        | Expected Error             |
| -------------------------------------------------------- | ---------------------------------- | -------------------------- |
| `test_block_token_extension_missing_admin_signer`        | Admin not signed                   | `MissingRequiredSignature` |
| `test_block_token_extension_extensions_not_writable`     | Extensions account not writable    | `ReadonlyDataModified`     |
| `test_block_token_extension_wrong_system_program`        | Invalid system program             | `IncorrectProgramId`       |
| `test_block_token_extension_wrong_escrow_program`        | Invalid escrow program             | `IncorrectProgramId`       |
| `test_block_token_extension_invalid_event_authority`     | Wrong event authority PDA          | `Custom(2)`                |
| `test_block_token_extension_invalid_extensions_bump`     | Incorrect extensions PDA bump      | `InvalidSeeds`             |
| `test_block_token_extension_empty_data`                  | Empty instruction data             | `InvalidInstructionData`   |
| `test_block_token_extension_truncated_data`              | Truncated instruction data         | `InvalidInstructionData`   |
| `test_block_token_extension_wrong_admin`                 | Non-admin tries to block extension | `Custom(1)`                |
| `test_block_token_extension_escrow_not_owned_by_program` | Escrow owned by wrong program      | `InvalidAccountOwner`      |
| `test_block_token_extension_duplicate_extension`         | Blocking same extension twice      | `Custom(12)`               |

### Happy Path Tests

| Test                                                     | Description                               |
| -------------------------------------------------------- | ----------------------------------------- |
| `test_block_token_extension_success`                     | Successfully blocks token extension       |
| `test_block_token_extension_success_single`              | Blocks single extension by type ID        |
| `test_block_token_extension_success_many_extensions`     | Blocks 20 extensions sequentially         |
| `test_block_token_extension_success_multiple_extensions` | Combines timelock with blocked extensions |

---

## SetHook

**File:** `test_set_hook.rs`

### Error Tests

| Test                                        | Description                     | Expected Error              |
| ------------------------------------------- | ------------------------------- | --------------------------- |
| `test_set_hook_missing_admin_signer`        | Admin not signed                | `MissingRequiredSignature`  |
| `test_set_hook_extensions_not_writable`     | Extensions account not writable | `ReadonlyDataModified`      |
| `test_set_hook_wrong_system_program`        | Invalid system program          | `IncorrectProgramId`        |
| `test_set_hook_wrong_escrow_program`        | Invalid escrow program          | `IncorrectProgramId`        |
| `test_set_hook_invalid_event_authority`     | Wrong event authority PDA       | `Custom(2)`                 |
| `test_set_hook_invalid_extensions_bump`     | Incorrect extensions PDA bump   | `InvalidSeeds`              |
| `test_set_hook_empty_data`                  | Empty instruction data          | `InvalidInstructionData`    |
| `test_set_hook_truncated_data`              | Truncated instruction data      | `InvalidInstructionData`    |
| `test_set_hook_wrong_admin`                 | Non-admin tries to set hook     | `Custom(1)`                 |
| `test_set_hook_escrow_not_owned_by_program` | Escrow owned by wrong program   | `InvalidAccountOwner`       |
| `test_set_hook_duplicate_extension`         | Setting hook twice              | `AccountAlreadyInitialized` |

### Happy Path Tests

| Test                                      | Description                             |
| ----------------------------------------- | --------------------------------------- | ------- |
| `test_set_hook_success`                   | Successfully sets hook program          |
| `test_set_hook_hook_program_zero_address` | Setting hook to zero address (disabled) | Success |

### Combined Extension Tests

| Test                                       | Description                                   |
| ------------------------------------------ | --------------------------------------------- |
| `test_add_timelock_then_set_hook`          | Adds timelock first, then hook                |
| `test_set_hook_then_add_timelock`          | Adds hook first, then timelock                |
| `test_multiple_extensions_extension_count` | Verifies extension count increments correctly |

---

## UpdateAdmin

**File:** `test_update_admin.rs`

### Error Tests

| Test                                         | Description                 | Expected Error             |
| -------------------------------------------- | --------------------------- | -------------------------- |
| `test_update_admin_missing_admin_signer`     | Admin not signed            | `MissingRequiredSignature` |
| `test_update_admin_missing_new_admin_signer` | New admin not signed        | `MissingRequiredSignature` |
| `test_update_admin_escrow_not_writable`      | Escrow account not writable | `ReadonlyDataModified`     |
| `test_update_admin_wrong_escrow_program`     | Invalid escrow program      | `IncorrectProgramId`       |
| `test_update_admin_invalid_event_authority`  | Wrong event authority PDA   | `Custom(2)`                |
| `test_update_admin_wrong_escrow_pda_bump`    | Wrong escrow PDA            | `InvalidAccountOwner`      |

### Custom Error Tests

| Test                                            | Description                   | Expected Error        |
| ----------------------------------------------- | ----------------------------- | --------------------- |
| `test_update_admin_wrong_admin`                 | Non-admin tries to update     | `Custom(1)`           |
| `test_update_admin_escrow_not_owned_by_program` | Escrow owned by wrong program | `InvalidAccountOwner` |

### Happy Path Tests

| Test                                        | Description                               |
| ------------------------------------------- | ----------------------------------------- | ------- |
| `test_update_admin_success`                 | Successfully updates admin                |
| `test_update_admin_can_update_again`        | New admin can update to another admin     |
| `test_update_admin_old_admin_cannot_update` | Old admin loses access after transfer     |
| `test_update_admin_idempotent`              | Update admin to same address (idempotent) | Success |

---

## Deposit

**File:** `test_deposit.rs`

### Error Tests

| Test                                                | Description                              | Expected Error             |
| --------------------------------------------------- | ---------------------------------------- | -------------------------- |
| `test_deposit_missing_depositor_signer`             | Depositor not signed                     | `MissingRequiredSignature` |
| `test_deposit_missing_receipt_seed_signer`          | Receipt seed not signed                  | `MissingRequiredSignature` |
| `test_receipt_not_writable`                         | Receipt account not writable             | `ReadonlyDataModified`     |
| `test_deposit_vault_not_writable`                   | Vault account not writable               | `ReadonlyDataModified`     |
| `test_deposit_depositor_token_account_not_writable` | Depositor token account not writable     | `ReadonlyDataModified`     |
| `test_deposit_wrong_system_program`                 | Invalid system program                   | `IncorrectProgramId`       |
| `test_deposit_wrong_current_program`                | Invalid escrow program                   | `IncorrectProgramId`       |
| `test_deposit_empty_data`                           | Empty instruction data                   | `InvalidInstructionData`   |
| `test_deposit_invalid_bump`                         | Incorrect PDA bump                       | `InvalidSeeds`             |
| `test_deposit_wrong_token_program`                  | Invalid token program                    | `IncorrectProgramId`       |
| `test_deposit_wrong_escrow_owner`                   | Escrow owned by wrong program            | `InvalidAccountOwner`      |
| `test_deposit_wrong_allowed_mint_owner`             | AllowedMint owned by wrong program       | `InvalidAccountOwner`      |
| `test_deposit_zero_amount`                          | Zero deposit amount                      | `ZeroDepositAmount`        |
| `test_deposit_invalid_event_authority`              | Wrong event authority PDA                | `Custom(2)`                |
| `test_deposit_wrong_vault_ata`                      | Wrong vault ATA address                  | `InvalidAccountData`       |
| `test_deposit_wrong_depositor_token_account_owner`  | Wrong depositor token account derivation | `InvalidAccountData`       |
| `test_deposit_mint_not_allowed`                     | Invalid AllowedMint PDA                  | `InvalidAccountOwner`      |

### Happy Path Tests

| Test                              | Description                                      |
| --------------------------------- | ------------------------------------------------ |
| `test_deposit_success`            | Successfully deposits tokens and creates receipt |
| `test_deposit_multiple_deposits`  | Multiple deposits with different receipt seeds   |
| `test_deposit_token_2022_success` | Works with Token-2022 tokens                     |

### Hook Program Tests

| Test                              | Description                  | Expected Error         |
| --------------------------------- | ---------------------------- | ---------------------- |
| `test_deposit_with_hook_success`  | Hook program allows deposit  | Success                |
| `test_deposit_with_hook_rejected` | Hook program rejects deposit | `TEST_HOOK_DENY_ERROR` |

---

## Withdraw

**File:** `test_withdraw.rs`

### Error Tests

| Test                                                  | Description                               | Expected Error             |
| ----------------------------------------------------- | ----------------------------------------- | -------------------------- |
| `test_withdraw_missing_withdrawer_signer`             | Withdrawer not signed                     | `MissingRequiredSignature` |
| `test_withdraw_receipt_not_writable`                  | Receipt account not writable              | `ReadonlyDataModified`     |
| `test_withdraw_vault_not_writable`                    | Vault account not writable                | `ReadonlyDataModified`     |
| `test_withdraw_withdrawer_token_account_not_writable` | Withdrawer token account not writable     | `ReadonlyDataModified`     |
| `test_withdraw_wrong_system_program`                  | Invalid system program                    | `IncorrectProgramId`       |
| `test_withdraw_wrong_current_program`                 | Invalid escrow program                    | `IncorrectProgramId`       |
| `test_withdraw_invalid_event_authority`               | Wrong event authority PDA                 | `Custom(2)`                |
| `test_withdraw_wrong_token_program`                   | Invalid token program                     | `IncorrectProgramId`       |
| `test_withdraw_wrong_escrow_owner`                    | Escrow owned by wrong program             | `InvalidAccountOwner`      |
| `test_withdraw_wrong_receipt_owner`                   | Receipt owned by wrong program            | `InvalidAccountOwner`      |
| `test_withdraw_wrong_extensions_account`              | Wrong extensions PDA address              | `InvalidSeeds`             |
| `test_withdraw_wrong_vault_ata`                       | Wrong vault ATA address                   | `InvalidAccountData`       |
| `test_withdraw_wrong_withdrawer_ata`                  | Wrong withdrawer token account derivation | `InvalidAccountData`       |

### Custom Error Tests

| Test                             | Description                     | Expected Error      |
| -------------------------------- | ------------------------------- | ------------------- |
| `test_withdraw_wrong_withdrawer` | Non-depositor tries to withdraw | `InvalidWithdrawer` |

### Timelock Tests

| Test                                     | Description                        | Expected Error       |
| ---------------------------------------- | ---------------------------------- | -------------------- |
| `test_withdraw_timelock_not_expired`     | Withdraw before lock expires       | `TimelockNotExpired` |
| `test_withdraw_timelock_expired_success` | Withdraw after lock expires        | Success              |
| `test_withdraw_no_timelock_success`      | Withdraw with no timelock          | Success              |
| `test_withdraw_timelock_zero_duration`   | Zero duration timelock (immediate) | Success              |

### Happy Path Tests

| Test                                             | Description                                 |
| ------------------------------------------------ | ------------------------------------------- |
| `test_withdraw_success`                          | Successfully withdraws tokens               |
| `test_withdraw_closes_receipt`                   | Receipt account closed after withdraw       |
| `test_withdraw_transfers_tokens`                 | Tokens transferred from vault to withdrawer |
| `test_withdraw_returns_rent_to_payer`            | Rent returned to payer                      |
| `test_withdraw_returns_rent_to_custom_recipient` | Rent returned to custom recipient           |
| `test_withdraw_token_2022_success`               | Works with Token-2022 tokens                |

### Hook Program Tests

| Test                                    | Description                              | Expected Error         |
| --------------------------------------- | ---------------------------------------- | ---------------------- |
| `test_withdraw_with_hook_success`       | Hook program allows withdrawal           | Success                |
| `test_withdraw_with_hook_rejected`      | Hook program rejects withdrawal          | `TEST_HOOK_DENY_ERROR` |
| `test_withdraw_with_hook_wrong_program` | Wrong hook program in remaining accounts | `HookProgramMismatch`  |

### Security Tests

| Test                                                    | Description                              | Expected Error         |
| ------------------------------------------------------- | ---------------------------------------- | ---------------------- |
| `test_withdraw_receipt_for_different_escrow_fails`      | Receipt from escrow A used with escrow B | `InvalidReceiptEscrow` |
| `test_withdraw_double_withdraw_fails`                   | Second withdraw on closed receipt        | `InvalidAccountOwner`  |
| `test_withdraw_rejects_reactivated_account_wrong_owner` | Spoofed receipt with wrong owner         | `InvalidAccountOwner`  |

---

## Test Categories

### 1. Signer Validation

Tests ensure all required signers are validated:

- Admin signer for configuration operations
- Depositor/Withdrawer signer for token operations
- Seed signers for PDA derivation

### 2. Writable Account Validation

Tests ensure writable accounts are properly marked:

- State accounts (escrow, extensions, receipt, allowed_mint)
- Token accounts (vault, user token accounts)

### 3. Program Validation

Tests verify correct programs are passed:

- System program for account creation
- Token program (SPL Token or Token-2022)
- Escrow program for CPI operations

### 4. PDA Validation

Tests verify PDA derivation:

- Correct bumps match seeds
- Seeds match account data

### 5. Admin Authorization

Tests ensure admin-only operations are protected:

- Configuration changes require admin signature
- Admin transfer properly revokes old admin access

### 6. Token Operations

Tests cover both token standards:

- SPL Token (original)
- Token-2022 (with extensions)

### 7. Extension System

Tests verify the extension framework:

- Individual extension creation
- Multiple extension combinations
- Extension ordering independence

### 8. Timelock Enforcement

Tests verify time-based restrictions:

- Lock duration enforcement
- Timestamp-based unlock
- Zero duration (immediate) handling

### 9. Hook Integration

Tests verify CPI hook calls:

- Allowing hooks approve operations
- Denying hooks reject operations
- Hook errors propagate correctly

### 10. Security Protections

Tests verify attack prevention:

- Re-initialization attacks blocked
- Double-withdraw attacks blocked
- Cross-escrow receipt attacks blocked
- Account reactivation attacks blocked
