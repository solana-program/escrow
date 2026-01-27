use escrow_program_client::instructions::{
    AddTimelockBuilder, AllowMintBuilder, CreatesEscrowBuilder, DepositBuilder, WithdrawBuilder,
};
use solana_sdk::{
    instruction::AccountMeta,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_token_2022::ID as TOKEN_2022_PROGRAM_ID;
use spl_token_interface::ID as TOKEN_PROGRAM_ID;

use crate::fixtures::{SetHookFixture, DEFAULT_DEPOSIT_AMOUNT};
use crate::utils::traits::{InstructionTestFixture, TestInstruction};
use crate::utils::{find_allowed_mint_pda, find_escrow_pda, find_extensions_pda, find_receipt_pda, TestContext};

pub struct WithdrawSetup {
    pub escrow_pda: Pubkey,
    pub escrow_bump: u8,
    pub escrow_seed: Keypair,
    pub extensions_pda: Pubkey,
    pub mint: Keypair,
    pub vault: Pubkey,
    pub depositor: Keypair,
    pub depositor_token_account: Pubkey,
    pub receipt_seed: Keypair,
    pub receipt_pda: Pubkey,
    pub admin: Keypair,
    pub token_program: Pubkey,
    pub hook_program: Option<Pubkey>,
}

impl WithdrawSetup {
    pub fn builder(ctx: &mut TestContext) -> WithdrawSetupBuilder<'_> {
        WithdrawSetupBuilder::new(ctx)
    }

    pub fn new(ctx: &mut TestContext) -> Self {
        Self::builder(ctx).build()
    }

    pub fn new_token_2022(ctx: &mut TestContext) -> Self {
        Self::builder(ctx).token_2022().build()
    }

    pub fn new_with_timelock(ctx: &mut TestContext, lock_duration: u64) -> Self {
        Self::builder(ctx).timelock(lock_duration).build()
    }

    pub fn new_token_2022_with_timelock(ctx: &mut TestContext, lock_duration: u64) -> Self {
        Self::builder(ctx).token_2022().timelock(lock_duration).build()
    }

    pub fn new_with_hook(ctx: &mut TestContext, hook_program: Pubkey) -> Self {
        Self::builder(ctx).hook_program(hook_program).build()
    }

    pub fn new_token_2022_with_hook(ctx: &mut TestContext, hook_program: Pubkey) -> Self {
        Self::builder(ctx).token_2022().hook_program(hook_program).build()
    }

    pub fn set_hook(&mut self, ctx: &mut TestContext, hook_program: Pubkey) {
        let test_ix =
            SetHookFixture::build_with_escrow(ctx, self.escrow_pda, self.admin.insecure_clone(), hook_program);
        test_ix.send_expect_success(ctx);
        self.hook_program = Some(hook_program);
    }

    pub fn build_instruction(&self, ctx: &TestContext) -> TestInstruction {
        self.build_instruction_with_rent_recipient(ctx, ctx.payer.pubkey())
    }

    pub fn build_instruction_with_rent_recipient(&self, ctx: &TestContext, rent_recipient: Pubkey) -> TestInstruction {
        let mut builder = WithdrawBuilder::new();
        builder
            .payer(ctx.payer.pubkey())
            .withdrawer(self.depositor.pubkey())
            .escrow(self.escrow_pda)
            .extensions(self.extensions_pda)
            .receipt(self.receipt_pda)
            .vault(self.vault)
            .withdrawer_token_account(self.depositor_token_account)
            .mint(self.mint.pubkey())
            .token_program(self.token_program)
            .rent_recipient(rent_recipient);

        if let Some(hook_program) = self.hook_program {
            builder.add_remaining_account(AccountMeta::new_readonly(hook_program, false));
        }

        let instruction = builder.instruction();

        TestInstruction { instruction, signers: vec![self.depositor.insecure_clone()], name: "Withdraw" }
    }
}

pub struct WithdrawSetupBuilder<'a> {
    ctx: &'a mut TestContext,
    token_program: Pubkey,
    timelock: Option<u64>,
    hook_program: Option<Pubkey>,
}

