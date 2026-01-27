import {
    Codama,
    constantPdaSeedNode,
    stringTypeNode,
    stringValueNode,
    variablePdaSeedNode,
    publicKeyTypeNode,
    addPdasVisitor,
} from 'codama';

/**
 * Adds PDA derivation functions for escrow accounts.
 */
export function appendPdaDerivers(escrowCodama: Codama): Codama {
    escrowCodama.update(
        addPdasVisitor({
            escrowProgram: [
                {
                    name: 'escrow',
                    seeds: [
                        constantPdaSeedNode(stringTypeNode('utf8'), stringValueNode('escrow')),
                        variablePdaSeedNode('escrowSeed', publicKeyTypeNode()),
                    ],
                },
                {
                    name: 'receipt',
                    seeds: [
                        constantPdaSeedNode(stringTypeNode('utf8'), stringValueNode('receipt')),
                        variablePdaSeedNode('escrow', publicKeyTypeNode()),
                        variablePdaSeedNode('depositor', publicKeyTypeNode()),
                        variablePdaSeedNode('mint', publicKeyTypeNode()),
                        variablePdaSeedNode('receiptSeed', publicKeyTypeNode()),
                    ],
                },
                {
                    name: 'allowedMint',
                    seeds: [
                        constantPdaSeedNode(stringTypeNode('utf8'), stringValueNode('allowed_mint')),
                        variablePdaSeedNode('escrow', publicKeyTypeNode()),
                        variablePdaSeedNode('mint', publicKeyTypeNode()),
                    ],
                },
                {
                    name: 'extensions',
                    seeds: [
                        constantPdaSeedNode(stringTypeNode('utf8'), stringValueNode('extensions')),
                        variablePdaSeedNode('escrow', publicKeyTypeNode()),
                    ],
                },
                {
                    name: 'eventAuthority',
                    seeds: [constantPdaSeedNode(stringTypeNode('utf8'), stringValueNode('event_authority'))],
                },
            ],
        }),
    );
    return escrowCodama;
}
