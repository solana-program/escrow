import { Codama, createFromJson } from 'codama';
import {
    appendAccountDiscriminator,
    appendAccountVersion,
    appendAddressType,
    appendPdaDerivers,
    setInstructionAccountDefaultValues,
    updateInstructionBumps,
} from './updates/';
import { removeEmitInstruction } from './updates/remove-emit-instruction';

/**
 * Builder for applying Codama IDL transformations before client generation.
 */
export class EscrowCodamaBuilder {
    private codama: Codama;

    constructor(escrowIdl: any) {
        const idlJson = typeof escrowIdl === 'string' ? escrowIdl : JSON.stringify(escrowIdl);
        this.codama = createFromJson(idlJson);
    }

    appendAccountDiscriminator(): this {
        this.codama = appendAccountDiscriminator(this.codama);
        return this;
    }

    appendAccountVersion(): this {
        this.codama = appendAccountVersion(this.codama);
        return this;
    }

    appendAddressType(): this {
        this.codama = appendAddressType(this.codama);
        return this;
    }

    appendPdaDerivers(): this {
        this.codama = appendPdaDerivers(this.codama);
        return this;
    }

    setInstructionAccountDefaultValues(): this {
        this.codama = setInstructionAccountDefaultValues(this.codama);
        return this;
    }

    updateInstructionBumps(): this {
        this.codama = updateInstructionBumps(this.codama);
        return this;
    }

    removeEmitInstruction(): this {
        this.codama = removeEmitInstruction(this.codama);
        return this;
    }

    build(): Codama {
        return this.codama;
    }
}

export function createEscrowCodamaBuilder(escrowIdl: any): EscrowCodamaBuilder {
    return new EscrowCodamaBuilder(escrowIdl);
}