impl<'a> WithdrawSetupBuilder<'a> {
    fn new(ctx: &'a mut TestContext) -> Self {
        Self { ctx, token_program: TOKEN_PROGRAM_ID, timelock: None, hook_program: None }
    }

    pub fn token_2022(mut self) -> Self {
        self.token_program = TOKEN_2022_PROGRAM_ID;
        self
    }

    pub fn token_program(mut self, program: Pubkey) -> Self {
        self.token_program = program;
        self
    }

    pub fn timelock(mut self, lock_duration: u64) -> Self {
        self.timelock = Some(lock_duration);
        self
    }

    pub fn hook_program(mut self, program: Pubkey) -> Self {
        self.hook_program = Some(program);
        self
    }

    pub fn build(self) -> WithdrawSetup {
        let admin = self.ctx.create_funded_keypair();
        let escrow_seed = Keypair::new();
        let (escrow_pda, escrow_bump) = find_escrow_pda(&escrow_seed.pubkey());

        let create_escrow_ix = CreatesEscrowBuilder::new()
            .payer(self.ctx.payer.pubkey())
            .admin(admin.pubkey())
            .escrow_seed(escrow_seed.pubkey())
            .escrow(escrow_pda)
            .bump(escrow_bump)
            .instruction();

        self.ctx.send_transaction(create_escrow_ix, &[&admin, &escrow_seed]).unwrap();

        let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

        if let Some(lock_duration) = self.timelock {
            let add_timelock_ix = AddTimelockBuilder::new()
                .payer(self.ctx.payer.pubkey())
                .admin(admin.pubkey())
                .escrow(escrow_pda)
                .extensions(extensions_pda)
                .extensions_bump(extensions_bump)
                .lock_duration(lock_duration)
                .instruction();

            self.ctx.send_transaction(add_timelock_ix, &[&admin]).unwrap();
        }

        if let Some(hook_id) = self.hook_program {
            let test_ix = SetHookFixture::build_with_escrow(self.ctx, escrow_pda, admin.insecure_clone(), hook_id);
            test_ix.send_expect_success(self.ctx);
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
            .vault(vault)
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

        let mut deposit_builder = DepositBuilder::new();
        deposit_builder
            .payer(self.ctx.payer.pubkey())
            .depositor(depositor.pubkey())
            .escrow(escrow_pda)
            .allowed_mint(allowed_mint_pda)
            .receipt_seed(receipt_seed.pubkey())
            .receipt(receipt_pda)
            .vault(vault)
            .depositor_token_account(depositor_token_account)
            .mint(mint.pubkey())
            .token_program(token_program)
            .extensions(extensions_pda)
            .bump(bump)
            .amount(DEFAULT_DEPOSIT_AMOUNT);

        if let Some(hook_id) = self.hook_program {
            deposit_builder.add_remaining_account(AccountMeta::new_readonly(hook_id, false));
        }

        let deposit_ix = deposit_builder.instruction();
        self.ctx.send_transaction(deposit_ix, &[&depositor, &receipt_seed]).unwrap();

        WithdrawSetup {
            escrow_pda,
            escrow_bump,
            escrow_seed,
            extensions_pda,
            mint,
            vault,
            depositor,
            depositor_token_account,
            receipt_seed,
            receipt_pda,
            admin,
            token_program,
            hook_program: self.hook_program,
        }
    }
}

pub struct WithdrawFixture;

impl InstructionTestFixture for WithdrawFixture {
    const INSTRUCTION_NAME: &'static str = "Withdraw";

    fn build_valid(ctx: &mut TestContext) -> TestInstruction {
        let setup = WithdrawSetup::new(ctx);
        setup.build_instruction(ctx)
    }

    /// Account indices that must be signers:
    /// 0: payer (handled by TestContext)
    /// 2: withdrawer
    fn required_signers() -> &'static [usize] {
        &[0, 2]
    }

    /// Account indices that must be writable:
    /// 1: rent_recipient
    /// 5: receipt
    /// 6: vault
    /// 7: withdrawer_token_account
    fn required_writable() -> &'static [usize] {
        &[1, 5, 6, 7]
    }

    fn system_program_index() -> Option<usize> {
        Some(10)
    }

    fn current_program_index() -> Option<usize> {
        Some(12)
    }

    fn data_len() -> usize {
        1
    }
}
