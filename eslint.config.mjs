import solanaConfig from '@solana/eslint-config-solana';

export default [
    ...solanaConfig,
    {
        ignores: [
            '**/dist/**',
            '**/node_modules/**',
            '**/target/**',
            '**/generated/**',
            'clients/typescript/src/generated/**',
            '**/.next/**',
            '**/e2e/**',
            '**/playwright-report/**',
            '**/test-results/**',
            'eslint.config.mjs',
            '**/*.mjs',
        ],
    },
];
