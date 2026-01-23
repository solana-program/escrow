use escrow_program_client::instructions::{AllowMintBuilder, CreatesEscrowBuilder, DepositBuilder, SetHookBuilder};
use solana_address::Address;
use solana_sdk::{
    instruction::AccountMeta,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;
use spl_token_interface::ID as TOKEN_PROGRAM_ID;

use crate::utils::traits::{InstructionTestFixture, TestInstruction};
use crate::utils::{find_allowed_mint_pda, find_escrow_pda, find_extensions_pda, find_receipt_pda, TestContext};

pub const DEFAULT_DEPOSIT_AMOUNT: u64 = 1_000_000;

pub struct DepositSetup {
    pub escrow_pda: Pubkey,
    pub extensions_pda: Pubkey,
    pub admin: Keypair,
    pub mint: Keypair,
    pub allowed_mint_pda: Pubkey,
    pub vault: Pubkey,
    pub depositor: Keypair,
    pub depositor_token_account: Pubkey,
    pub receipt_seed: Keypair,
    pub receipt_pda: Pubkey,
    pub bump: u8,
    pub token_program: Pubkey,
    pub hook_program: Option<Pubkey>,
}

impl DepositSetup {
    pub fn builder(ctx: &mut TestContext) -> DepositSetupBuilder<'_> {
        DepositSetupBuilder::new(ctx)
    }

    pub fn new(ctx: &mut TestContext) -> Self {
        Self::builder(ctx).build()
    }

    pub fn new_token_2022(ctx: &mut TestContext) -> Self {
        Self::builder(ctx).token_2022().build()
    }

    pub fn new_with_hook(ctx: &mut TestContext, hook_program: Pubkey) -> Self {
        Self::builder(ctx).hook_program(hook_program).build()
    }

    pub fn new_token_2022_with_hook(ctx: &mut TestContext, hook_program: Pubkey) -> Self {
        Self::builder(ctx).token_2022().hook_program(hook_program).build()
    }

    pub fn build_instruction(&self, ctx: &TestContext) -> TestInstruction {
        let mut builder = DepositBuilder::new();
        builder
            .payer(ctx.payer.pubkey())
            .depositor(self.depositor.pubkey())
            .escrow(self.escrow_pda)
            .allowed_mint(self.allowed_mint_pda)
            .receipt_seed(self.receipt_seed.pubkey())
            .receipt(self.receipt_pda)
            .vault(self.vault)
            .depositor_token_account(self.depositor_token_account)
            .mint(self.mint.pubkey())
            .token_program(self.token_program)
            .extensions(self.extensions_pda)
            .bump(self.bump)
            .amount(DEFAULT_DEPOSIT_AMOUNT);

        if let Some(hook_program) = self.hook_program {
            builder.add_remaining_account(AccountMeta::new_readonly(hook_program, false));
        }

        let instruction = builder.instruction();

        TestInstruction {
            instruction,
            signers: vec![self.depositor.insecure_clone(), self.receipt_seed.insecure_clone()],
            name: "Deposit",
        }
    }
}

pub struct DepositSetupBuilder<'a> {
    ctx: &'a mut TestContext,
    token_program: Pubkey,
    hook_program: Option<Pubkey>,
}

impl<'a> DepositSetupBuilder<'a> {
    fn new(ctx: &'a mut TestContext) -> Self {
        Self { ctx, token_program: TOKEN_PROGRAM_ID, hook_program: None }
    }

    pub fn token_2022(mut self) -> Self {
        self.token_program = TOKEN_2022_PROGRAM_ID;
        self
    }

    pub fn token_program(mut self, program: Pubkey) -> Self {
        self.token_program = program;
        self
    }

    pub fn hook_program(mut self, program: Pubkey) -> Self {
        self.hook_program = Some(program);
        self
    }

