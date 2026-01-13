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
 * Adds PDA derivation functions for escrow, and eventAuthority.
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
                    name: 'eventAuthority',
                    seeds: [constantPdaSeedNode(stringTypeNode('utf8'), stringValueNode('event_authority'))],
                },
            ],
        }),
    );
    return escrowCodama;
}
