# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Required Versions

- **Rust**: See `rust-toolchain.toml` (auto-installed by rustup)
- **Node.js**: See `.nvmrc` (use `nvm use` or `fnm use`)
- **pnpm**: See `package.json` `packageManager` field

## Build Commands

```bash
# Full build (generates IDL → clients → program binary)
just build

# Individual steps
just generate-idl          # Generate IDL from Rust code via Codama
just generate-clients      # Generate TypeScript + Rust clients from IDL
cd program && cargo-build-sbf  # Build .so binary only

# Formatting and linting
just fmt                   # cargo fmt, clippy, prettier
just check                 # cargo check + prettier check

# Testing (integration tests use LiteSVM)
just integration-test      # cargo test -p tests-escrow-program

# Run a single test
cargo test -p tests-escrow-program test_create_escrow_success

# Dependencies
just install               # pnpm install
```

## Architecture

This is a Solana program using **Pinocchio** (lightweight `no_std` framework) with **Codama** for IDL-driven client generation.

### Code Flow

```
program/src/lib.rs (declares ID, exports modules)
    ↓
program/src/entrypoint.rs (routes by 1-byte discriminator)
    ↓
program/src/processors/*.rs (instruction handlers)
    ↓
program/src/state/*.rs (PDA account structs)
```

### Client Generation Pipeline

```
Rust code with #[derive(Codama*)] macros
    ↓
program/build.rs → idl/escrow_program.json
    ↓
scripts/generate-clients.ts (applies Codama transformations)
    ↓
clients/rust/src/generated/   (auto-generated, do not edit)
clients/typescript/src/generated/  (auto-generated, do not edit)
```

### Key Modules

- **`program/src/traits/`**: Core traits + discriminator enums
    - `account.rs`: `Discriminator`, `AccountSerialize`, `AccountDeserialize` (zero-copy via pointer casting)
    - `instruction.rs`: `Instruction`, `InstructionData`, `InstructionAccounts` (marker trait), `EscrowInstructionDiscriminators` enum (`CreateEscrow = 0`, `EmitEvent = 228`)
    - `pda.rs`: `PdaSeeds` trait for PDA derivation
    - `event.rs`: `EventDiscriminator`, `EventSerialize` + `EVENT_DISCRIMINATOR_LEN`
- **`program/src/utils/`**: Reusable utilities
    - `macros.rs`: Validation macros (`require_len!`, `validate_discriminator!`, `assert_no_padding!`)
    - `account_utils.rs`: Account validation helpers (`verify_signer`, `verify_writable`, `verify_owned_by`, `verify_system_account`, `verify_current_program_account`)
    - `program_utils.rs`: Program validation helpers (`verify_system_program`, `verify_current_program`)
    - `pda_utils.rs`: PDA account creation utilities (`create_pda_account`, `create_pda_account_idempotent`)
    - `event_utils.rs`: Event emission helpers (`emit_event`, `verify_event_authority`)
- **`program/src/events/`**: Event structs and constants
    - `shared.rs`: Event constants (`EVENT_IX_TAG_LE`, `EVENT_AUTHORITY_SEED`) and `event_authority_pda` module (compile-time PDA via `crate::ID.as_array()`)
    - `create_escrow.rs`: Event struct implementing `EventSerialize`
- **`program/src/instructions/`**: Instruction definitions with `TryFrom` pattern
    - Each instruction has its own directory: `accounts.rs`, `data.rs`, `instruction.rs` (no code in `mod.rs`)
    - `definition.rs`: Codama IDL generation (keep for client generation)
- **`program/src/state/`**: Account structs implementing trait hierarchy
- **`program/src/processors/`**: Instruction handlers
    - `create_escrow.rs`: Main escrow creation logic
    - `emit_event.rs`: CPI-based event emission (validates event authority PDA signer)
- **`scripts/lib/updates/`**: Codama transformations applied during client generation

### Trait-Based Patterns

**Account Traits** (`program/src/traits/account.rs`):

