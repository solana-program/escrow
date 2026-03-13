import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
    transpilePackages: ['@solana/escrow-program-client'],
    env: {
        NEXT_PUBLIC_RPC_URL: process.env.NEXT_PUBLIC_RPC_URL ?? 'https://api.devnet.solana.com',
        NEXT_PUBLIC_PROGRAM_ID: process.env.NEXT_PUBLIC_PROGRAM_ID ?? 'Escrowae7RaUfNn4oEZHywMXE5zWzYCXenwrCDaEoifg',
    },
};

export default nextConfig;
