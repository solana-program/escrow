import { Address } from '@solana/kit';

import { CONFIG, formatTokenAmount, truncateAddress } from './config.js';

const TOTAL_STEPS = 8;

/**
 * Log a step header
 */
export function logStep(step: number, description: string): void {
    console.log(`\n[${step}/${TOTAL_STEPS}] ${description}`);
    console.log('━'.repeat(50));
}

/**
 * Log info with arrow prefix
 */
export function logInfo(key: string, value: string): void {
    console.log(`  → ${key}: ${value}`);
}

/**
 * Log success with checkmark
 */
export function logSuccess(message: string): void {
    console.log(`  ✓ ${message}`);
}

/**
 * Log error with X
 */
export function logError(message: string): void {
    console.log(`  ✗ ${message}`);
}

/**
 * Log address info (truncated)
 */
export function logAddress(label: string, address: string): void {
    logInfo(label, truncateAddress(address));
}

/**
 * Log separator
 */
function logSeparator(): void {
    console.log('\n' + '═'.repeat(50));
}

/**
 * Log title
 */
export function logTitle(title: string): void {
    console.log('\n' + '═'.repeat(50));
    console.log(`  ${title}`);
    console.log('═'.repeat(50));
}

/**
 * Log Summary
 */
export function logSummary(escrowPda: Address, mint: Address): void {
    logSeparator();
    console.log('\n  Demo completed successfully!');
    console.log('\n  Summary:');
    console.log(`    - Created escrow: ${truncateAddress(escrowPda)}`);
    console.log(`    - Test mint: ${truncateAddress(mint)}`);
    console.log(`    - Deposited: ${formatTokenAmount(CONFIG.DEPOSIT_AMOUNT)} tokens`);
    console.log(`    - Timelock: ${CONFIG.TIMELOCK_DURATION} seconds`);
    console.log(`    - Withdrew: ${formatTokenAmount(CONFIG.DEPOSIT_AMOUNT)} tokens`);
    logSeparator();
}
