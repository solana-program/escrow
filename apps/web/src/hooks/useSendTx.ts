'use client';

import { useCallback, useState } from 'react';
import {
    appendTransactionMessageInstructions,
    assertIsTransactionWithBlockhashLifetime,
    createTransactionMessage,
    getSignatureFromTransaction,
    pipe,
    sendAndConfirmTransactionFactory,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
    type Instruction,
} from '@solana/kit';
import { useRpc, useRpcSubscriptions } from './useRpc';
import type { RecentTransactionValues } from '@/contexts/RecentTransactionsContext';
import { useRecentTransactions } from '@/contexts/RecentTransactionsContext';
import { useWallet } from '@/contexts/WalletContext';
import { formatTransactionError } from '@/lib/transactionErrors';

export interface SendTxState {
    sending: boolean;
    signature: string | null;
    error: string | null;
}

export interface SendTxOptions {
    action?: string;
    values?: RecentTransactionValues;
}

export function useSendTx() {
    const rpc = useRpc();
    const rpcSubscriptions = useRpcSubscriptions();
    const { createSigner } = useWallet();
    const { addRecentTransaction } = useRecentTransactions();

    const [state, setState] = useState<SendTxState>({
        sending: false,
        signature: null,
        error: null,
    });

    const send = useCallback(
        async (instructions: readonly Instruction[], options?: SendTxOptions) => {
            const signer = createSigner();
            if (!signer) {
                setState(s => ({ ...s, error: 'Wallet not connected' }));
                return null;
            }

            setState({ sending: true, signature: null, error: null });

            let txSignature: string | null = null;
            try {
                const sendAndConfirm = sendAndConfirmTransactionFactory({ rpc, rpcSubscriptions });
                const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();
                const txId = `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

                const txMessage = pipe(
                    createTransactionMessage({ version: 0 }),
                    tx => setTransactionMessageFeePayerSigner(signer, tx),
                    tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
                    tx => appendTransactionMessageInstructions(instructions, tx),
                );

                const signedTx = await signTransactionMessageWithSigners(txMessage);
                txSignature = getSignatureFromTransaction(signedTx);
                assertIsTransactionWithBlockhashLifetime(signedTx);

                await sendAndConfirm(signedTx, {
                    commitment: 'confirmed',
                    skipPreflight: true,
                });
                addRecentTransaction({
                    id: txId,
                    signature: txSignature,
                    action: options?.action ?? 'Transaction',
                    timestamp: Date.now(),
                    status: 'success',
                    values: options?.values,
                });
                setState({ sending: false, signature: txSignature, error: null });
                return txSignature;
            } catch (err) {
                const message = formatTransactionError(err);
                // Keep detailed error in devtools while presenting concise UI text.
                console.error('Transaction send failed', err);
                addRecentTransaction({
                    id: `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`,
                    signature: txSignature,
                    action: options?.action ?? 'Transaction',
                    timestamp: Date.now(),
                    status: 'failed',
                    error: message,
                    values: options?.values,
                });
                setState({ sending: false, signature: null, error: message });
                return null;
            }
        },
        [rpc, rpcSubscriptions, createSigner, addRecentTransaction],
    );

    const reset = useCallback(() => {
        setState({ sending: false, signature: null, error: null });
    }, []);

    return { ...state, send, reset };
}
