use escrow_program_client::ESCROW_PROGRAM_ID;
use solana_sdk::pubkey::Pubkey;

const ESCROW_SEED: &[u8] = b"escrow";
const EXTENSIONS_SEED: &[u8] = b"extensions";
const EVENT_AUTHORITY_SEED: &[u8] = b"event_authority";
const RECEIPT_SEED: &[u8] = b"receipt";
const ALLOWED_MINT_SEED: &[u8] = b"allowed_mint";

pub fn find_escrow_pda(escrow_seed: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ESCROW_SEED, escrow_seed.as_ref()], &ESCROW_PROGRAM_ID)
}

pub fn find_extensions_pda(escrow: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[EXTENSIONS_SEED, escrow.as_ref()], &ESCROW_PROGRAM_ID)
}

pub fn find_event_authority_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[EVENT_AUTHORITY_SEED], &ESCROW_PROGRAM_ID)
}

pub fn find_receipt_pda(escrow: &Pubkey, depositor: &Pubkey, mint: &Pubkey, receipt_seed: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[RECEIPT_SEED, escrow.as_ref(), depositor.as_ref(), mint.as_ref(), receipt_seed.as_ref()],
        &ESCROW_PROGRAM_ID,
    )
}

pub fn find_allowed_mint_pda(escrow: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[ALLOWED_MINT_SEED, escrow.as_ref(), mint.as_ref()], &ESCROW_PROGRAM_ID)
}
