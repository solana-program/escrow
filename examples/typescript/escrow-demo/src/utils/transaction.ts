import {
    appendTransactionMessageInstructions,
    assertIsTransactionWithBlockhashLifetime,
    createTransactionMessage,
    getSignatureFromTransaction,
    type Instruction,
    pipe,
    Rpc,
    RpcSubscriptions,
    sendAndConfirmTransactionFactory,
    setTransactionMessageFeePayerSigner,
    setTransactionMessageLifetimeUsingBlockhash,
    signTransactionMessageWithSigners,
    SolanaRpcApi,
    SolanaRpcSubscriptionsApi,
    type TransactionSigner,
} from '@solana/kit';
import {
    estimateAndUpdateProvisoryComputeUnitLimitFactory,
    estimateComputeUnitLimitFactory,
    getSetComputeUnitLimitInstruction,
    getSetComputeUnitPriceInstruction,
    MAX_COMPUTE_UNIT_LIMIT,
} from '@solana-program/compute-budget';

// Helper to build and send transactions
async function buildAndSend({
    rpc,
    rpcSubscriptions,
    payer,
    instructions,
    microLamports = 1000n,
    skipComputeEstimate = false,
}: {
    instructions: readonly Instruction[];
    microLamports?: bigint;
    payer: TransactionSigner;
    rpc: Rpc<SolanaRpcApi>;
    rpcSubscriptions: RpcSubscriptions<SolanaRpcSubscriptionsApi>;
    skipComputeEstimate?: boolean;
}): Promise<string> {
    const sendAndConfirmTransaction = sendAndConfirmTransactionFactory({
        rpc,
        rpcSubscriptions,
    });

    const { value: latestBlockhash } = await rpc.getLatestBlockhash().send();

    // Build transaction with compute budget
    const computeBudgetIxs = [
        getSetComputeUnitLimitInstruction({ units: MAX_COMPUTE_UNIT_LIMIT }),
        getSetComputeUnitPriceInstruction({ microLamports }),
    ];

    const txMessage = pipe(
        createTransactionMessage({ version: 0 }),
        tx => setTransactionMessageFeePayerSigner(payer, tx),
        tx => setTransactionMessageLifetimeUsingBlockhash(latestBlockhash, tx),
        tx => appendTransactionMessageInstructions([...computeBudgetIxs, ...instructions], tx),
    );

    if (skipComputeEstimate) {
        const signedTx = await signTransactionMessageWithSigners(txMessage);

        // Send and confirm - cast to expected type since we know it's blockhash-based
        assertIsTransactionWithBlockhashLifetime(signedTx);
        await sendAndConfirmTransaction(signedTx, {
            commitment: 'confirmed',
            skipPreflight: true,
        });
        return getSignatureFromTransaction(signedTx);
    }

    // Estimate compute unit limit
    const estimateAndUpdateProvisoryComputeUnitLimit = estimateAndUpdateProvisoryComputeUnitLimitFactory(
        estimateComputeUnitLimitFactory({ rpc }),
    );
    let updatedTxMessage = await estimateAndUpdateProvisoryComputeUnitLimit(txMessage);

    // Refresh blockhash
    const { value: freshBlockhash } = await rpc.getLatestBlockhash().send();
    updatedTxMessage = setTransactionMessageLifetimeUsingBlockhash(freshBlockhash, updatedTxMessage);

    // Sign transaction
    const signedTx = await signTransactionMessageWithSigners(updatedTxMessage);

    // Send and confirm - cast to expected type since we know it's blockhash-based
    assertIsTransactionWithBlockhashLifetime(signedTx);
    await sendAndConfirmTransaction(signedTx, {
        commitment: 'confirmed',
    });

    return getSignatureFromTransaction(signedTx);
}

export { buildAndSend };
