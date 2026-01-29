# Possible Improvements

The following enhancements could be considered for future iterations of the program:

1. **Global Omni Vaults per Mint** - Implement shared vault accounts per mint across all escrows rather than individual vaults per escrow per mint. This would reduce account overhead and simplify token management at scale.

2. **Block All Token Extensions Option** - Add a configuration flag within the blocked extension data to reject all token extensions by default, providing a simpler security posture for escrows that require only standard SPL tokens or vanilla SPL 2022 tokens.

3. **Partial Withdrawals** - Enable withdrawals of a specified amount rather than requiring full balance withdrawals, allowing for more flexible fund disbursement patterns.

4. **TypeScript Client Testing** - Develop a comprehensive test suite for the generated TypeScript clients to ensure client-side reliability and validate the end-to-end integration with the on-chain program.

5. **Receipt Seed Space Optimization** - The current `receipt_seed` uses a 32-byte `Address` type. Two alternatives could save space:
    - **Use `u8` counter**: Change to a simple counter (0-255), saving 31 bytes per receipt. Limits to 256 receipts per depositor/escrow/mint combination, which is acceptable for most use cases.
    - **Single receipt with `deposit_additional` instruction**: Allow users to add to an existing receipt rather than creating new ones. This would require handling complexities around `deposited_at` timestamps (e.g., weighted average, use latest, or track per-deposit).
