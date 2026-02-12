# Test Coverage

## Estimated Coverage

> **Methodology**: This is a semantic coverage estimate produced by analyzing test
> assertions against the program's testable surface. It is not instrumented line
> coverage -- Solana SBF programs do not support LLVM coverage instrumentation.
> Coverage is risk-weighted: instruction handlers, account validation, and business
> logic errors carry more weight than events and utilities.

| Category                      | Covered | Total | Est. Coverage |
| ----------------------------- | ------- | ----- | ------------- |
| Instruction handlers          | 11      | 11    | 100%          |
| Account validation paths      | 83      | 106   | 78%           |
| Business logic error branches | 40      | 70    | 57%           |
| Custom error codes exercised  | 14      | 15    | 93%           |
| State & trait coverage (unit) | 24      | 24    | 100%          |
| Event & utility coverage      | 12      | 18    | 67%           |
| Security edge cases           | 3       | 3     | 100%          |
| **Overall (risk-weighted)**   |         |       | **~80%**      |

### Coverage Gaps

**Untested Error Variants:**

- `InvalidEscrowId` (0) -- no integration test triggers this error

**Untested Validation Paths:**

- Deposit: `verify_signer(payer, true)` -- payer signer implicitly tested via transaction fee payer but no explicit negative test
- Deposit: `verify_readonly(escrow)` -- no test makes escrow writable
- Deposit: `verify_readonly(allowed_mint)` -- no test makes allowed_mint writable
- Deposit: `verify_readonly(mint)` -- no test makes mint writable
- Deposit: `verify_readonly(extensions)` -- no test makes extensions writable
- Withdraw: `verify_writable(rent_recipient, true)` -- no test makes rent_recipient read-only
- Withdraw: `verify_readonly(escrow)` -- no test makes escrow writable
- Withdraw: `verify_readonly(extensions)` -- no test makes extensions writable
- Withdraw: `verify_readonly(mint)` -- no test makes mint writable
- AllowMint: `verify_signer(payer, true)` -- payer signer implicitly tested
- AllowMint: `verify_readonly(escrow)` -- no explicit test
- AllowMint: `verify_readonly(escrow_extensions)` -- no explicit test
- AllowMint: `verify_readonly(mint)` -- no explicit test
- AllowMint: `validate_associated_token_account_address(vault)` -- no explicit ATA address mismatch test
- BlockMint: `verify_writable(rent_recipient, true)` -- no explicit test
- BlockMint: `verify_readonly(escrow)` -- no explicit test
- BlockMint: `verify_readonly(mint)` -- no explicit test
- AddTimelock: `verify_signer(payer, true)` -- payer signer implicitly tested
- AddTimelock: `verify_readonly(escrow)` -- no explicit test
- SetHook: `verify_signer(payer, true)` -- payer signer implicitly tested
- SetHook: `verify_readonly(escrow)` -- no explicit test
- BlockTokenExtension: `verify_signer(payer, true)` -- payer signer implicitly tested
- BlockTokenExtension: `verify_readonly(escrow)` -- no explicit test

**Untested Processor Branches:**

- CreateEscrow: `escrow_seeds.try_into().map_err(InvalidArgument)` -- no test provides seeds that fail conversion
- CreateEscrow: `escrow.write_to_slice()` failure (AccountDataTooSmall) -- only tested in unit tests, not integration
- CreateEscrow: `emit_event()` failure -- only tested indirectly via event_authority validation
- Deposit: `Escrow::from_account()` discriminator/PDA failure after ownership check -- not directly targeted
- Deposit: `AllowedMint::from_account()` discriminator failure (mapped to MintNotAllowed) -- partially covered by `allowed_mint_not_owned_by_program`
- Deposit: `Clock::get()` failure -- not feasible to test in LiteSVM
- Deposit: `HookData::from_bytes()` parse error -- no test provides malformed hook extension data
- Deposit: hook `PostDeposit` invocation failure -- no test triggers post-deposit hook rejection independently
- Deposit: `get_mint_decimals()` failure -- no test provides invalid mint data
- Deposit: `TransferChecked.invoke()` CPI failure -- no test causes token transfer to fail (e.g., insufficient balance)
- Withdraw: `Escrow::from_bytes()` second borrow for transfer_checked -- not separately tested
- Withdraw: `close_pda_account()` arithmetic overflow -- not tested
- Withdraw: hook `PostWithdraw` invocation failure -- no test triggers post-withdraw hook rejection independently
- BlockMint: `close_pda_account()` arithmetic overflow -- not tested
- BlockTokenExtension: `BlockTokenExtensionsData::new()` overflow (InvalidArgument) -- not tested in integration
- BlockTokenExtension: `update_or_append_extension()` CPI/resize failure -- not tested
- EmitEvent: account count guard (NotEnoughAccountKeys) -- not directly tested
- EmitEvent: `verify_signer(event_authority)` -- not directly tested (always invoked via CPI)

**Untested Utility Functions (no direct unit tests):**