- `Discriminator`: Single-byte account type discriminator
- `AccountDeserialize`: Zero-copy `from_bytes()` via pointer casting
- `AccountSerialize`: `to_bytes()` with discriminator prefix

**PDA Seeds** (`program/src/traits/pda.rs`):

```rust
impl PdaSeeds for Escrow {
    const PREFIX: &'static [u8] = b"escrow";
    fn seeds(&self) -> Vec<&[u8]>;
    fn seeds_with_bump<'a>(&'a self, bump: &'a [u8; 1]) -> Vec<Seed<'a>>;
}
```

**Instruction TryFrom** (`program/src/instructions/create_escrow/`):

```rust
// All validation in TryFrom - processor only has business logic
let ix = CreateEscrow::try_from((instruction_data, accounts))?;
```

Structure per instruction directory:

- `accounts.rs`: `CreateEscrowAccounts` implements `TryFrom<&[AccountView]>` + `InstructionAccounts` marker trait
- `data.rs`: `CreateEscrowData` implements `TryFrom<&[u8]>` + `InstructionData` trait
- `instruction.rs`: `CreateEscrow` struct combining accounts + data, implements `Instruction` trait

**Macros** (`program/src/utils/macros.rs`):

```rust
// Validation macros for readability
require_len!(data, Self::LEN);
validate_discriminator!(data, Self::DISCRIMINATOR);

// Compile-time size assertion for zero-copy structs
assert_no_padding!(Escrow, 1 + 1 + 32 + 32);
```

### Testing with LiteSVM

Tests in `tests/integration-tests/` use LiteSVM for fast local execution. The test context (`src/utils/`) loads the compiled `.so` from `target/deploy/`.

**Test Utilities** (`tests/integration-tests/src/utils/`):

- `setup.rs`: `TestContext` struct - LiteSVM initialization, transaction helpers
- `traits.rs`: `TestInstruction` wrapper with builder pattern, `InstructionTestFixture` trait
- `test_helpers.rs`: Generic test functions (`test_missing_signer`, `test_not_writable`, `test_wrong_system_program`, etc.)
- `assertions.rs`: `assert_account_exists`, `assert_custom_error`, `assert_escrow_account`
- `cu_utils.rs`: `CuTracker` for compute unit profiling (enabled via `CU_TRACKING` env var)
- `pda_utils.rs`: PDA derivation helpers for tests

**Fixtures** (`tests/integration-tests/src/fixtures/`): Instruction test builders implementing `InstructionTestFixture`.

```rust
// Example: Build instruction using auto-generated client
let instruction = CreatesEscrowBuilder::new()
    .payer(context.payer.pubkey())
    .admin(admin.pubkey())
    .escrow_seed(escrow_seed.pubkey())
    .escrow(escrow_pda)
    .bump(bump)
    .instruction();
```

### PDA Pattern

PDAs use the `PdaSeeds` trait on state structs (seeds co-located with data):

- Escrow: `[b"escrow", escrow_seed.as_ref()]` via `Escrow::PREFIX` + `seeds_with_bump()`

### Program ID

`GokvZqD2yP696rzNBNbQvcZ4VsLW7jNvFXU1kW9m7k83`

## Conventions

- **No code in `mod.rs`**: Only module declarations and re-exports
- **No magic numbers**: Use named constants (e.g., `DATA_LEN`, `EVENT_DISCRIMINATOR_LEN`)
- **Compile-time assertions**: Use `assert_no_padding!` for zero-copy structs to catch padding issues
- **Single source of truth**: Reference `crate::ID.as_array()` for program ID bytes instead of duplicating
- **Marker traits**: `InstructionAccounts` is a marker trait with bounds but no methods - declares type constraints
- **Validation in TryFrom**: All account/data validation happens in `TryFrom` impls, processors only contain business logic

## Workspace Structure

- `program/` - Main Solana program (Pinocchio)
- `clients/rust/` - Auto-generated Rust client
- `clients/typescript/` - Auto-generated TypeScript client
- `tests/integration-tests/` - LiteSVM-based integration tests
- `idl/` - Generated Codama IDL JSON
- `scripts/` - Client generation scripts
