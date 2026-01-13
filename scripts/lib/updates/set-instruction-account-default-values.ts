import {
    Codama,
    pdaNode,
    pdaValueNode,
    pdaSeedValueNode,
    publicKeyTypeNode,
    accountValueNode,
    variablePdaSeedNode,
    publicKeyValueNode,
    setInstructionAccountDefaultValuesVisitor,
} from 'codama';

const ESCROW_PROGRAM_ID = 'GokvZqD2yP696rzNBNbQvcZ4VsLW7jNvFXU1kW9m7k83';

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
            {
                account: 'escrowAta',
                defaultValue: createAtaPdaValueNode('escrow', 'mint', 'tokenProgram'),
            },
            {
                account: 'userAta',
                defaultValue: createAtaPdaValueNode('user', 'mint', 'tokenProgram'),
            },
        ]),
    );
    return escrowCodama;
}