- `account_utils.rs`: `verify_writable`, `verify_readonly`, `verify_signer`, `verify_owned_by`, `verify_system_account`, `verify_token_program_account`, `verify_current_program_account`
- `program_utils.rs`: `verify_system_program`, `verify_token_program`, `verify_current_program`, `verify_associated_token_program`
- `event_utils.rs`: `verify_event_authority`, `emit_event`
- `pda_utils.rs`: `close_pda_account`, `create_pda_account`, `create_pda_account_idempotent`
- `token_utils.rs`: `validate_associated_token_account_address`, `validate_associated_token_account`, `get_mint_decimals`
- `token2022_utils.rs`: `validate_mint_extensions`

Note: All utility functions above are exercised indirectly through integration tests. Only direct unit test coverage is missing.

---

## Summary

| Instruction         | File                            | Test Count |
| ------------------- | ------------------------------- | ---------- |
| CreateEscrow        | `test_create_escrow.rs`         | 10         |
| AddTimelock         | `test_add_timelock.rs`          | 13         |
| SetHook             | `test_set_hook.rs`              | 16         |
| Deposit             | `test_deposit.rs`               | 22         |
| UpdateAdmin         | `test_update_admin.rs`          | 11         |
| Withdraw            | `test_withdraw.rs`              | 38         |
| AllowMint           | `test_allow_mint.rs`            | 23         |
| BlockMint           | `test_block_mint.rs`            | 14         |
| BlockTokenExtension | `test_block_token_extension.rs` | 15         |
| SetArbiter          | `test_set_arbiter.rs`           | 16         |
| **Total**           |                                 | **178**    |

## Error Codes Validated

### System Errors (InstructionError)

- `MissingRequiredSignature` -- signer validation across all instructions
- `Immutable` -- writable account validation across all instructions
- `IncorrectProgramId` -- system program, current program, token program, associated token program validation
- `InvalidSeeds` -- PDA bump validation across all instructions with PDA accounts
- `InvalidInstructionData` -- empty/truncated instruction data across all instructions
- `InvalidAccountOwner` -- account ownership checks (escrow, receipt, allowed_mint, mint)
- `InvalidAccountData` -- ATA address mismatch, invalid mint data
- `AccountAlreadyInitialized` -- re-initialization (CreateEscrow), duplicate extensions (AddTimelock, SetHook, SetArbiter)

### Custom Errors (EscrowProgramError)

- `InvalidAdmin` (1) -- admin authorization failed (UpdateAdmin, AllowMint, BlockMint, AddTimelock, SetHook, BlockTokenExtension, SetArbiter)
- `InvalidEventAuthority` (2) -- event authority PDA mismatch (all instructions)
- `TimelockNotExpired` (3) -- withdraw before lock expires (Withdraw)
- `HookRejected` (4) -- external hook denied operation (Deposit, Withdraw)
- `InvalidWithdrawer` (5) -- withdrawer does not match receipt depositor (Withdraw)
- `InvalidReceiptEscrow` (6) -- receipt escrow does not match target escrow (Withdraw)
- `HookProgramMismatch` (7) -- hook program does not match stored hook (Withdraw)
- `MintNotAllowed` (8) -- mint blocked by escrow token extension rules (AllowMint)
- `PermanentDelegateNotAllowed` (9) -- mint has PermanentDelegate extension (AllowMint)
- `NonTransferableNotAllowed` (10) -- mint has NonTransferable extension (AllowMint)
- `PausableNotAllowed` (11) -- mint has Pausable extension (AllowMint)
- `TokenExtensionAlreadyBlocked` (12) -- duplicate blocked extension (BlockTokenExtension)
- `ZeroDepositAmount` (13) -- zero deposit amount (Deposit)
- `InvalidArbiter` (14) -- arbiter signer missing or wrong address (Withdraw)

### Untested Custom Errors

- `InvalidEscrowId` (0) -- not triggered by any test

---

## CreateEscrow

**File:** `test_create_escrow.rs`

### Error Tests

| Test                                            | Description                 | Expected Error                        |
| ----------------------------------------------- | --------------------------- | ------------------------------------- |
| `test_create_escrow_missing_admin_signer`       | Admin not signed            | `MissingRequiredSignature`            |
| `test_create_escrow_missing_escrow_seed_signer` | Escrow seed not signed      | `MissingRequiredSignature`            |
| `test_create_escrow_escrow_not_writable`        | Escrow account not writable | `Immutable`                           |
| `test_create_escrow_wrong_system_program`       | Invalid system program      | `IncorrectProgramId`                  |
| `test_create_escrow_wrong_current_program`      | Invalid escrow program      | `IncorrectProgramId`                  |
| `test_create_escrow_invalid_event_authority`    | Wrong event authority PDA   | `Custom(2)` / `InvalidEventAuthority` |
| `test_create_escrow_invalid_bump`               | Incorrect PDA bump          | `InvalidSeeds`                        |
| `test_create_escrow_empty_data`                 | Empty instruction data      | `InvalidInstructionData`              |

### Security Tests

