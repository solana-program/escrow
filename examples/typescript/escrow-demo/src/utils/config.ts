/**
 * Configuration constants for the escrow demo
 */
export const CONFIG = {
    /** Amount to deposit (100, in raw units) */
    DEPOSIT_AMOUNT: 100_000_000n,

    /** Amount of tokens to mint (in raw units) */
    MINT_AMOUNT: 1_000_000_000n,

    // 100 tokens with 6 decimals
    /** Timelock duration in seconds */
    TIMELOCK_DURATION: 2n,

    /** Wait buffer after timelock (in ms) */
    TIMELOCK_WAIT_MS: 2500,

    // 10 SOL
    /** Token decimals for the test mint */
    TOKEN_DECIMALS: 6,
} as const;

/**
 * Sleep utility
 */
export function sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
}

/**
 * Truncate address for display
 */
export function truncateAddress(address: string): string {
    return `${address.slice(0, 8)}...`;
}

/**
 * Format token amount for display
 */
export function formatTokenAmount(amount: bigint, decimals: number = CONFIG.TOKEN_DECIMALS): string {
    const divisor = BigInt(10 ** decimals);
    const whole = amount / divisor;
    const fraction = amount % divisor;
    if (fraction === 0n) {
        return whole.toString();
    }
    const fractionStr = fraction.toString().padStart(decimals, '0').replace(/0+$/, '');
    return `${whole}.${fractionStr}`;
}
