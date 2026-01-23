use escrow_program_client::instructions::SetHookBuilder;
use solana_address::Address;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::{
    fixtures::CreateEscrowFixture,
    utils::{find_escrow_pda, find_extensions_pda, TestContext},
};

use crate::utils::traits::{InstructionTestFixture, TestInstruction};

pub struct SetHookFixture;

impl SetHookFixture {
    pub fn build_with_escrow(
        ctx: &mut TestContext,
        escrow_pda: Pubkey,
        admin: Keypair,
        hook_program: Pubkey,
    ) -> TestInstruction {
        let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

        let instruction = SetHookBuilder::new()
            .payer(ctx.payer.pubkey())
            .admin(admin.pubkey())
            .escrow(escrow_pda)
            .extensions(extensions_pda)
            .extensions_bump(extensions_bump)
            .hook_program(Address::from(hook_program.to_bytes()))
            .instruction();

        TestInstruction { instruction, signers: vec![admin], name: Self::INSTRUCTION_NAME }
    }
}

impl InstructionTestFixture for SetHookFixture {
    const INSTRUCTION_NAME: &'static str = "SetHook";

    fn build_valid(ctx: &mut TestContext) -> TestInstruction {
        let escrow_ix = CreateEscrowFixture::build_valid(ctx);
        let admin = escrow_ix.signers[0].insecure_clone();
        let escrow_seed = escrow_ix.signers[1].pubkey();
        escrow_ix.send_expect_success(ctx);

        let (escrow_pda, _) = find_escrow_pda(&escrow_seed);
        let (extensions_pda, extensions_bump) = find_extensions_pda(&escrow_pda);

        let hook_program = Pubkey::new_unique();

        let instruction = SetHookBuilder::new()
            .payer(ctx.payer.pubkey())
            .admin(admin.pubkey())
            .escrow(escrow_pda)
            .extensions(extensions_pda)
            .extensions_bump(extensions_bump)
            .hook_program(Address::from(hook_program.to_bytes()))
            .instruction();

        TestInstruction { instruction, signers: vec![admin], name: Self::INSTRUCTION_NAME }
    }

    /// Account indices that must be signers:
    /// 1: admin (payer at 0 is handled separately by TestContext)
    fn required_signers() -> &'static [usize] {
        &[1]
    }

    /// Account indices that must be writable:
    /// 3: extensions (payer at 0 is handled separately by TestContext)
    fn required_writable() -> &'static [usize] {
        &[3]
    }

    fn system_program_index() -> Option<usize> {
        Some(4)
    }

    fn current_program_index() -> Option<usize> {
        Some(6)
    }

    fn data_len() -> usize {
        34
    }
}