| Test                                        | Description                          | Expected Error              |
| ------------------------------------------- | ------------------------------------ | --------------------------- |
| `test_create_escrow_reinitialization_fails` | Prevents overwriting existing escrow | `AccountAlreadyInitialized` |

### Happy Path Tests

| Test                         | Description                                    |
| ---------------------------- | ---------------------------------------------- |
| `test_create_escrow_success` | Successfully creates escrow with correct state |

---

## AddTimelock

**File:** `test_add_timelock.rs`

### Error Tests

| Test                                        | Description                     | Expected Error                        |
| ------------------------------------------- | ------------------------------- | ------------------------------------- |
| `test_add_timelock_missing_admin_signer`    | Admin not signed                | `MissingRequiredSignature`            |
| `test_add_timelock_extensions_not_writable` | Extensions account not writable | `Immutable`                           |
| `test_add_timelock_wrong_system_program`    | Invalid system program          | `IncorrectProgramId`                  |
| `test_add_timelock_wrong_escrow_program`    | Invalid escrow program          | `IncorrectProgramId`                  |
| `test_add_timelock_invalid_event_authority` | Wrong event authority PDA       | `Custom(2)` / `InvalidEventAuthority` |
| `test_add_timelock_invalid_extensions_bump` | Incorrect extensions PDA bump   | `InvalidSeeds`                        |
| `test_add_timelock_empty_data`              | Empty instruction data          | `InvalidInstructionData`              |
| `test_add_timelock_truncated_data`          | Truncated instruction data      | `InvalidInstructionData`              |
| `test_add_timelock_duplicate_extension`     | Timelock already exists         | `AccountAlreadyInitialized`           |

### Custom Error Tests

| Test                                            | Description                     | Expected Error               |
| ----------------------------------------------- | ------------------------------- | ---------------------------- |
| `test_add_timelock_wrong_admin`                 | Non-admin tries to add timelock | `Custom(1)` / `InvalidAdmin` |
| `test_add_timelock_escrow_not_owned_by_program` | Escrow not owned by program     | `InvalidAccountOwner`        |

### Happy Path Tests

| Test                                             | Description                                        |
| ------------------------------------------------ | -------------------------------------------------- |
| `test_add_timelock_success`                      | Successfully adds timelock extension               |
| `test_add_timelock_success_lock_duration_values` | Various durations: 0, 1, 60, 3600, 86400, u64::MAX |

---

## SetHook

**File:** `test_set_hook.rs`

### Error Tests

| Test                                    | Description                     | Expected Error                        |
| --------------------------------------- | ------------------------------- | ------------------------------------- |
| `test_set_hook_missing_admin_signer`    | Admin not signed                | `MissingRequiredSignature`            |
| `test_set_hook_extensions_not_writable` | Extensions account not writable | `Immutable`                           |
| `test_set_hook_wrong_system_program`    | Invalid system program          | `IncorrectProgramId`                  |
| `test_set_hook_wrong_escrow_program`    | Invalid escrow program          | `IncorrectProgramId`                  |
| `test_set_hook_invalid_event_authority` | Wrong event authority PDA       | `Custom(2)` / `InvalidEventAuthority` |
| `test_set_hook_invalid_extensions_bump` | Incorrect extensions PDA bump   | `InvalidSeeds`                        |
| `test_set_hook_empty_data`              | Empty instruction data          | `InvalidInstructionData`              |
| `test_set_hook_truncated_data`          | Truncated instruction data      | `InvalidInstructionData`              |
| `test_set_hook_duplicate_extension`     | Hook already exists             | `AccountAlreadyInitialized`           |

### Custom Error Tests

| Test                                        | Description                 | Expected Error               |
| ------------------------------------------- | --------------------------- | ---------------------------- |
| `test_set_hook_wrong_admin`                 | Non-admin tries to set hook | `Custom(1)` / `InvalidAdmin` |
| `test_set_hook_escrow_not_owned_by_program` | Escrow not owned by program | `InvalidAccountOwner`        |

### Happy Path Tests

| Test                                      | Description                      |
| ----------------------------------------- | -------------------------------- |
| `test_set_hook_success`                   | Successfully sets hook extension |
| `test_set_hook_hook_program_zero_address` | Sets hook to default (disabled)  |

### Combined Extension Tests

| Test                                       | Description                          |
| ------------------------------------------ | ------------------------------------ |
| `test_add_timelock_then_set_hook`          | Timelock first, then hook            |
| `test_set_hook_then_add_timelock`          | Hook first, then timelock            |
| `test_multiple_extensions_extension_count` | Extension count increments correctly |

---

## Deposit

**File:** `test_deposit.rs`

### Error Tests

