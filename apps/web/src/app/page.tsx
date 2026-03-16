'use client';

import { useState } from 'react';
import { Button } from '@solana/design-system/button';
import { WalletButton } from '@/components/WalletButton';
import { RpcBadge } from '@/components/RpcBadge';
import { ProgramBadge } from '@/components/ProgramBadge';
import { QuickDefaults } from '@/components/QuickDefaults';
import { RecentTransactions } from '@/components/RecentTransactions';
import { CreateEscrow } from '@/components/instructions/CreateEscrow';
import { UpdateAdmin } from '@/components/instructions/UpdateAdmin';
import { AllowMint } from '@/components/instructions/AllowMint';
import { BlockMint } from '@/components/instructions/BlockMint';
import { AddTimelock } from '@/components/instructions/AddTimelock';
import { SetHook } from '@/components/instructions/SetHook';
import { BlockTokenExtension } from '@/components/instructions/BlockTokenExtension';
import { SetArbiter } from '@/components/instructions/SetArbiter';
import { Deposit } from '@/components/instructions/Deposit';
import { Withdraw } from '@/components/instructions/Withdraw';

type InstructionId =
    | 'createEscrow'
    | 'updateAdmin'
    | 'allowMint'
    | 'blockMint'
    | 'addTimelock'
    | 'setHook'
    | 'blockTokenExtension'
    | 'setArbiter'
    | 'deposit'
    | 'withdraw';

const NAV: {
    group: string;
    items: { id: InstructionId; label: string }[];
}[] = [
    {
        group: 'ADMIN',
        items: [
            { id: 'createEscrow', label: 'Create Escrow' },
            { id: 'updateAdmin', label: 'Update Admin' },
            { id: 'allowMint', label: 'Allow Mint' },
            { id: 'blockMint', label: 'Block Mint' },
        ],
    },
    {
        group: 'EXTENSIONS',
        items: [
            { id: 'addTimelock', label: 'Add Timelock' },
            { id: 'setHook', label: 'Set Hook' },
            { id: 'blockTokenExtension', label: 'Block Token Ext' },
            { id: 'setArbiter', label: 'Set Arbiter' },
        ],
    },
    {
        group: 'OPERATIONS',
        items: [
            { id: 'deposit', label: 'Deposit' },
            { id: 'withdraw', label: 'Withdraw' },
        ],
    },
];

const PANELS: Record<InstructionId, { title: string; component: React.ComponentType }> = {
    createEscrow: { title: 'Create Escrow', component: CreateEscrow },
    updateAdmin: { title: 'Update Admin', component: UpdateAdmin },
    allowMint: { title: 'Allow Mint', component: AllowMint },
    blockMint: { title: 'Block Mint', component: BlockMint },
    addTimelock: { title: 'Add Timelock', component: AddTimelock },
    setHook: { title: 'Set Hook', component: SetHook },
    blockTokenExtension: { title: 'Block Token Extension', component: BlockTokenExtension },
    setArbiter: { title: 'Set Arbiter', component: SetArbiter },
    deposit: { title: 'Deposit', component: Deposit },
    withdraw: { title: 'Withdraw', component: Withdraw },
};

export default function HomePage() {
    const [active, setActive] = useState<InstructionId>('createEscrow');
    const panel = PANELS[active];
    const Panel = panel.component;

    return (
        <div style={{ minHeight: '100vh', display: 'flex', flexDirection: 'column' }}>
            {/* Header */}
            <header
                style={{
                    display: 'flex',
                    alignItems: 'center',
                    justifyContent: 'space-between',
                    padding: '12px 24px',
                    borderBottom: '1px solid var(--color-border)',
                    background: 'var(--color-card)',
                    position: 'sticky',
                    top: 0,
                    zIndex: 10,
                }}
            >
                <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
                    <span style={{ fontWeight: 700, fontSize: '1rem', color: 'var(--color-accent)' }}>
                        Escrow Program
                    </span>
                    <RpcBadge />
                    <ProgramBadge />
                </div>
                <WalletButton />
            </header>

            {/* Body */}
            <div style={{ display: 'flex', flex: 1, overflow: 'hidden' }}>
                {/* Sidebar */}
                <nav
                    style={{
                        width: 200,
                        borderRight: '1px solid var(--color-border)',
                        padding: '16px 0',
                        flexShrink: 0,
                        overflowY: 'auto',
                    }}
                >
                    {NAV.map(({ group, items }) => (
                        <div key={group} style={{ marginBottom: 24 }}>
                            <div
                                style={{
                                    fontSize: '0.6875rem',
                                    fontWeight: 700,
                                    color: 'var(--color-muted)',
                                    letterSpacing: '0.08em',
                                    padding: '0 16px',
                                    marginBottom: 6,
                                }}
                            >
                                {group}
                            </div>
                            {items.map(item => (
                                <Button
                                    key={item.id}
                                    onClick={() => setActive(item.id)}
                                    variant={active === item.id ? 'primary' : 'secondary'}
                                    size="sm"
                                    style={{
                                        width: '100%',
                                        justifyContent: 'flex-start',
                                        borderRadius: 0,
                                    }}
                                >
                                    {item.label}
                                </Button>
                            ))}
                        </div>
                    ))}
                </nav>

                {/* Main panel */}
                <main style={{ flex: 1, padding: '32px 40px', overflowY: 'auto' }}>
                    <QuickDefaults />
                    <RecentTransactions />
                    <h2
                        style={{
                            fontSize: '1.125rem',
                            fontWeight: 600,
                            marginBottom: 24,
                            paddingBottom: 16,
                            borderBottom: '1px solid var(--color-border)',
                        }}
                    >
                        {panel.title}
                    </h2>
                    <div style={{ maxWidth: 520 }}>
                        <Panel />
                    </div>
                </main>
            </div>
        </div>
    );
}
