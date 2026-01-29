import { Codama, updateInstructionsVisitor, accountBumpValueNode } from 'codama';

/**
 * Sets default bump values for createEscrow and allowMint instructions.
 */
export function updateInstructionBumps(escrowCodama: Codama): Codama {
    escrowCodama.update(
        updateInstructionsVisitor({
            createsEscrow: {
                arguments: {
                    bump: {
                        defaultValue: accountBumpValueNode('escrow'),
                    },
                },
            },
            allowMint: {
                arguments: {
                    bump: {
                        defaultValue: accountBumpValueNode('allowedMint'),
                    },
                },
            },
            addTimelock: {
                arguments: {
                    extensionsBump: {
                        defaultValue: accountBumpValueNode('extensions'),
                    },
                },
            },
            setHook: {
                arguments: {
                    extensionsBump: {
                        defaultValue: accountBumpValueNode('extensions'),
                    },
                },
            },
            deposit: {
                arguments: {
                    bump: {
                        defaultValue: accountBumpValueNode('receipt'),
                    },
                },
            },
        }),
    );
    return escrowCodama;
}