| Test                                                | Description                          | Expected Error             |
| --------------------------------------------------- | ------------------------------------ | -------------------------- |
| `test_deposit_missing_depositor_signer`             | Depositor not signed                 | `MissingRequiredSignature` |
| `test_deposit_missing_receipt_seed_signer`          | Receipt seed not signed              | `MissingRequiredSignature` |
| `test_receipt_not_writable`                         | Receipt account not writable         | `Immutable`                |
| `test_deposit_vault_not_writable`                   | Vault not writable                   | `Immutable`                |
| `test_deposit_depositor_token_account_not_writable` | Depositor token account not writable | `Immutable`                |
| `test_deposit_wrong_system_program`                 | Invalid system program               | `IncorrectProgramId`       |
| `test_deposit_wrong_current_program`                | Invalid escrow program               | `IncorrectProgramId`       |
| `test_deposit_wrong_token_program`                  | Invalid token program                | `IncorrectProgramId`       |
| `test_deposit_invalid_bump`                         | Incorrect PDA bump                   | `InvalidSeeds`             |
| `test_deposit_empty_data`                           | Empty instruction data               | `InvalidInstructionData`   |
| `test_deposit_wrong_escrow_owner`                   | Escrow not owned by program          | `InvalidAccountOwner`      |
| `test_deposit_wrong_allowed_mint_owner`             | AllowedMint not owned by program     | `InvalidAccountOwner`      |
| `test_deposit_allowed_mint_not_owned_by_program`    | AllowedMint wrong owner              | `InvalidAccountOwner`      |
| `test_deposit_wrong_vault_ata`                      | Wrong vault ATA address              | `InvalidAccountData`       |
| `test_deposit_wrong_depositor_token_account_owner`  | Wrong depositor token account        | `InvalidAccountData`       |

### Custom Error Tests

| Test                                   | Description               | Expected Error                        |
| -------------------------------------- | ------------------------- | ------------------------------------- |
| `test_deposit_zero_amount`             | Zero deposit amount       | `Custom(13)` / `ZeroDepositAmount`    |
| `test_deposit_invalid_event_authority` | Wrong event authority PDA | `Custom(2)` / `InvalidEventAuthority` |

### Hook Program Tests

| Test                              | Description         | Expected Error                       |
| --------------------------------- | ------------------- | ------------------------------------ |
| `test_deposit_with_hook_success`  | Hook allows deposit | --                                   |
| `test_deposit_with_hook_rejected` | Hook denies deposit | `Custom(1)` / `TEST_HOOK_DENY_ERROR` |

### Token-2022 Extension Tests

| Test                              | Description                  |
| --------------------------------- | ---------------------------- |
| `test_deposit_token_2022_success` | Deposit with Token-2022 mint |

### Happy Path Tests

| Test                             | Description                                      |
| -------------------------------- | ------------------------------------------------ |
| `test_deposit_success`           | Successfully deposits tokens and creates receipt |
| `test_deposit_multiple_deposits` | Two deposits with different receipt seeds        |

---

## UpdateAdmin

**File:** `test_update_admin.rs`

### Error Tests

| Test                                            | Description                 | Expected Error             |
| ----------------------------------------------- | --------------------------- | -------------------------- |
| `test_update_admin_missing_admin_signer`        | Admin not signed            | `MissingRequiredSignature` |
| `test_update_admin_missing_new_admin_signer`    | New admin not signed        | `MissingRequiredSignature` |
| `test_update_admin_escrow_not_writable`         | Escrow account not writable | `Immutable`                |
| `test_update_admin_wrong_escrow_program`        | Invalid escrow program      | `IncorrectProgramId`       |
| `test_update_admin_escrow_not_owned_by_program` | Escrow not owned by program | `InvalidAccountOwner`      |

### Custom Error Tests

| Test                                        | Description                            | Expected Error                        |
| ------------------------------------------- | -------------------------------------- | ------------------------------------- |
| `test_update_admin_wrong_admin`             | Non-admin tries to update              | `Custom(1)` / `InvalidAdmin`          |
| `test_update_admin_invalid_event_authority` | Wrong event authority PDA              | `Custom(2)` / `InvalidEventAuthority` |
| `test_update_admin_old_admin_cannot_update` | Previous admin rejected after transfer | `Custom(1)` / `InvalidAdmin`          |

### Happy Path Tests

| Test                                 | Description                         |
| ------------------------------------ | ----------------------------------- |
| `test_update_admin_success`          | Successfully updates admin          |
| `test_update_admin_can_update_again` | Two sequential admin updates        |
| `test_update_admin_idempotent`       | Update admin to same admin succeeds |

---

## Withdraw

**File:** `test_withdraw.rs`

### Error Tests

| Test                                                  | Description                           | Expected Error             |
| ----------------------------------------------------- | ------------------------------------- | -------------------------- |
| `test_withdraw_missing_withdrawer_signer`             | Withdrawer not signed                 | `MissingRequiredSignature` |
| `test_withdraw_receipt_not_writable`                  | Receipt not writable                  | `Immutable`                |
| `test_withdraw_vault_not_writable`                    | Vault not writable                    | `Immutable`                |
| `test_withdraw_withdrawer_token_account_not_writable` | Withdrawer token account not writable | `Immutable`                |
| `test_withdraw_wrong_system_program`                  | Invalid system program                | `IncorrectProgramId`       |
| `test_withdraw_wrong_current_program`                 | Invalid escrow program                | `IncorrectProgramId`       |
| `test_withdraw_wrong_token_program`                   | Invalid token program                 | `IncorrectProgramId`       |
| `test_withdraw_wrong_escrow_owner`                    | Escrow not owned by program           | `InvalidAccountOwner`      |
| `test_withdraw_wrong_receipt_owner`                   | Receipt not owned by program          | `InvalidAccountOwner`      |
| `test_withdraw_wrong_extensions_account`              | Wrong extensions PDA                  | `InvalidSeeds`             |
| `test_withdraw_wrong_vault_ata`                       | Wrong vault ATA address               | `InvalidAccountData`       |
| `test_withdraw_wrong_withdrawer_ata`                  | Wrong withdrawer token account        | `InvalidAccountData`       |

