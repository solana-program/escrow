/**
 * E2E tests for the Escrow Program devnet UI.
 *
 * Tests run serially and share on-chain state (a single escrow created in the first test).
 * Set PLAYRIGHT_WALLET (base58 secret key) and optionally APP_URL in .env at repo root.
 *
 * Known on-chain failures are assertions too — they verify the UI surfaces the right error.
 */
import { expect, type Page, test } from '@playwright/test';

import { connectWallet, injectWallet } from './helpers/wallet';

// ─── Constants ────────────────────────────────────────────────────────────────

const DEVNET_USDC_MINT = '4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU';
const SYSTEM_PROGRAM = '11111111111111111111111111111111';
// Wrapped SOL has no AllowedMint PDA so Block Mint should reject it (tests the error path).
const WRAPPED_SOL = 'So11111111111111111111111111111111111111112';

// ─── Shared state (populated by earlier tests) ───────────────────────────────

let walletAddress = '';
let escrowPda = '';
let receiptPda = '';

// ─── Helpers ─────────────────────────────────────────────────────────────────

/**
 * Navigate to a panel via the sidebar.
 * `navLabel` is the sidebar button text (may be abbreviated); `headingName` is the h2 on the panel.
 * When they match, only one argument is needed.
 */
async function openPanel(page: Page, headingName: string, navLabel?: string): Promise<void> {
    await page.getByRole('button', { exact: true, name: navLabel ?? headingName }).click();
    await expect(page.getByRole('heading', { level: 2, name: headingName })).toBeVisible();
}

/** Click the single Autofill button on the active panel. */
async function autofill(page: Page, nth = 0): Promise<void> {
    await page.getByRole('button', { name: 'Autofill' }).nth(nth).click();
}

/**
 * Clicks Send and waits for the transaction to land (success or failure).
 *
 * Reads the RecentTransactions count BEFORE clicking so fast devnet confirmations
 * (< 500ms) don't cause a TOCTOU race. Returns 'success' | 'failed'.
 */
async function sendAndWait(page: Page): Promise<'failed' | 'success'> {
    const heading = page.getByRole('heading', { name: /Recent Transactions/ });

    // Snapshot count BEFORE clicking send — must happen first to avoid races.
    // The heading is only rendered once there is ≥1 transaction, so default to 0.
    const beforeText = (await heading.textContent({ timeout: 500 }).catch(() => '')) ?? '';
    const beforeCount = parseInt(beforeText.match(/\d+/)?.[0] ?? '0');

    await page.getByRole('button', { name: 'Send Transaction' }).click();

    // Wait until a new entry appears (count increases by 1).
    await expect(async () => {
        const text = (await heading.textContent()) ?? '';
        const count = parseInt(text.match(/\d+/)?.[0] ?? '0');
        expect(count).toBeGreaterThan(beforeCount);
    }).toPass({ intervals: [500, 1000, 2000], timeout: 45_000 });

    if (await page.getByText('Success', { exact: true }).last().isVisible()) return 'success';
    return 'failed';
}

// ─── Suite setup ─────────────────────────────────────────────────────────────

