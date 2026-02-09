use codama::CodamaInstructions;
use pinocchio::Address;

/// Instructions for the Escrow Program.
#[allow(clippy::large_enum_variant)]
#[repr(C, u8)]
#[derive(Clone, Debug, PartialEq, CodamaInstructions)]
pub enum EscrowProgramInstruction {
    /// Create a new escrow with the specified admin.
    #[codama(account(name = "payer", docs = "Pays for escrow account creation", signer, writable))]
    #[codama(account(name = "admin", docs = "Admin authority for the escrow", signer))]
    #[codama(account(name = "escrow_seed", docs = "Random keypair seed for escrow PDA derivation", signer))]
    #[codama(account(name = "escrow", docs = "Escrow PDA account to be created", writable))]
    #[codama(account(name = "system_program", docs = "System program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    CreatesEscrow {
        /// Bump for the escrow PDA
        bump: u8,
    } = 0,

    /// Add timelock extension to an escrow.
    #[codama(account(name = "payer", docs = "Pays for extensions account creation", signer, writable))]
    #[codama(account(name = "admin", docs = "Admin authority for the escrow", signer))]
    #[codama(account(name = "escrow", docs = "Escrow account to add timelock to"))]
    #[codama(account(name = "extensions", docs = "Extensions PDA account to store timelock config", writable))]
    #[codama(account(name = "system_program", docs = "System program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    AddTimelock {
        /// Bump for extensions PDA
        extensions_bump: u8,
        /// Lock duration in seconds from deposit
        lock_duration: u64,
    } = 1,

    /// Set hook program on an escrow.
    #[codama(account(name = "payer", docs = "Pays for extensions account creation", signer, writable))]
    #[codama(account(name = "admin", docs = "Admin authority for the escrow", signer))]
    #[codama(account(name = "escrow", docs = "Escrow account to set hook on"))]
    #[codama(account(name = "extensions", docs = "Extensions PDA account to store hook config", writable))]
    #[codama(account(name = "system_program", docs = "System program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    SetHook {
        /// Bump for extensions PDA
        extensions_bump: u8,
        /// Hook program address
        hook_program: Address,
    } = 2,

    /// Deposit tokens into an escrow vault and create a receipt.
    #[codama(account(name = "payer", docs = "Pays for receipt account creation", signer, writable))]
    #[codama(account(name = "depositor", docs = "Authority depositing tokens", signer))]
    #[codama(account(name = "escrow", docs = "Escrow account to deposit into"))]
    #[codama(account(name = "allowed_mint", docs = "Allowed mint PDA proving this mint is permitted"))]
    #[codama(account(name = "receipt_seed", docs = "Random keypair seed for receipt PDA derivation", signer))]
    #[codama(account(name = "receipt", docs = "Deposit receipt PDA to be created", writable))]
    #[codama(account(name = "vault", docs = "Escrow vault token account to receive tokens", writable))]
    #[codama(account(
        name = "depositor_token_account",
        docs = "Depositor's token account to transfer from",
        writable
    ))]
    #[codama(account(name = "mint", docs = "Token mint of the deposited tokens"))]
    #[codama(account(name = "token_program", docs = "SPL Token program"))]
    #[codama(account(name = "system_program", docs = "System program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    #[codama(account(name = "extensions", docs = "Extensions PDA for escrow configuration"))]
    Deposit {
        /// Bump for the deposit receipt PDA
        bump: u8,
        /// Amount of tokens to deposit
        amount: u64,
    } = 3,

    /// Update the admin on an escrow.
    #[codama(account(name = "admin", docs = "Current admin authority for the escrow", signer))]
    #[codama(account(name = "new_admin", docs = "New admin authority to transfer ownership to", signer))]
    #[codama(account(name = "escrow", docs = "Escrow account to update admin on", writable))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    UpdateAdmin {} = 4,

    /// Withdraw tokens from an escrow vault back to the original depositor.
    #[codama(account(name = "rent_recipient", docs = "Receives rent from closed receipt account", writable))]
    #[codama(account(name = "withdrawer", docs = "Authority withdrawing tokens", signer))]
    #[codama(account(name = "escrow", docs = "Escrow account to withdraw from"))]
    #[codama(account(name = "extensions", docs = "Extensions PDA for escrow configuration"))]
    #[codama(account(name = "receipt", docs = "Deposit receipt to close upon withdrawal", writable))]
    #[codama(account(name = "vault", docs = "Escrow vault token account to transfer from", writable))]
    #[codama(account(
        name = "withdrawer_token_account",
        docs = "Withdrawer's token account to receive tokens",
        writable
    ))]
    #[codama(account(name = "mint", docs = "Token mint of the withdrawn tokens"))]
    #[codama(account(name = "token_program", docs = "SPL Token program"))]
    #[codama(account(name = "system_program", docs = "System program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    Withdraw {} = 5,

    /// Allow a token mint for deposits into an escrow.
    /// Also creates the vault ATA for the escrow to hold tokens of this mint.
    #[codama(account(name = "payer", docs = "Pays for allowed mint and vault account creation", signer, writable))]
    #[codama(account(name = "admin", docs = "Admin authority for the escrow", signer))]
    #[codama(account(name = "escrow", docs = "Escrow account to allow mint on"))]
    #[codama(account(name = "escrow_extensions", docs = "Extensions PDA for escrow configuration"))]
    #[codama(account(name = "mint", docs = "Token mint to allow for deposits"))]
    #[codama(account(name = "allowed_mint", docs = "Allowed mint PDA to be created", writable))]
    #[codama(account(name = "vault", docs = "Escrow vault ATA to be created for this mint", writable))]
    #[codama(account(name = "token_program", docs = "SPL Token program"))]
    #[codama(account(name = "associated_token_program", docs = "Associated Token program for vault creation"))]
    #[codama(account(name = "system_program", docs = "System program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    AllowMint {
        /// Bump for the allowed_mint PDA
        bump: u8,
    } = 6,

    /// Block a token mint from deposits into an escrow.
    #[codama(account(name = "admin", docs = "Admin authority for the escrow", signer))]
    #[codama(account(name = "rent_recipient", docs = "Receives rent from closed allowed mint account", writable))]
    #[codama(account(name = "escrow", docs = "Escrow account to block mint on"))]
    #[codama(account(name = "mint", docs = "Token mint to block from deposits"))]
    #[codama(account(name = "allowed_mint", docs = "Allowed mint PDA to be closed", writable))]
    #[codama(account(name = "token_program", docs = "SPL Token program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    BlockMint {} = 7,

    /// Block a token extension for an escrow.
    #[codama(account(name = "payer", docs = "Pays for extensions account creation", signer, writable))]
    #[codama(account(name = "admin", docs = "Admin authority for the escrow", signer))]
    #[codama(account(name = "escrow", docs = "Escrow account to block extension on"))]
    #[codama(account(name = "extensions", docs = "Extensions PDA account to store blocked extensions", writable))]
    #[codama(account(name = "system_program", docs = "System program"))]
    #[codama(account(name = "event_authority", docs = "Event authority PDA for CPI event emission"))]
    #[codama(account(name = "escrow_program", docs = "Escrow program for CPI event emission"))]
    BlockTokenExtension {
        /// Bump for extensions PDA
        extensions_bump: u8,
        /// Token-2022 ExtensionType value to block
        blocked_extension: u16,
    } = 8,

    /// Set an arbiter on an escrow. The arbiter must sign withdrawal transactions.
    /// This is immutable — once set, the arbiter cannot be changed.
    #[codama(account(name = "payer", signer, writable))]
    #[codama(account(name = "admin", signer))]
    #[codama(account(name = "arbiter", signer))]
    #[codama(account(name = "escrow"))]
    #[codama(account(name = "extensions", writable))]
    #[codama(account(name = "system_program"))]
    #[codama(account(name = "event_authority"))]
    #[codama(account(name = "escrow_program"))]
    SetArbiter {
        /// Bump for extensions PDA
        extensions_bump: u8,
    } = 9,

    /// Invoked via CPI to emit event data in instruction args (prevents log truncation).
    #[codama(account(name = "event_authority", docs = "Event authority PDA that must sign via CPI", signer))]
    EmitEvent {} = 228,
}
