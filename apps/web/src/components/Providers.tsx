'use client';

import { RpcProvider } from '@/contexts/RpcContext';
import { WalletProvider } from '@/contexts/WalletContext';
import { RecentTransactionsProvider } from '@/contexts/RecentTransactionsContext';
import { SavedValuesProvider } from '@/contexts/SavedValuesContext';

export function Providers({ children }: { children: React.ReactNode }) {
    return (
        <RpcProvider>
            <WalletProvider>
                <RecentTransactionsProvider>
                    <SavedValuesProvider>{children}</SavedValuesProvider>
                </RecentTransactionsProvider>
            </WalletProvider>
        </RpcProvider>
    );
}