### Custom Error Tests

| Test                                    | Description                        | Expected Error                        |
| --------------------------------------- | ---------------------------------- | ------------------------------------- |
| `test_withdraw_wrong_withdrawer`        | Withdrawer doesn't match depositor | `Custom(5)` / `InvalidWithdrawer`     |
| `test_withdraw_invalid_event_authority` | Wrong event authority PDA          | `Custom(2)` / `InvalidEventAuthority` |

### Timelock Tests

| Test                                     | Description                       | Expected Error                     |
| ---------------------------------------- | --------------------------------- | ---------------------------------- |
| `test_withdraw_timelock_not_expired`     | Withdraw before lock expires      | `Custom(3)` / `TimelockNotExpired` |
| `test_withdraw_timelock_expired_success` | Withdraw after lock expires       | --                                 |
| `test_withdraw_timelock_zero_duration`   | Timelock with duration=0 succeeds | --                                 |

### Hook Program Tests

| Test                                    | Description           | Expected Error                       |
| --------------------------------------- | --------------------- | ------------------------------------ |
| `test_withdraw_with_hook_success`       | Hook allows withdraw  | --                                   |
| `test_withdraw_with_hook_rejected`      | Hook denies withdraw  | `Custom(1)` / `TEST_HOOK_DENY_ERROR` |
| `test_withdraw_with_hook_wrong_program` | Hook program mismatch | `Custom(7)` / `HookProgramMismatch`  |

### Security Tests

| Test                                                    | Description                              | Expected Error                       |
| ------------------------------------------------------- | ---------------------------------------- | ------------------------------------ |
| `test_withdraw_receipt_for_different_escrow_fails`      | Receipt from escrow A used with escrow B | `Custom(6)` / `InvalidReceiptEscrow` |
| `test_withdraw_double_withdraw_fails`                   | Double withdraw attempt                  | `InvalidAccountOwner`                |
| `test_withdraw_rejects_reactivated_account_wrong_owner` | Reactivated receipt rejected             | `InvalidAccountOwner`                |

### Token-2022 Extension Tests

| Test                               | Description                   |
| ---------------------------------- | ----------------------------- |
| `test_withdraw_token_2022_success` | Withdraw with Token-2022 mint |

### Arbiter Tests

| Test                                                       | Description                            | Expected Error                  |
| ---------------------------------------------------------- | -------------------------------------- | ------------------------------- |
| `test_withdraw_with_arbiter_success`                       | Withdraw with arbiter co-signer        | --                              |
| `test_withdraw_with_arbiter_missing_signer`                | Arbiter not signed                     | `Custom(14)` / `InvalidArbiter` |
| `test_withdraw_with_arbiter_wrong_address`                 | Wrong arbiter address                  | `Custom(14)` / `InvalidArbiter` |
| `test_withdraw_with_arbiter_no_remaining_accounts`         | No arbiter in remaining accounts       | `Custom(14)` / `InvalidArbiter` |
| `test_withdraw_without_arbiter_extension_no_signer_needed` | No arbiter extension, no signer needed | --                              |

### Combined Extension Tests

| Test                                                   | Description                   |
| ------------------------------------------------------ | ----------------------------- |
| `test_withdraw_with_arbiter_and_hook_success`          | Arbiter + hook combo          |
| `test_withdraw_with_arbiter_and_timelock_success`      | Arbiter + timelock combo      |
| `test_withdraw_with_arbiter_hook_and_timelock_success` | All three extensions combined |

### Happy Path Tests

| Test                                             | Description                           |
| ------------------------------------------------ | ------------------------------------- |
| `test_withdraw_no_timelock_success`              | Simple withdraw without timelock      |
| `test_withdraw_success`                          | Validates token balance changes       |
| `test_withdraw_closes_receipt`                   | Receipt account closed after withdraw |
| `test_withdraw_transfers_tokens`                 | Full balance validation               |
| `test_withdraw_returns_rent_to_payer`            | Rent returned to payer                |
| `test_withdraw_returns_rent_to_custom_recipient` | Rent returned to custom recipient     |

---

## AllowMint

**File:** `test_allow_mint.rs`

### Error Tests

