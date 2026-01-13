use escrow_program_client::instructions::CreatesEscrowBuilder;
use solana_sdk::signature::{Keypair, Signer};

use crate::utils::{find_escrow_pda, TestContext};

use crate::utils::traits::{InstructionTestFixture, TestInstruction};

pub struct CreateEscrowFixture;

impl InstructionTestFixture for CreateEscrowFixture {
    const INSTRUCTION_NAME: &'static str = "CreateEscrow";

    fn build_valid(ctx: &mut TestContext) -> TestInstruction {
        let admin = ctx.create_funded_keypair();
        let escrow_seed = Keypair::new();
        let (escrow_pda, bump) = find_escrow_pda(&escrow_seed.pubkey());

        let instruction = CreatesEscrowBuilder::new()
            .payer(ctx.payer.pubkey())
            .admin(admin.pubkey())
            .escrow_seed(escrow_seed.pubkey())
            .escrow(escrow_pda)
            .bump(bump)
            .instruction();

        TestInstruction { instruction, signers: vec![admin, escrow_seed], name: Self::INSTRUCTION_NAME }
    }

    /// Account indices that must be signers:
    /// 0: payer (handled by TestContext)
    /// 1: admin
    /// 2: escrow_seed
    fn required_signers() -> &'static [usize] {
        &[0, 1, 2]
    }

    /// Account indices that must be writable:
    /// 0: payer
    /// 3: escrow
    fn required_writable() -> &'static [usize] {
        &[0, 3]
    }

    fn system_program_index() -> Option<usize> {
        Some(4)
    }

    fn current_program_index() -> Option<usize> {
        Some(6)
    }

    fn data_len() -> usize {
        2
    }
}
