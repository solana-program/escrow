# Escrow Program

> **⚠️ SECURITY NOTICE**: This program has not been audited. Use at your own risk. Not recommended for production use with real funds without a thorough security review. The authors and contributors are not responsible for any loss of funds or damages resulting from the use of this program.

<!-- CU_SUMMARY_START -->

| Instruction             | Best  | Avg   | Worst | Count |
| ----------------------- | ----- | ----- | ----- | ----- |
| AddBlockTokenExtensions | 7282  | 8800  | 23782 | 26    |
| AddTimelock             | 7386  | 9278  | 16386 | 12    |
| AllowMint               | 8567  | 13407 | 25840 | 22    |
| BlockMint               | 5337  | 5948  | 6837  | 5     |
| CreateEscrow            | 4802  | 5914  | 18302 | 58    |
| Deposit                 | 18819 | 26670 | 32536 | 4     |
| SetHook                 | 7421  | 9007  | 13421 | 5     |
| UpdateAdmin             | 3396  | 5646  | 9396  | 4     |
| Withdraw                | 15342 | 23462 | 30509 | 11    |

<!-- CU_SUMMARY_END -->

## Possible Improvements

The following enhancements could be considered for future iterations of the program:

1. **Global Omni Vaults per Mint** - Implement shared vault accounts per mint across all escrows rather than individual vaults per escrow per mint. This would reduce account overhead and simplify token management at scale.

2. **Block All Token Extensions Option** - Add a configuration flag within the blocked extension data to reject all token extensions by default, providing a simpler security posture for escrows that require only standard SPL tokens or vanilla SPL 2022 tokens.

3. **Partial Withdrawals** - Enable withdrawals of a specified amount rather than requiring full balance withdrawals, allowing for more flexible fund disbursement patterns.

4. **TypeScript Client Testing** - Develop a comprehensive test suite for the generated TypeScript clients to ensure client-side reliability and validate the end-to-end integration with the on-chain program.

5. **Receipt Seed Space Optimization** - The current `receipt_seed` uses a 32-byte `Address` type. Two alternatives could save space:
    - **Use `u8` counter**: Change to a simple counter (0-255), saving 31 bytes per receipt. Limits to 256 receipts per depositor/escrow/mint combination, which is acceptable for most use cases.
    - **Single receipt with `deposit_additional` instruction**: Allow users to add to an existing receipt rather than creating new ones. This would require handling complexities around `deposited_at` timestamps (e.g., weighted average, use latest, or track per-deposit).

---

Built and maintained by the Solana Foundation.

Licensed under MIT. See [LICENSE](LICENSE) for details.
