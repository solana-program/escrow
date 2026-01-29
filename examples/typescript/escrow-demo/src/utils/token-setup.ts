import { type Address, type Instruction, Rpc, SolanaRpcApi, type TransactionSigner } from '@solana/kit';
import { getCreateAccountInstruction } from '@solana-program/system';
import { getInitializeMintInstruction, getMintSize, TOKEN_PROGRAM_ADDRESS } from '@solana-program/token';

/**
 * Create instructions to initialize a new SPL Token mint
 */
export async function createMintInstructions(
    payer: TransactionSigner,
    mintKeypair: TransactionSigner,
    mintAuthority: Address,
    decimals: number,
    rpc: Rpc<SolanaRpcApi>,
): Promise<Instruction[]> {
    const mintSize = getMintSize();
    const mintRent = await rpc.getMinimumBalanceForRentExemption(BigInt(mintSize)).send();

    const createAccountIx = getCreateAccountInstruction({
        lamports: mintRent,
        newAccount: mintKeypair,
        payer,
        programAddress: TOKEN_PROGRAM_ADDRESS,
        space: mintSize,
    });

    const initMintIx = getInitializeMintInstruction({
        decimals,
        freezeAuthority: null,
        mint: mintKeypair.address,
        mintAuthority,
    });

    return [createAccountIx, initMintIx];
}
