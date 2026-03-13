'use client';

import { useState } from 'react';
import type { Address } from '@solana/kit';
import { getWithdrawInstructionAsync } from '@solana/escrow-program-client';
import { useSendTx } from '@/hooks/useSendTx';
import { useSavedValues } from '@/contexts/SavedValuesContext';
import { useWallet } from '@/contexts/WalletContext';
import { TxResult } from '@/components/TxResult';
import { firstValidationError, validateAddress, validateOptionalAddress } from '@/lib/validation';
import { FormField, SendButton } from './shared';

export function Withdraw() {
    const { account, createSigner } = useWallet();
    const { send, sending, signature, error, reset } = useSendTx();
    const { defaultEscrow, defaultMint, defaultReceipt, rememberEscrow, rememberMint, rememberReceipt } =
        useSavedValues();
    const [escrow, setEscrow] = useState('');
    const [mint, setMint] = useState('');
    const [receipt, setReceipt] = useState('');
    const [rentRecipient, setRentRecipient] = useState('');
    const [formError, setFormError] = useState<string | null>(null);

    const handleSubmit = async (e: React.FormEvent) => {
        e.preventDefault();
        reset();
        setFormError(null);
        const signer = createSigner();
        if (!signer) return;

        const validationError = firstValidationError(
            validateAddress(escrow, 'Escrow address'),
            validateAddress(mint, 'Mint address'),
            validateAddress(receipt, 'Receipt address'),
            validateOptionalAddress(rentRecipient, 'Rent recipient'),
        );
        if (validationError) {
            setFormError(validationError);
            return;
        }

        const ix = await getWithdrawInstructionAsync({
            withdrawer: signer,
            escrow: escrow as Address,
            mint: mint as Address,
            receipt: receipt as Address,
            rentRecipient: (rentRecipient || signer.address) as Address,
        });
        const txSignature = await send([ix], {
            action: 'Withdraw',
            values: { escrow, mint, receipt, rentRecipient: rentRecipient || account?.address || '' },
        });
        if (txSignature) {
            rememberEscrow(escrow);
            rememberMint(mint);
            rememberReceipt(receipt);
        }
    };

    return (
        <form onSubmit={handleSubmit} style={{ display: 'flex', flexDirection: 'column', gap: 16 }}>
            <FormField
                label="Escrow Address"
                value={escrow}
                onChange={setEscrow}
                autoFillValue={defaultEscrow}
                onAutoFill={setEscrow}
                placeholder="Escrow PDA address"
                required
            />
            <FormField
                label="Mint Address"
                value={mint}
                onChange={setMint}
                autoFillValue={defaultMint}
                onAutoFill={setMint}
                placeholder="SPL token mint address"
                required
            />
            <FormField
                label="Receipt Address"
                value={receipt}
                onChange={setReceipt}
                autoFillValue={defaultReceipt}
                onAutoFill={setReceipt}
                placeholder="Receipt PDA address from deposit"
                hint="The receipt PDA created during Deposit"
                required
            />
            <FormField
                label="Rent Recipient"
                value={rentRecipient}
                onChange={setRentRecipient}
                placeholder={account?.address ?? 'Defaults to connected wallet'}
                hint="Address that receives rent from the closed receipt account"
            />
            <SendButton sending={sending} />
            <TxResult signature={signature} error={formError ?? error} />
        </form>
    );
}
