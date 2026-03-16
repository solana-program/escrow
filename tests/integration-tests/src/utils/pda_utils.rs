use escrow_program_client::accounts::{AllowedMint, Escrow, EscrowExtensionsHeader, EventAuthority, Receipt};
use solana_sdk::pubkey::Pubkey;

pub fn find_escrow_pda(escrow_seed: &Pubkey) -> (Pubkey, u8) {
    Escrow::find_pda(escrow_seed)
}

pub fn find_extensions_pda(escrow: &Pubkey) -> (Pubkey, u8) {
    EscrowExtensionsHeader::find_pda(escrow)
}

pub fn find_event_authority_pda() -> (Pubkey, u8) {
    EventAuthority::find_pda()
}

pub fn find_receipt_pda(escrow: &Pubkey, depositor: &Pubkey, mint: &Pubkey, receipt_seed: &Pubkey) -> (Pubkey, u8) {
    Receipt::find_pda(escrow, depositor, mint, receipt_seed)
}

pub fn find_allowed_mint_pda(escrow: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    AllowedMint::find_pda(escrow, mint)
}
