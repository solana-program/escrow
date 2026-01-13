# Escrow Extension Generator Plugin

A Claude Code plugin that helps you add new extensions to the Solana escrow program.

## Usage

### Command

```
/add-extension
```

### Agent

The `extension-generator` agent automatically activates when you mention adding a new extension to the escrow program.

## What It Generates

When you add a new extension, the plugin generates:

### Program Files

- **State**: Extension data struct in `program/src/state/extensions/`
- **Instructions**: Complete instruction structure (accounts, data, instruction)
- **Processor**: Instruction handler logic
- **Events**: Event struct for extension addition
- **Errors**: Custom error types (if needed)
- **TLV Helpers**: TLV writer/reader methods

### Test Files

- **Fixtures**: Test fixture builder
- **Integration Tests**: Comprehensive test suite

### Integration Updates

- Instruction discriminator enum
- Event discriminator enum
- ExtensionType enum
- Entrypoint routing
- Extension validation dispatch
- Module exports

## Extension Pattern

Extensions follow a consistent pattern:

1. **Extension Data**: Struct stored in TLV format with fixed size
2. **Instruction**: `Add{ExtensionName}` with standard accounts + extension-specific data
3. **Processor**: Validates admin, creates/updates extensions PDA, appends TLV entry
4. **Event**: Emits `{ExtensionName}Added` event
5. **Validation**: Optional `validate()` method for runtime checks

## Example

To add a rate limit extension:

1. Run `/add-extension` or mention "I want to add a rate limit extension"
2. Answer questions about:
    - Extension name: `rate_limit`
    - Data fields: `max_withdrawals_per_day: u8, window_start: u64`
    - Validation: Check withdrawal count in time window
    - Errors: `RateLimitExceeded`
3. Review generated code
4. Validate with clarifying questions
5. Run `just test` - the agent will loop until all tests pass
6. All tests must pass - no skipped or commented tests allowed

## Requirements

- Extension name (snake_case)
- Data structure (fields and types)
- Validation logic (when/how to validate)
- Custom errors (if any)
- Event payload

The agent will guide you through gathering all necessary information.

## Testing Requirements

After generating the extension code, the agent will:

1. Run `just test` to build and test everything
2. Fix any compilation errors or test failures
3. Loop with `just test` until **all tests pass**
4. **Never skip or comment out tests** - always fix the root cause

Common issues the agent will fix:

- Missing imports (especially `alloc::vec::Vec`)
- Incorrect data byte offsets (discriminator bytes)
- Struct padding issues
- Wrong error types in test assertions
- Missing test helpers