    pub fn build(self) -> DepositSetup {
        let admin = self.ctx.create_funded_keypair();
        let escrow_seed = Keypair::new();
        let (escrow_pda, escrow_bump) = find_escrow_pda(&escrow_seed.pubkey());
        let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

        let create_escrow_ix = CreatesEscrowBuilder::new()
            .payer(self.ctx.payer.pubkey())
            .admin(admin.pubkey())
            .escrow_seed(escrow_seed.pubkey())
            .escrow(escrow_pda)
            .bump(escrow_bump)
            .instruction();

        self.ctx.send_transaction(create_escrow_ix, &[&admin, &escrow_seed]).unwrap();

        if let Some(hook_id) = self.hook_program {
            let set_hook_ix = SetHookBuilder::new()
                .payer(self.ctx.payer.pubkey())
                .admin(admin.pubkey())
                .escrow(escrow_pda)
                .extensions(extensions_pda)
                .extensions_bump(extensions_bump)
                .hook_program(Address::from(hook_id.to_bytes()))
                .instruction();

            self.ctx.send_transaction(set_hook_ix, &[&admin]).unwrap();
        }

        let mint = Keypair::new();
        let token_program = self.token_program;
        let (vault, depositor_token_account);

        if token_program == TOKEN_2022_PROGRAM_ID {
            self.ctx.create_token_2022_mint(&mint, &self.ctx.payer.pubkey(), 6);
            vault = self.ctx.create_token_2022_account(&escrow_pda, &mint.pubkey());
        } else {
            self.ctx.create_mint(&mint, &self.ctx.payer.pubkey(), 6);
            vault = self.ctx.create_token_account(&escrow_pda, &mint.pubkey());
        }

        let (allowed_mint_pda, allowed_mint_bump) = find_allowed_mint_pda(&escrow_pda, &mint.pubkey());

        let allow_mint_ix = AllowMintBuilder::new()
            .payer(self.ctx.payer.pubkey())
            .admin(admin.pubkey())
            .escrow(escrow_pda)
            .escrow_extensions(extensions_pda)
            .mint(mint.pubkey())
            .allowed_mint(allowed_mint_pda)
            .token_program(token_program)
            .bump(allowed_mint_bump)
            .instruction();

        self.ctx.send_transaction(allow_mint_ix, &[&admin]).unwrap();

        let depositor = self.ctx.create_funded_keypair();
        if token_program == TOKEN_2022_PROGRAM_ID {
            depositor_token_account = self.ctx.create_token_2022_account_with_balance(
                &depositor.pubkey(),
                &mint.pubkey(),
                DEFAULT_DEPOSIT_AMOUNT * 10,
            );
        } else {
            depositor_token_account = self.ctx.create_token_account_with_balance(
                &depositor.pubkey(),
                &mint.pubkey(),
                DEFAULT_DEPOSIT_AMOUNT * 10,
            );
        }

        let receipt_seed = Keypair::new();
        let (receipt_pda, bump) =
            find_receipt_pda(&escrow_pda, &depositor.pubkey(), &mint.pubkey(), &receipt_seed.pubkey());

        DepositSetup {
            escrow_pda,
            extensions_pda,
            admin,
            mint,
            allowed_mint_pda,
            vault,
            depositor,
            depositor_token_account,
            receipt_seed,
            receipt_pda,
            bump,
            token_program,
            hook_program: self.hook_program,
        }
    }
}

pub struct DepositFixture;

impl InstructionTestFixture for DepositFixture {
    const INSTRUCTION_NAME: &'static str = "Deposit";

    fn build_valid(ctx: &mut TestContext) -> TestInstruction {
        let setup = DepositSetup::new(ctx);
        setup.build_instruction(ctx)
    }

    /// Account indices that must be signers:
    /// 0: payer (handled by TestContext)
    /// 1: depositor
    /// 4: receipt_seed
    fn required_signers() -> &'static [usize] {
        &[0, 1, 4]
    }

    /// Account indices that must be writable:
    /// 0: payer (handled by TestContext)
    /// 5: receipt
    /// 6: vault
    /// 7: depositor_token_account
    fn required_writable() -> &'static [usize] {
        &[0, 5, 6, 7]
    }

    fn system_program_index() -> Option<usize> {
        Some(10)
    }

    fn current_program_index() -> Option<usize> {
        Some(12)
    }

    fn data_len() -> usize {
        10
    }
}