| Test                                             | Description                      | Expected Error             |
| ------------------------------------------------ | -------------------------------- | -------------------------- |
| `test_allow_mint_missing_admin_signer`           | Admin not signed                 | `MissingRequiredSignature` |
| `test_allow_mint_allowed_mint_not_writable`      | AllowedMint not writable         | `Immutable`                |
| `test_allow_mint_vault_not_writable`             | Vault not writable               | `Immutable`                |
| `test_allow_mint_wrong_system_program`           | Invalid system program           | `IncorrectProgramId`       |
| `test_allow_mint_wrong_current_program`          | Invalid escrow program           | `IncorrectProgramId`       |
| `test_allow_mint_wrong_token_program`            | Invalid token program            | `IncorrectProgramId`       |
| `test_allow_mint_wrong_associated_token_program` | Invalid associated token program | `IncorrectProgramId`       |
| `test_allow_mint_invalid_bump`                   | Incorrect PDA bump               | `InvalidSeeds`             |
| `test_allow_mint_wrong_escrow`                   | Escrow not owned by program      | `InvalidAccountOwner`      |
| `test_allow_mint_wrong_mint`                     | Mint not owned by token program  | `InvalidAccountOwner`      |
| `test_allow_mint_duplicate`                      | Duplicate AllowMint              | `AlreadyProcessed`         |

### Custom Error Tests

| Test                                      | Description                   | Expected Error                        |
| ----------------------------------------- | ----------------------------- | ------------------------------------- |
| `test_allow_mint_wrong_admin`             | Non-admin tries to allow mint | `Custom(1)` / `InvalidAdmin`          |
| `test_allow_mint_invalid_event_authority` | Wrong event authority PDA     | `Custom(2)` / `InvalidEventAuthority` |

### Token-2022 Extension Tests

| Test                                                            | Description                            | Expected Error                              |
| --------------------------------------------------------------- | -------------------------------------- | ------------------------------------------- |
| `test_allow_mint_rejects_permanent_delegate`                    | Mint has PermanentDelegate             | `Custom(9)` / `PermanentDelegateNotAllowed` |
| `test_allow_mint_rejects_non_transferable`                      | Mint has NonTransferable               | `Custom(10)` / `NonTransferableNotAllowed`  |
| `test_allow_mint_rejects_pausable`                              | Mint has Pausable                      | `Custom(11)` / `PausableNotAllowed`         |
| `test_allow_mint_rejects_escrow_blocked_extension`              | Mint blocked by escrow extension rules | `Custom(8)` / `MintNotAllowed`              |
| `test_allow_mint_accepts_mint_without_escrow_blocked_extension` | Mint passes with non-blocked extension | --                                          |
| `test_allow_mint_token_2022_success`                            | Token-2022 mint allowed                | --                                          |

### Happy Path Tests

| Test                                           | Description                              |
| ---------------------------------------------- | ---------------------------------------- |
| `test_allow_mint_success`                      | Successfully allows mint                 |
| `test_allow_mint_multiple_mints`               | Allow two different mints on same escrow |
| `test_allow_mint_creates_vault_ata`            | Verifies vault ATA creation              |
| `test_allow_mint_creates_vault_ata_token_2022` | Verifies Token-2022 vault ATA creation   |

---

## BlockMint

**File:** `test_block_mint.rs`

### Error Tests

| Test                                           | Description                       | Expected Error             |
| ---------------------------------------------- | --------------------------------- | -------------------------- |
| `test_block_mint_missing_admin_signer`         | Admin not signed                  | `MissingRequiredSignature` |
| `test_block_mint_allowed_mint_not_writable`    | AllowedMint not writable          | `Immutable`                |
| `test_block_mint_wrong_current_program`        | Invalid escrow program            | `IncorrectProgramId`       |
| `test_block_mint_wrong_token_program`          | Invalid token program             | `IncorrectProgramId`       |
| `test_block_mint_wrong_escrow`                 | Escrow not owned by program       | `InvalidAccountOwner`      |
| `test_block_mint_wrong_mint`                   | Mint not owned by token program   | `InvalidAccountOwner`      |
| `test_block_mint_wrong_allowed_mint`           | AllowedMint not owned by program  | `InvalidAccountOwner`      |
| `test_block_mint_allowed_mint_escrow_mismatch` | AllowedMint from different escrow | `InvalidSeeds`             |

### Custom Error Tests

| Test                                      | Description                   | Expected Error                        |
| ----------------------------------------- | ----------------------------- | ------------------------------------- |
| `test_block_mint_wrong_admin`             | Non-admin tries to block mint | `Custom(1)` / `InvalidAdmin`          |
| `test_block_mint_invalid_event_authority` | Wrong event authority PDA     | `Custom(2)` / `InvalidEventAuthority` |

### Happy Path Tests

| Test                                     | Description                           |
| ---------------------------------------- | ------------------------------------- |
| `test_block_mint_success`                | Successfully blocks mint              |
| `test_block_mint_rent_returned_to_payer` | Rent returned to payer after blocking |
| `test_block_multiple_mints_same_escrow`  | Block two mints from same escrow      |
| `test_block_mint_token_2022_success`     | Token-2022 block mint                 |

---

## BlockTokenExtension

**File:** `test_block_token_extension.rs`

### Error Tests

