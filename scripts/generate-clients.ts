/**
 * Generates TypeScript and Rust clients from the Codama IDL.
 */

import fs from 'fs';
import path from 'path';
import { preserveConfigFiles } from './lib/utils';
import { createEscrowCodamaBuilder } from './lib/escrow-codama-builder';
import { renderVisitor as renderRustVisitor } from '@codama/renderers-rust';
import { renderVisitor as renderJavaScriptVisitor } from '@codama/renderers-js';

const projectRoot = path.join(__dirname, '..');
const idlDir = path.join(projectRoot, 'idl');
const escrowIdl = require(path.join(idlDir, 'escrow_program.json'));
const rustClientsDir = path.join(__dirname, '..', 'clients', 'rust');
const typescriptClientsDir = path.join(__dirname, '..', 'clients', 'typescript');

const escrowCodama = createEscrowCodamaBuilder(escrowIdl)
    .appendAccountDiscriminator()
    .appendAccountVersion()
    .appendPdaDerivers()
    .setInstructionAccountDefaultValues()
    .updateInstructionBumps()
    .removeEmitInstruction()
    .build();

// Preserve configuration files during generation
const configPreserver = preserveConfigFiles(typescriptClientsDir, rustClientsDir);

// Generate Rust client
escrowCodama.accept(
    renderRustVisitor(path.join(rustClientsDir, 'src', 'generated'), {
        formatCode: true,
        crateFolder: rustClientsDir,
        deleteFolderBeforeRendering: true,
    }),
);

// Generate TypeScript client
escrowCodama.accept(
    renderJavaScriptVisitor(path.join(typescriptClientsDir, 'src', 'generated'), {
        formatCode: true,
        deleteFolderBeforeRendering: true,
    }),
);

// Restore configuration files after generation
configPreserver.restore();

// Post-process generated Address type to use solana_address::Address instead of Pubkey
const addressRsPath = path.join(rustClientsDir, 'src', 'generated', 'types', 'address.rs');
if (fs.existsSync(addressRsPath)) {
    let content = fs.readFileSync(addressRsPath, 'utf-8');
    // Replace Pubkey with solana_address::Address
    content = content.replace(/use solana_pubkey::Pubkey;/, 'use solana_address::Address as SolanaAddress;');
    content = content.replace(/pub type Address = Pubkey;/, 'pub type Address = SolanaAddress;');
    fs.writeFileSync(addressRsPath, content);
}
