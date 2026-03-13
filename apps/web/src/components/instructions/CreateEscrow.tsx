'use client';

import { useState } from 'react';
import { generateKeyPairSigner, type Address } from '@solana/kit';
import { findEscrowPda, getCreatesEscrowInstructionAsync } from '@solana/escrow-program-client';
import { useSendTx } from '@/hooks/useSendTx';
import { useSavedValues } from '@/contexts/SavedValuesContext';
import { useWallet } from '@/contexts/WalletContext';
import { TxResult } from '@/components/TxResult';
import { FormField, SendButton } from './shared';

export function CreateEscrow() {
    const { account, createSigner } = useWallet();
    const { send, sending, signature, error, reset } = useSendTx();
    const { rememberEscrow } = useSavedValues();
    const [generatedSeed, setGeneratedSeed] = useState('');
    const [generatedEscrow, setGeneratedEscrow] = useState('');

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        reset();
        const signer = createSigner();
        if (!signer) return;

        const escrowSeed = await generateKeyPairSigner();
        setGeneratedSeed(escrowSeed.address);
        const [escrow] = await findEscrowPda({ escrowSeed: escrowSeed.address as Address });
        setGeneratedEscrow(escrow);

        const ix = await getCreatesEscrowInstructionAsync({
            admin: signer,
            escrowSeed,
            payer: signer,
        });
        const txSignature = await send([ix], {
            action: 'Create Escrow',
            values: { escrow },
        });
        if (txSignature) {
            rememberEscrow(escrow);
        }
    };

    return (
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
            <FormField
                label="Admin Address"
                value={account?.address ?? ''}
                onChange={() => {}}
                placeholder="Connect wallet first"
                hint="The admin authority for the escrow (connected wallet)"
                readOnly
            />
            {generatedSeed && (
                <FormField
                    label="Generated Escrow Seed"
                    value={generatedSeed}
                    onChange={() => {}}
                    readOnly
                    hint="Auto-generated keypair used as escrow PDA seed"
                />
            )}
            {generatedEscrow && (
                <FormField
                    label="Generated Escrow PDA"
                    value={generatedEscrow}
                    onChange={() => {}}
                    readOnly
                    hint="Saved as the default escrow when creation succeeds"
                />
            )}
            <SendButton sending={sending} />
            <TxResult signature={signature} error={error} />
        </form>
    );
}
