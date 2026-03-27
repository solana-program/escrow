use escrow_program_client::instructions::SetImmutableBuilder;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};

use crate::{
    fixtures::CreateEscrowFixture,
    utils::{find_escrow_pda, TestContext},
};

use crate::utils::traits::{InstructionTestFixture, TestInstruction};

pub struct SetImmutableFixture;

impl SetImmutableFixture {
    pub fn build_with_escrow(_ctx: &mut TestContext, escrow_pda: Pubkey, admin: Keypair) -> TestInstruction {
        let instruction = SetImmutableBuilder::new().admin(admin.pubkey()).escrow(escrow_pda).instruction();

        TestInstruction { instruction, signers: vec![admin], name: Self::INSTRUCTION_NAME }
    }
}

impl InstructionTestFixture for SetImmutableFixture {
    const INSTRUCTION_NAME: &'static str = "SetImmutable";

    fn build_valid(ctx: &mut TestContext) -> TestInstruction {
        let escrow_ix = CreateEscrowFixture::build_valid(ctx);
        let admin = escrow_ix.signers[0].insecure_clone();
        let escrow_seed = escrow_ix.signers[1].pubkey();
        escrow_ix.send_expect_success(ctx);

        let (escrow_pda, _) = find_escrow_pda(&escrow_seed);

        let instruction = SetImmutableBuilder::new().admin(admin.pubkey()).escrow(escrow_pda).instruction();

        TestInstruction { instruction, signers: vec![admin], name: Self::INSTRUCTION_NAME }
    }

    /// Account indices that must be signers:
    /// 0: admin
    fn required_signers() -> &'static [usize] {
        &[0]
    }

    /// Account indices that must be writable:
    /// 1: escrow
    fn required_writable() -> &'static [usize] {
        &[1]
    }

    fn system_program_index() -> Option<usize> {
        None
    }

    fn current_program_index() -> Option<usize> {
        Some(3)
    }

    fn data_len() -> usize {
        1 // Just the discriminator
    }
}
