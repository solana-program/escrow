import { Codama, bottomUpTransformerVisitor, definedTypeNode, publicKeyTypeNode, assertIsNode } from 'codama';

/**
 * Adds Address type definition to the IDL.
 *
 * Codama generates references to `crate::generated::types::Address` in account structs
 * and events, but doesn't define it. This visitor adds the type definition so Codama
 * can generate it. The type is defined as a publicKeyTypeNode (32 bytes) which matches
 * solana_address::Address.
 */
export function appendAddressType(escrowCodama: Codama): Codama {
    escrowCodama.update(
        bottomUpTransformerVisitor([
            {
                select: '[programNode]',
                transform: node => {
                    assertIsNode(node, 'programNode');

                    // Check if address type already exists
                    const hasAddressType = node.definedTypes?.some((dt: any) => dt.name === 'address');

                    if (!hasAddressType) {
                        const addressType = definedTypeNode({
                            name: 'address',
                            type: publicKeyTypeNode(),
                        });

                        return {
                            ...node,
                            definedTypes: [...(node.definedTypes || []), addressType],
                        };
                    }

                    return node;
                },
            },
        ]),
    );
    return escrowCodama;
}
