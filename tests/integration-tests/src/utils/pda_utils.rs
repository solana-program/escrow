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

pub fn find_noncanonical_program_address(seeds: &[&[u8]], program_id: &Pubkey) -> Option<(Pubkey, u8)> {
    let (_, canonical_bump) = Pubkey::find_program_address(seeds, program_id);

    for bump in (0u8..=u8::MAX).rev() {
        if bump == canonical_bump {
            continue;
        }

        let bump_seed = [bump];
        let mut seeds_with_bump = seeds.to_vec();
        seeds_with_bump.push(&bump_seed);

        if let Ok(address) = Pubkey::create_program_address(&seeds_with_bump, program_id) {
            return Some((address, bump));
        }
    }

    None
}