| Test                                                 | Description                   | Expected Error             |
| ---------------------------------------------------- | ----------------------------- | -------------------------- |
| `test_block_token_extension_missing_admin_signer`    | Admin not signed              | `MissingRequiredSignature` |
| `test_block_token_extension_extensions_not_writable` | Extensions not writable       | `Immutable`                |
| `test_block_token_extension_wrong_system_program`    | Invalid system program        | `IncorrectProgramId`       |
| `test_block_token_extension_wrong_escrow_program`    | Invalid escrow program        | `IncorrectProgramId`       |
| `test_block_token_extension_invalid_extensions_bump` | Incorrect extensions PDA bump | `InvalidSeeds`             |
| `test_block_token_extension_empty_data`              | Empty instruction data        | `InvalidInstructionData`   |
| `test_block_token_extension_truncated_data`          | Truncated instruction data    | `InvalidInstructionData`   |

### Custom Error Tests

| Test                                                     | Description                        | Expected Error                                |
| -------------------------------------------------------- | ---------------------------------- | --------------------------------------------- |
| `test_block_token_extension_wrong_admin`                 | Non-admin tries to block extension | `Custom(1)` / `InvalidAdmin`                  |
| `test_block_token_extension_invalid_event_authority`     | Wrong event authority PDA          | `Custom(2)` / `InvalidEventAuthority`         |
| `test_block_token_extension_escrow_not_owned_by_program` | Escrow not owned by program        | `InvalidAccountOwner`                         |
| `test_block_token_extension_duplicate_extension`         | Extension already blocked          | `Custom(12)` / `TokenExtensionAlreadyBlocked` |

### Happy Path Tests

| Test                                                     | Description                           |
| -------------------------------------------------------- | ------------------------------------- |
| `test_block_token_extension_success`                     | Successfully blocks token extension   |
| `test_block_token_extension_success_single`              | Single specific extension (42u16)     |
| `test_block_token_extension_success_many_extensions`     | 20 extensions one at a time           |
| `test_block_token_extension_success_multiple_extensions` | Timelock + 3 blocked token extensions |

---

## SetArbiter

**File:** `test_set_arbiter.rs`

### Error Tests

| Test                                       | Description                     | Expected Error              |
| ------------------------------------------ | ------------------------------- | --------------------------- |
| `test_set_arbiter_missing_admin_signer`    | Admin not signed                | `MissingRequiredSignature`  |
| `test_set_arbiter_missing_arbiter_signer`  | Arbiter not signed              | `MissingRequiredSignature`  |
| `test_set_arbiter_extensions_not_writable` | Extensions not writable         | `Immutable`                 |
| `test_set_arbiter_wrong_system_program`    | Invalid system program          | `IncorrectProgramId`        |
| `test_set_arbiter_wrong_escrow_program`    | Invalid escrow program          | `IncorrectProgramId`        |
| `test_set_arbiter_invalid_extensions_bump` | Incorrect extensions PDA bump   | `InvalidSeeds`              |
| `test_set_arbiter_empty_data`              | Empty instruction data          | `InvalidInstructionData`    |
| `test_set_arbiter_truncated_data`          | Truncated instruction data      | `InvalidInstructionData`    |
| `test_set_arbiter_duplicate_extension`     | Arbiter already set (immutable) | `AccountAlreadyInitialized` |

### Custom Error Tests

| Test                                           | Description                    | Expected Error                        |
| ---------------------------------------------- | ------------------------------ | ------------------------------------- |
| `test_set_arbiter_wrong_admin`                 | Non-admin tries to set arbiter | `Custom(1)` / `InvalidAdmin`          |
| `test_set_arbiter_invalid_event_authority`     | Wrong event authority PDA      | `Custom(2)` / `InvalidEventAuthority` |
| `test_set_arbiter_escrow_not_owned_by_program` | Escrow not owned by program    | `InvalidAccountOwner`                 |

### Happy Path Tests

| Test                       | Description                         |
| -------------------------- | ----------------------------------- |
| `test_set_arbiter_success` | Successfully sets arbiter extension |

### Combined Extension Tests

| Test                                 | Description                        |
| ------------------------------------ | ---------------------------------- |
| `test_add_timelock_then_set_arbiter` | Timelock first, then arbiter       |
| `test_set_arbiter_then_set_hook`     | Arbiter first, then hook           |
| `test_all_three_extensions`          | Timelock + hook + arbiter combined |

---

## Unit Tests

