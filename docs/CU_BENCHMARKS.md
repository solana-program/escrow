# Compute Unit Benchmarks

Compute unit (CU) usage per instruction, measured via LiteSVM integration tests with `CU_TRACKING=1`.

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

## Running Benchmarks

```bash
# Run with CU tracking enabled
./scripts/integration-test-with-cu.sh

# This will:
# 1. Run all integration tests with CU_TRACKING=1
# 2. Print a summary table to the terminal
# 3. Update this file with the latest results
```

## Notes

- **Best**: Minimum CUs observed across all test runs
- **Avg**: Average CUs across all test runs
- **Worst**: Maximum CUs observed (often includes account creation overhead)
- **Count**: Number of test invocations measured

CU usage varies based on:

- Account creation vs. existing accounts
- Extension data presence and size
- Hook program invocations
- Token program (SPL Token vs. Token-2022)
