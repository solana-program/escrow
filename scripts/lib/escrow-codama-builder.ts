import { Codama, createFromJson } from 'codama';
import { appendAccountVersion } from './updates/';

/**
 * Builder for applying Codama IDL transformations before client generation.
 */
export class EscrowCodamaBuilder {
    private codama: Codama;

    constructor(escrowIdl: any) {
        const idlJson = typeof escrowIdl === 'string' ? escrowIdl : JSON.stringify(escrowIdl);
        this.codama = createFromJson(idlJson);
    }

    appendAccountVersion(): this {
        this.codama = appendAccountVersion(this.codama);
        return this;
    }

    build(): Codama {
        return this.codama;
    }
}

export function createEscrowCodamaBuilder(escrowIdl: any): EscrowCodamaBuilder {
    return new EscrowCodamaBuilder(escrowIdl);
}