test.describe('Escrow Program UI', () => {
    test.describe.configure({ mode: 'serial' });

    let page: Page;

    test.beforeAll(async ({ browser }) => {
        const walletKey = process.env.PLAYRIGHT_WALLET;
        if (!walletKey) throw new Error('PLAYRIGHT_WALLET env var is not set');

        page = await browser.newPage();
        await page.goto('/');
        walletAddress = await injectWallet(page, walletKey);
        await connectWallet(page);
    });

    test.afterAll(async () => {
        await page.close();
    });

    // ─── Instruction: Create Escrow ──────────────────────────────────────────

    test('Create Escrow — succeeds and saves PDA to QuickDefaults', async () => {
        await openPanel(page, 'Create Escrow');
        await expect(page.getByRole('textbox', { name: 'Admin Address' })).toHaveValue(walletAddress);

        const result = await sendAndWait(page);
        expect(result).toBe('success');

        // The escrow PDA is saved automatically to the QuickDefaults combobox.
        const defaultEscrow = page.getByRole('combobox', { name: 'Default Escrow' });
        await expect(defaultEscrow).not.toHaveValue('');
        escrowPda = await defaultEscrow.inputValue();
        expect(escrowPda.length).toBeGreaterThanOrEqual(32);
        expect(escrowPda.length).toBeLessThanOrEqual(44);

        await expect(page.locator('text=1 saved').first()).toBeVisible();
    });

    // ─── Instruction: Allow Mint ─────────────────────────────────────────────

    test('Allow Mint — succeeds for devnet USDC', async () => {
        await openPanel(page, 'Allow Mint');
        await autofill(page, 0); // Escrow
        await page.getByRole('textbox', { name: 'Mint Address' }).fill(DEVNET_USDC_MINT);

        expect(await sendAndWait(page)).toBe('success');

        // Mint is saved to QuickDefaults after success.
        await expect(page.getByRole('combobox', { name: 'Default Mint' })).toHaveValue(DEVNET_USDC_MINT);
    });

    // ─── Instruction: Deposit ────────────────────────────────────────────────

    test('Deposit — succeeds and saves receipt PDA to QuickDefaults', async () => {
        await openPanel(page, 'Deposit');
        await autofill(page, 0); // Escrow
        await autofill(page, 1); // Mint
        await page.getByRole('spinbutton', { name: 'Amount (in base units)' }).fill('100');

        expect(await sendAndWait(page)).toBe('success');

        const defaultReceipt = page.getByRole('combobox', { name: 'Default Receipt' });
        await expect(defaultReceipt).not.toHaveValue('');
        receiptPda = await defaultReceipt.inputValue();
        expect(receiptPda.length).toBeGreaterThanOrEqual(32);
        expect(receiptPda.length).toBeLessThanOrEqual(44);
    });

    // ─── Instruction: Add Timelock ───────────────────────────────────────────

    test('Add Timelock — succeeds with 1s duration', async () => {
        await openPanel(page, 'Add Timelock');
        await autofill(page);
        await page.getByRole('spinbutton', { name: 'Lock Duration (seconds)' }).fill('1');

        expect(await sendAndWait(page)).toBe('success');
    });

    // ─── Instruction: Set Arbiter ────────────────────────────────────────────

    test('Set Arbiter — succeeds (connected wallet as arbiter)', async () => {
        await openPanel(page, 'Set Arbiter');
        await autofill(page);

        expect(await sendAndWait(page)).toBe('success');
    });

    // ─── Instruction: Withdraw ───────────────────────────────────────────────
    // Runs before Set Hook so only the arbiter extension is active.
    // The arbiter (connected wallet) is auto-detected from the extensions PDA.

    test('Withdraw — succeeds (arbiter auto-detected and signed automatically)', async () => {
        await openPanel(page, 'Withdraw');
        await autofill(page, 0); // Escrow
        await autofill(page, 1); // Mint
        await autofill(page, 2); // Receipt

        expect(await sendAndWait(page)).toBe('success');
    });

    // ─── Instruction: Set Hook ───────────────────────────────────────────────

    test('Set Hook — succeeds with System Program as hook address', async () => {
        await openPanel(page, 'Set Hook');
        await autofill(page);
        await page.getByRole('textbox', { name: 'Hook Program Address' }).fill(SYSTEM_PROGRAM);

        expect(await sendAndWait(page)).toBe('success');
    });

    // ─── Instruction: Block Token Extension ─────────────────────────────────

    test('Block Token Extension — succeeds for NonTransferable (type 5)', async () => {
        await openPanel(page, 'Block Token Extension', 'Block Token Ext');
        await autofill(page);
        // Extension Type defaults to 5 (NonTransferable); no change needed.

        expect(await sendAndWait(page)).toBe('success');
    });

    // ─── Instruction: Block Mint ─────────────────────────────────────────────

    test('Block Mint — fails for mint that was never allowed (wSOL)', async () => {
        await openPanel(page, 'Block Mint');
        await autofill(page, 0); // Escrow
        await page.getByRole('textbox', { name: 'Mint Address' }).fill(WRAPPED_SOL);

        expect(await sendAndWait(page)).toBe('failed');
        await expect(page.getByText('Transaction failed').last()).toBeVisible();
    });

    test('Block Mint — succeeds for the previously allowed mint', async () => {
        await openPanel(page, 'Block Mint');
        await autofill(page, 0); // Escrow
        await autofill(page, 1); // Mint (autofills devnet USDC from QuickDefaults)

        expect(await sendAndWait(page)).toBe('success');
    });

    // ─── Instruction: Update Admin ───────────────────────────────────────────

    test('Update Admin — succeeds (idempotent, keeps same admin)', async () => {
        await openPanel(page, 'Update Admin');
        await autofill(page);

        expect(await sendAndWait(page)).toBe('success');
    });

    // ─── Client-side validation ──────────────────────────────────────────────

    test.describe('Client-side validation', () => {
        test.beforeEach(async () => {
            await openPanel(page, 'Deposit');
        });

        test('empty required field — browser native validation blocks submit', async () => {
            // All fields blank; clicking Send should NOT trigger a network request.
            // The browser focuses the first empty required input instead.
            const txCountBefore = await page.getByRole('heading', { name: /Recent Transactions/ }).textContent();

            await page.getByRole('button', { name: 'Send Transaction' }).click();

            // Transaction count must not change.
            await expect(page.getByRole('heading', { name: /Recent Transactions/ })).toHaveText(txCountBefore!);

            // Escrow field should be focused (browser scrolled to it).
            await expect(page.getByRole('textbox', { name: 'Escrow Address' })).toBeFocused();
        });

        test('invalid address — shows validation error without submitting', async () => {
            await page.getByRole('textbox', { name: 'Escrow Address' }).fill('notanaddress');
            await page.getByRole('textbox', { name: 'Mint Address' }).fill(DEVNET_USDC_MINT);
            await page.getByRole('spinbutton', { name: 'Amount' }).fill('100');

            await page.getByRole('button', { name: 'Send Transaction' }).click();

            await expect(page.getByText('Escrow address is not a valid Solana address.')).toBeVisible();
        });

        test('zero amount — shows "Amount must be greater than 0"', async () => {
            await page.getByRole('textbox', { name: 'Escrow Address' }).fill(escrowPda);
            await page.getByRole('textbox', { name: 'Mint Address' }).fill(DEVNET_USDC_MINT);
            await page.getByRole('spinbutton', { name: 'Amount' }).fill('0');

            await page.getByRole('button', { name: 'Send Transaction' }).click();

            await expect(page.getByText('Amount must be greater than 0.')).toBeVisible();
        });

        test('negative amount — shows "Amount must be a whole number"', async () => {
            await page.getByRole('textbox', { name: 'Escrow Address' }).fill(escrowPda);
            await page.getByRole('textbox', { name: 'Mint Address' }).fill(DEVNET_USDC_MINT);
            await page.getByRole('spinbutton', { name: 'Amount' }).fill('-1');

            await page.getByRole('button', { name: 'Send Transaction' }).click();

            await expect(page.getByText('Amount must be a whole number.')).toBeVisible();
        });

        test('Withdraw — empty receipt shows validation error', async () => {
            await openPanel(page, 'Withdraw');
            await page.getByRole('textbox', { name: 'Escrow Address' }).fill(escrowPda);
            await page.getByRole('textbox', { name: 'Mint Address' }).fill(DEVNET_USDC_MINT);
            // Leave Receipt blank.

            await page.getByRole('button', { name: 'Send Transaction' }).click();

            // Browser native required validation focuses the receipt field.
            await expect(page.getByRole('textbox', { name: 'Receipt Address' })).toBeFocused();
        });
    });

    // ─── UI components ───────────────────────────────────────────────────────

    test.describe('UI components', () => {
        test('RPC badge opens dropdown with network presets and custom URL input', async () => {
            await page.getByRole('button', { name: /Devnet/ }).click();
            await expect(page.getByRole('button', { name: /Mainnet/i })).toBeVisible();
            await expect(page.getByRole('button', { name: /Testnet/i })).toBeVisible();
            await expect(page.getByRole('button', { name: /Localhost/i })).toBeVisible();
            await expect(page.getByRole('textbox', { name: /my-rpc/i })).toBeVisible();
            await page.keyboard.press('Escape');
        });

        test('Program badge shows editable program ID', async () => {
            await page.getByRole('button', { name: /Program:/ }).click();
            await expect(page.getByRole('textbox', { name: /Escrowae7/ })).toBeVisible();
            await page.keyboard.press('Escape');
        });

        test('QuickDefaults Clear removes all saved values', async () => {
            // Ensure at least escrow is saved from earlier tests.
            await expect(page.getByRole('combobox', { name: 'Default Escrow' })).not.toHaveValue('');

            await page.getByRole('button', { name: 'Clear Saved' }).click();

            await expect(page.getByRole('combobox', { name: 'Default Escrow' })).toHaveValue('');
            await expect(page.getByRole('combobox', { name: 'Default Mint' })).toHaveValue('');
            await expect(page.getByRole('combobox', { name: 'Default Receipt' })).toHaveValue('');
            await expect(page.getByText('0 saved').first()).toBeVisible();
        });

        test('RecentTransactions shows all successful txs with View Explorer links', async () => {
            // At this point we should have multiple successful transactions.
            const heading = page.getByRole('heading', { name: /Recent Transactions \(\d+\)/ });
            await expect(heading).toBeVisible();

            const count = parseInt((await heading.textContent())!.match(/\d+/)![0]);
            expect(count).toBeGreaterThanOrEqual(6); // Create, AllowMint, Deposit, Timelock, SetHook, BlockTokenExt, BlockMint, UpdateAdmin

            // Every successful entry should have a View Explorer button.
            const explorerButtons = page.getByRole('button', { name: 'View Explorer' });
            await expect(explorerButtons.first()).toBeVisible();
        });
    });
});
