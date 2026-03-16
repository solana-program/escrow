'use client';

import { useEffect, useRef, useState } from 'react';
import { Button } from '@solana/design-system/button';
import { TextInput } from '@solana/design-system/text-input';
import { DEFAULT_PROGRAM_ID, useProgramContext } from '@/contexts/ProgramContext';

function truncateAddress(address: string): string {
    return `${address.slice(0, 8)}…${address.slice(-8)}`;
}

export function ProgramBadge() {
    const { programId, setProgramId } = useProgramContext();
    const [open, setOpen] = useState(false);
    const [customInput, setCustomInput] = useState('');
    const containerRef = useRef<HTMLDivElement | null>(null);

    const label = programId === DEFAULT_PROGRAM_ID ? 'Default' : truncateAddress(programId);

    useEffect(() => {
        const handlePointerDown = (event: MouseEvent) => {
            if (!open) return;
            if (!containerRef.current?.contains(event.target as Node)) {
                setOpen(false);
            }
        };

        const handleEscape = (event: KeyboardEvent) => {
            if (event.key === 'Escape') {
                setOpen(false);
            }
        };

        document.addEventListener('mousedown', handlePointerDown);
        document.addEventListener('keydown', handleEscape);
        return () => {
            document.removeEventListener('mousedown', handlePointerDown);
            document.removeEventListener('keydown', handleEscape);
        };
    }, [open]);

    return (
        <div ref={containerRef} style={{ position: 'relative' }}>
            <Button
                onClick={() => setOpen(v => !v)}
                variant="secondary"
                size="sm"
                style={{
                    fontSize: '0.75rem',
                    display: 'flex',
                    alignItems: 'center',
                    gap: 4,
                }}
            >
                Program: {label} ▾
            </Button>

            {open && (
                <div
                    style={{
                        position: 'absolute',
                        top: '110%',
                        left: 0,
                        background: 'var(--color-card)',
                        border: '1px solid var(--color-border)',
                        borderRadius: 6,
                        minWidth: 340,
                        zIndex: 100,
                        overflow: 'hidden',
                    }}
                >
                    <div
                        style={{
                            padding: '8px 10px',
                            display: 'flex',
                            gap: 6,
                            alignItems: 'center',
                        }}
                    >
                        <div style={{ flex: 1 }}>
                            <TextInput
                                value={customInput}
                                onChange={e => setCustomInput(e.target.value)}
                                placeholder={DEFAULT_PROGRAM_ID}
                                size="md"
                                onKeyDown={e => {
                                    if (e.key === 'Enter' && customInput) {
                                        setProgramId(customInput);
                                        setCustomInput('');
                                        setOpen(false);
                                    }
                                }}
                            />
                        </div>
                        <Button
                            onClick={() => {
                                if (customInput) {
                                    setProgramId(customInput);
                                    setCustomInput('');
                                    setOpen(false);
                                }
                            }}
                            size="sm"
                        >
                            Set
                        </Button>
                    </div>
                    {programId !== DEFAULT_PROGRAM_ID && (
                        <div
                            style={{
                                borderTop: '1px solid var(--color-border)',
                                padding: '6px 10px',
                            }}
                        >
                            <Button
                                onClick={() => {
                                    setProgramId(DEFAULT_PROGRAM_ID);
                                    setOpen(false);
                                }}
                                variant="secondary"
                                size="sm"
                                style={{ width: '100%', fontSize: '0.8125rem' }}
                            >
                                Reset to default
                            </Button>
                        </div>
                    )}
                </div>
            )}
        </div>
    );
}
