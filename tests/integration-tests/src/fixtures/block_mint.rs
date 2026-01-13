use escrow_program_client::instructions::BlockMintBuilder;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::fixtures::allow_mint::AllowMintSetup;
use crate::utils::traits::{InstructionTestFixture, TestInstruction};
use crate::utils::TestContext;

pub struct BlockMintSetup {
    pub escrow_pda: Pubkey,
    pub admin: Keypair,
    pub mint: Keypair,
    pub allowed_mint_pda: Pubkey,
    pub token_program: Pubkey,
}

impl BlockMintSetup {
    pub fn new(ctx: &mut TestContext) -> Self {
        let allow_mint_setup = AllowMintSetup::new(ctx);
        Self::from_allow_mint_setup(ctx, allow_mint_setup)
    }

    pub fn new_token_2022(ctx: &mut TestContext) -> Self {
        let allow_mint_setup = AllowMintSetup::new_token_2022(ctx);
        Self::from_allow_mint_setup(ctx, allow_mint_setup)
    }

    fn from_allow_mint_setup(ctx: &mut TestContext, allow_mint_setup: AllowMintSetup) -> Self {
        let allow_ix = allow_mint_setup.build_instruction(ctx);
        allow_ix.send_expect_success(ctx);

        Self {
            escrow_pda: allow_mint_setup.escrow_pda,
            admin: allow_mint_setup.admin,
            mint: allow_mint_setup.mint,
            allowed_mint_pda: allow_mint_setup.allowed_mint_pda,
            token_program: allow_mint_setup.token_program,
        }
    }

    pub fn build_instruction(&self, ctx: &TestContext) -> TestInstruction {
        let instruction = BlockMintBuilder::new()
            .admin(self.admin.pubkey())
            .payer(ctx.payer.pubkey())
            .escrow(self.escrow_pda)
            .mint(self.mint.pubkey())
            .allowed_mint(self.allowed_mint_pda)
            .token_program(self.token_program)
            .instruction();

        TestInstruction { instruction, signers: vec![self.admin.insecure_clone()], name: "BlockMint" }
    }
}

pub struct BlockMintFixture;

impl InstructionTestFixture for BlockMintFixture {
    const INSTRUCTION_NAME: &'static str = "BlockMint";

    fn build_valid(ctx: &mut TestContext) -> TestInstruction {
        let setup = BlockMintSetup::new(ctx);
        setup.build_instruction(ctx)
    }

    /// Account indices that must be signers:
    /// 0: admin
    fn required_signers() -> &'static [usize] {
        &[0]
    }

    /// Account indices that must be writable:
    /// 1: payer (receives rent refund)
    /// 4: allowed_mint (being closed)
    fn required_writable() -> &'static [usize] {
        &[1, 4]
    }

    fn system_program_index() -> Option<usize> {
        None
    }

    fn current_program_index() -> Option<usize> {
        Some(7)
    }

    fn data_len() -> usize {
        1
    }
}