| Module               | File(s)                                                                                                                                                                                                                                | Test Count | Coverage                                                                                    |
| -------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------------------------------------------- |
| state                | `escrow.rs`, `receipt.rs`, `allowed_mint.rs`, `escrow_extensions.rs`                                                                                                                                                                   | 36         | Serialization, deserialization, seeds, invalid data, admin validation, depositor validation |
| state/extensions     | `timelock.rs`, `hook.rs`, `arbiter.rs`, `block_token_extension.rs`                                                                                                                                                                     | 19         | Extension data round-trips, is_enabled, add_extension, duplicate detection, boundary values |
| traits               | `account.rs`, `instruction.rs`, `event.rs`, `pda.rs`                                                                                                                                                                                   | 24         | Discriminators, mutable references, unchecked deserialization, roundtrips, PDA derivation   |
| utils                | `tlv.rs`, `macros.rs`                                                                                                                                                                                                                  | 11         | TLV read/write, require_len!, validate_discriminator!, multi-extension parsing              |
| instructions         | `create_escrow/data.rs`, `deposit/data.rs`, `withdraw/data.rs`, `update_admin/data.rs`, `allow_mint/data.rs`, `block_mint/data.rs`, `add_timelock/data.rs`, `set_hook/data.rs`, `set_arbiter/data.rs`, `block_token_extension/data.rs` | 20         | Instruction data parsing, length validation, zero amount rejection                          |
| events               | `create_escrow.rs`, `deposit.rs`, `withdraw.rs`, `admin_update.rs`, `allow_mint.rs`, `block_mint.rs`, `timelock_added.rs`, `hook_set.rs`, `arbiter_set.rs`, `token_extension_blocked.rs`                                               | 25         | Event serialization, discriminator prefixes, byte layout                                    |
| errors               | `errors.rs`                                                                                                                                                                                                                            | 1          | Error code conversion (variants 0-5 only)                                                   |
| **Total Unit Tests** |                                                                                                                                                                                                                                        | **136**    |                                                                                             |

---

## Test Categories

### 1. Signer Validation

Tests ensure all required signers are validated:

- Admin signer checked in: CreateEscrow, AllowMint, BlockMint, AddTimelock, SetHook, BlockTokenExtension, SetArbiter, UpdateAdmin
- Depositor signer checked in: Deposit
- Withdrawer signer checked in: Withdraw
- Escrow seed signer checked in: CreateEscrow
- Receipt seed signer checked in: Deposit
- New admin signer checked in: UpdateAdmin
- Arbiter signer checked in: SetArbiter

### 2. Writable Account Validation

Tests verify writable requirements for:

- Escrow account: CreateEscrow, UpdateAdmin
- Extensions account: AddTimelock, SetHook, BlockTokenExtension, SetArbiter
- Receipt account: Deposit, Withdraw
- Vault account: Deposit, Withdraw, AllowMint
- Token accounts: Deposit (depositor), Withdraw (withdrawer)
- AllowedMint account: AllowMint, BlockMint

### 3. Program Validation

Tests verify program ID checks for:

- System program: CreateEscrow, Deposit, Withdraw, AllowMint, AddTimelock, SetHook, BlockTokenExtension, SetArbiter
- Current (escrow) program: all instructions
- Token program: Deposit, Withdraw, AllowMint, BlockMint
- Associated token program: AllowMint

### 4. PDA Validation

Tests verify PDA bump/seed validation for:

- Escrow PDA: CreateEscrow
- Receipt PDA: Deposit
- Extensions PDA: AddTimelock, SetHook, BlockTokenExtension, SetArbiter, Withdraw
- AllowedMint PDA: AllowMint, BlockMint
- Event authority PDA: all instructions

### 5. Admin Authorization

Tests verify admin authorization in: UpdateAdmin, AllowMint, BlockMint, AddTimelock, SetHook, BlockTokenExtension, SetArbiter. Includes tests for admin transfer (old admin rejected after update).

### 6. Token Operations

Tests cover:

- SPL Token deposits and withdrawals
- Token-2022 deposits and withdrawals
- ATA validation (wrong vault, wrong depositor/withdrawer token account)
- Mint decimals handling

### 7. Extension System

Tests cover:

- Timelock: creation, duplicate prevention, various durations (0 to u64::MAX)
- Hook: creation, duplicate prevention, zero-address (disabled)
- Arbiter: creation, duplicate prevention (immutability)
- BlockTokenExtension: creation, duplicate extension rejection, single/many extensions
- Combined extensions: all pairwise and triple combinations
- Extension count tracking

### 8. Timelock Enforcement

Tests cover:

- Withdraw blocked before timelock expires
- Withdraw succeeds after timelock expires (via clock warp)
- Zero-duration timelock succeeds immediately
- Timelock combined with arbiter and hook

### 9. Hook Integration

Tests cover:

- Hook allows deposit/withdraw
- Hook rejects deposit/withdraw
- Hook program mismatch detection
- Hook combined with arbiter and timelock

### 10. Security Protections

Tests cover:

- Re-initialization prevention (CreateEscrow)
- Double withdraw prevention
- Cross-escrow receipt replay (receipt from escrow A used with escrow B)
- Reactivated account rejection (fake account data after close)
- Account ownership verification across all instructions
- Token-2022 dangerous extension blocking (PermanentDelegate, NonTransferable, Pausable)
- Custom extension blocking per escrow

### 11. Arbiter Validation

Tests cover:

- Arbiter co-signature required for withdraw
- Missing arbiter signer rejected
- Wrong arbiter address rejected
- No remaining accounts when arbiter expected
- No arbiter needed when extension not set
- Arbiter combined with hook and timelock

---

**Grand Total: 136 unit tests + 178 integration tests = 314 total tests**
