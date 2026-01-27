use codama::CodamaInstructions;
use pinocchio::Address;

/// Instructions for the Escrow Program.
#[allow(clippy::large_enum_variant)]
#[repr(C, u8)]
#[derive(Clone, Debug, PartialEq, CodamaInstructions)]
pub enum EscrowProgramInstruction {
    /// Create a new escrow with the specified admin.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "escrow_seed", signer))]
    #[codama(account(name = "escrow", writable))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    CreatesEscrow {
        /// Bump for the escrow PDA
        bump: u8,
    } = 0,

    /// Add timelock extension to an escrow.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "extensions", writable))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    AddTimelock {
        /// Bump for extensions PDA
        extensions_bump: u8,
        /// Lock duration in seconds from deposit
        lock_duration: u64,
    } = 1,

    /// Set hook program on an escrow.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "extensions", writable))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    SetHook {
        /// Bump for extensions PDA
        extensions_bump: u8,
        /// Hook program address
        hook_program: Address,
    } = 2,

    /// Deposit tokens into an escrow vault and create a receipt.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "depositor", signer))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "allowed_mint"))]
    #[codama(account(name = "receipt_seed", signer))]
    #[codama(account(name = "receipt", writable))]
    #[codama(account(name = "vault", writable))]
    #[codama(account(name = "depositor_token_account", writable))]
    #[codama(account(name = "mint"))]
    #[codama(account(name = "token_program"))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    #[codama(account(name = "extensions"))]
    Deposit {
        /// Bump for the deposit receipt PDA
        bump: u8,
        /// Amount of tokens to deposit
        amount: u64,
    } = 3,

    /// Update the admin on an escrow.
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "new_admin", signer))]
    #[codama(account(name = "escrow", writable))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    UpdateAdmin {} = 4,

    /// Withdraw tokens from an escrow vault back to the original depositor.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "withdrawer", signer))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "extensions"))]
    #[codama(account(name = "receipt", writable))]
    #[codama(account(name = "vault", writable))]
    #[codama(account(name = "withdrawer_token_account", writable))]
    #[codama(account(name = "mint"))]
    #[codama(account(name = "token_program"))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    Withdraw {} = 5,

    /// Allow a token mint for deposits into an escrow.
    /// Also creates the vault ATA for the escrow to hold tokens of this mint.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "escrow_extensions"))]
    #[codama(account(name = "mint"))]
    #[codama(account(name = "allowed_mint", writable))]
    #[codama(account(name = "vault", writable))]
    #[codama(account(name = "token_program"))]
    #[codama(account(name = "associated_token_program"))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    AllowMint {
        /// Bump for the allowed_mint PDA
        bump: u8,
    } = 6,

    /// Block a token mint from deposits into an escrow.
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "payer", writable))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "mint"))]
    #[codama(account(name = "allowed_mint", writable))]
    #[codama(account(name = "token_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    BlockMint {} = 7,

    /// Block a token extension for an escrow.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "extensions", writable))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    BlockTokenExtension {
        /// Bump for extensions PDA
        extensions_bump: u8,
        /// Token-2022 ExtensionType value to block
        blocked_extension: u16,
    } = 8,

    /// Invoked via CPI to emit event data in instruction args (prevents log truncation).
    #[codama(account(name = "event_authority", signer))]
    EmitEvent {} = 228,
}
