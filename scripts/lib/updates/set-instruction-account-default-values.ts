import {
    Codama,
    pdaNode,
    pdaValueNode,
    pdaSeedValueNode,
    publicKeyTypeNode,
    accountValueNode,
    variablePdaSeedNode,
    publicKeyValueNode,
    pdaLinkNode,
    setInstructionAccountDefaultValuesVisitor,
} from 'codama';

const ESCROW_PROGRAM_ID = 'Escrowae7RaUfNn4oEZHywMXE5zWzYCXenwrCDaEoifg';

const ATA_PROGRAM_ID = 'ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL';
const SYSTEM_PROGRAM_ID = '11111111111111111111111111111111';
const EVENT_AUTHORITY_PDA = 'G9CCHrvvmKuoM9vqcEWCxmbFiyJqXTLJBJjpSFv5v3Fm';
const TOKEN_PROGRAM_ID = 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';

function createAtaPdaValueNode(ownerAccount: string, mintAccount: string, tokenProgram: string) {
    return pdaValueNode(
        pdaNode({
            name: 'associatedTokenAccount',
            seeds: [
                variablePdaSeedNode('owner', publicKeyTypeNode()),
                variablePdaSeedNode('tokenProgram', publicKeyTypeNode()),
                variablePdaSeedNode('mint', publicKeyTypeNode()),
            ],
            programId: ATA_PROGRAM_ID,
        }),
        [
            pdaSeedValueNode('owner', accountValueNode(ownerAccount)),
            pdaSeedValueNode('tokenProgram', accountValueNode(tokenProgram)),
            pdaSeedValueNode('mint', accountValueNode(mintAccount)),
        ],
    );
}

/**
 * Sets default values for common instruction accounts (program IDs, PDAs, ATAs).
 */
export function setInstructionAccountDefaultValues(escrowCodama: Codama): Codama {
    escrowCodama.update(
        setInstructionAccountDefaultValuesVisitor([
            // Global Constants
            {
                account: 'escrowProgram',
                defaultValue: publicKeyValueNode(ESCROW_PROGRAM_ID),
            },
            {
                account: 'tokenProgram',
                defaultValue: publicKeyValueNode(TOKEN_PROGRAM_ID),
            },
            {
                account: 'associatedTokenProgram',
                defaultValue: publicKeyValueNode(ATA_PROGRAM_ID),
            },
            {
                account: 'systemProgram',
                defaultValue: publicKeyValueNode(SYSTEM_PROGRAM_ID),
            },
            {
                account: 'eventAuthority',
                defaultValue: publicKeyValueNode(EVENT_AUTHORITY_PDA),
            },

            // PDA Derivations
            {
                account: 'escrow',
                defaultValue: pdaValueNode(pdaLinkNode('escrow'), [
                    pdaSeedValueNode('escrowSeed', accountValueNode('escrowSeed')),
                ]),
            },
            {
                account: 'receipt',
                defaultValue: pdaValueNode(pdaLinkNode('receipt'), [
                    pdaSeedValueNode('escrow', accountValueNode('escrow')),
                    pdaSeedValueNode('depositor', accountValueNode('depositor')),
                    pdaSeedValueNode('mint', accountValueNode('mint')),
                    pdaSeedValueNode('receiptSeed', accountValueNode('receiptSeed')),
                ]),
            },
            {
                account: 'allowedMint',
                defaultValue: pdaValueNode(pdaLinkNode('allowedMint'), [
                    pdaSeedValueNode('escrow', accountValueNode('escrow')),
                    pdaSeedValueNode('mint', accountValueNode('mint')),
                ]),
            },
            {
                account: 'extensions',
                defaultValue: pdaValueNode(pdaLinkNode('extensions'), [
                    pdaSeedValueNode('escrow', accountValueNode('escrow')),
                ]),
            },
            {
                account: 'escrowExtensions',
                defaultValue: pdaValueNode(pdaLinkNode('extensions'), [
                    pdaSeedValueNode('escrow', accountValueNode('escrow')),
                ]),
            },

            // ATAs
            {
                account: 'vault',
                defaultValue: createAtaPdaValueNode('escrow', 'mint', 'tokenProgram'),
            },
            {
                account: 'depositorTokenAccount',
                defaultValue: createAtaPdaValueNode('depositor', 'mint', 'tokenProgram'),
            },
            {
                account: 'withdrawerTokenAccount',
                defaultValue: createAtaPdaValueNode('withdrawer', 'mint', 'tokenProgram'),
            },
        ]),
    );
    return escrowCodama;
}
