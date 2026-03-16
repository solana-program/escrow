'use client';

import { RpcProvider } from '@/contexts/RpcContext';
import { ProgramProvider } from '@/contexts/ProgramContext';
import { WalletProvider } from '@/contexts/WalletContext';
import { RecentTransactionsProvider } from '@/contexts/RecentTransactionsContext';
import { SavedValuesProvider } from '@/contexts/SavedValuesContext';

export function Providers({ children }: { children: React.ReactNode }) {
    return (
        <RpcProvider>
            <ProgramProvider>
                <WalletProvider>
                    <RecentTransactionsProvider>
                        <SavedValuesProvider>{children}</SavedValuesProvider>
                    </RecentTransactionsProvider>
                </WalletProvider>
            </ProgramProvider>
        </RpcProvider>
    );
}
