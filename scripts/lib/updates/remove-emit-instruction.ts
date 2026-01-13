import { Codama, updateInstructionsVisitor } from 'codama';

/**
 * Removes the internal emitEvent instruction from client APIs.
 */
export function removeEmitInstruction(escrowCodama: Codama): Codama {
    escrowCodama.update(
        updateInstructionsVisitor({
            emitEvent: {
                delete: true,
            },
        }),
    );
    return escrowCodama;
}
