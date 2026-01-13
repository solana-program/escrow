import { Codama, updateInstructionsVisitor, accountBumpValueNode } from 'codama';

/**
 * Sets default bump values for createEscrow and allowMint instructions.
 */
export function updateInstructionBumps(escrowCodama: Codama): Codama {
    escrowCodama.update(
        updateInstructionsVisitor({
            createEscrow: {
                arguments: {
                    bump: {
                        defaultValue: accountBumpValueNode('escrow'),
                    },
                },
            },
        }),
    );
    return escrowCodama;
}
