use crate::utils::traits::{InstructionTestFixture, TestInstruction};
use crate::utils::{find_allowed_mint_pda, find_escrow_pda, find_extensions_pda, TestContext};
use escrow_program_client::instructions::{AllowMintBuilder, BlockTokenExtensionBuilder, CreatesEscrowBuilder};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token_2022::{extension::ExtensionType, ID as TOKEN_2022_PROGRAM_ID};
use spl_token_interface::ID as TOKEN_PROGRAM_ID;

pub struct AllowMintSetup {
    pub escrow_pda: Pubkey,
    pub escrow_extensions_pda: Pubkey,
    pub admin: Keypair,
    pub mint: Keypair,
    pub allowed_mint_pda: Pubkey,
    pub allowed_mint_bump: u8,
    pub token_program: Pubkey,
}

impl AllowMintSetup {
    pub fn builder(ctx: &mut TestContext) -> AllowMintSetupBuilder<'_> {
        AllowMintSetupBuilder::new(ctx)
    }

    pub fn new(ctx: &mut TestContext) -> Self {
        Self::builder(ctx).build()
    }

    pub fn new_token_2022(ctx: &mut TestContext) -> Self {
        Self::builder(ctx).token_2022().build()
    }

    pub fn new_with_extension(ctx: &mut TestContext, extension_type: ExtensionType) -> Self {
        Self::builder(ctx).mint_extension(extension_type).build()
    }

    pub fn new_with_escrow_blocked_extension(ctx: &mut TestContext, blocked_extension_type: ExtensionType) -> Self {
        Self::builder(ctx).block_extension(blocked_extension_type).mint_extension(blocked_extension_type).build()
    }

    pub fn new_with_different_extension_blocked(
        ctx: &mut TestContext,
        blocked_extension_type: ExtensionType,
        mint_extension_type: ExtensionType,
    ) -> Self {
        Self::builder(ctx).block_extension(blocked_extension_type).mint_extension(mint_extension_type).build()
    }

    pub fn build_instruction(&self, ctx: &TestContext) -> TestInstruction {
        let instruction = AllowMintBuilder::new()
            .payer(ctx.payer.pubkey())
            .admin(self.admin.pubkey())
            .escrow(self.escrow_pda)
            .escrow_extensions(self.escrow_extensions_pda)
            .mint(self.mint.pubkey())
            .allowed_mint(self.allowed_mint_pda)
            .token_program(self.token_program)
            .bump(self.allowed_mint_bump)
            .instruction();

        TestInstruction { instruction, signers: vec![self.admin.insecure_clone()], name: "AllowMint" }
    }
}

pub struct AllowMintSetupBuilder<'a> {
    ctx: &'a mut TestContext,
    token_program: Pubkey,
    mint_extension: Option<ExtensionType>,
    blocked_extensions: Vec<ExtensionType>,
}

impl<'a> AllowMintSetupBuilder<'a> {
    fn new(ctx: &'a mut TestContext) -> Self {
        Self { ctx, token_program: TOKEN_PROGRAM_ID, mint_extension: None, blocked_extensions: Vec::new() }
    }

    pub fn token_2022(mut self) -> Self {
        self.token_program = TOKEN_2022_PROGRAM_ID;
        self
    }

    pub fn token_program(mut self, program: Pubkey) -> Self {
        self.token_program = program;
        self
    }

    pub fn mint_extension(mut self, extension: ExtensionType) -> Self {
        self.mint_extension = Some(extension);
        self.token_program = TOKEN_2022_PROGRAM_ID;
        self
    }

    pub fn block_extension(mut self, extension: ExtensionType) -> Self {
        self.blocked_extensions.push(extension);
        self
    }

    pub fn build(self) -> AllowMintSetup {
        let admin = self.ctx.create_funded_keypair();
        let escrow_seed = Keypair::new();
        let (escrow_pda, escrow_bump) = find_escrow_pda(&escrow_seed.pubkey());
        let (escrow_extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

        let create_escrow_ix = CreatesEscrowBuilder::new()
            .payer(self.ctx.payer.pubkey())
            .admin(admin.pubkey())
            .escrow_seed(escrow_seed.pubkey())
            .escrow(escrow_pda)
            .bump(escrow_bump)
            .instruction();

        self.ctx.send_transaction(create_escrow_ix, &[&admin, &escrow_seed]).unwrap();

        for blocked_ext in &self.blocked_extensions {
            let block_ext_ix = BlockTokenExtensionBuilder::new()
                .payer(self.ctx.payer.pubkey())
                .admin(admin.pubkey())
                .escrow(escrow_pda)
                .extensions(escrow_extensions_pda)
                .extensions_bump(extensions_bump)
                .blocked_extension(*blocked_ext as u16)
                .instruction();

            self.ctx.send_transaction(block_ext_ix, &[&admin]).unwrap();
        }

        let mint = Keypair::new();
        let token_program = if self.mint_extension.is_some() { TOKEN_2022_PROGRAM_ID } else { self.token_program };

        match self.mint_extension {
            Some(ext) => {
                self.ctx.create_token_2022_mint_with_extension(&mint, &self.ctx.payer.pubkey(), 6, ext);
            }
            None if token_program == TOKEN_2022_PROGRAM_ID => {
                self.ctx.create_token_2022_mint(&mint, &self.ctx.payer.pubkey(), 6);
            }
            None => {
                self.ctx.create_mint(&mint, &self.ctx.payer.pubkey(), 6);
            }
        }

        let (allowed_mint_pda, allowed_mint_bump) = find_allowed_mint_pda(&escrow_pda, &mint.pubkey());

        AllowMintSetup {
            escrow_pda,
            escrow_extensions_pda,
            admin,
            mint,
            allowed_mint_pda,
            allowed_mint_bump,
            token_program,
        }
    }
}

pub struct AllowMintFixture;

impl InstructionTestFixture for AllowMintFixture {
    const INSTRUCTION_NAME: &'static str = "AllowMint";

    fn build_valid(ctx: &mut TestContext) -> TestInstruction {
        let setup = AllowMintSetup::new(ctx);
        setup.build_instruction(ctx)
    }

    /// Account indices that must be signers:
    /// 0: payer (handled by TestContext)
    /// 1: admin
    fn required_signers() -> &'static [usize] {
        &[0, 1]
    }

    /// Account indices that must be writable:
    /// 0: payer
    /// 5: allowed_mint
    fn required_writable() -> &'static [usize] {
        &[0, 5]
    }

    fn system_program_index() -> Option<usize> {
        Some(7)
    }

    fn current_program_index() -> Option<usize> {
        Some(9)
    }

    fn data_len() -> usize {
        2
    }
}
