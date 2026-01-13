use alloc::vec;
use alloc::vec::Vec;
use codama::CodamaAccount;
use pinocchio::{account::AccountView, cpi::Seed, error::ProgramError, Address};

use crate::assert_no_padding;
use crate::errors::EscrowProgramError::{InvalidReceiptEscrow, InvalidWithdrawer};
use crate::traits::{
    AccountParse, AccountSerialize, AccountSize, Discriminator, EscrowAccountDiscriminators, PdaSeeds, Versioned,
};

/// Receipt account state
///
/// # PDA Seeds
/// `[b"receipt", escrow.as_ref(), depositor.as_ref(), mint.as_ref(), receipt_seed.as_ref()]`
#[derive(Clone, Debug, PartialEq, CodamaAccount)]
#[repr(C)]
pub struct Receipt {
    pub bump: u8,
    _padding: [u8; 7],

    pub escrow: Address,
    pub depositor: Address,
    pub mint: Address,

    pub receipt_seed: Address,

    pub amount: u64,

    pub deposited_at: i64,
}

assert_no_padding!(Receipt, 1 + 7 + 32 + 32 + 32 + 32 + 8 + 8);

impl Discriminator for Receipt {
    const DISCRIMINATOR: u8 = EscrowAccountDiscriminators::ReceiptDiscriminator as u8;
}

impl Versioned for Receipt {
    const VERSION: u8 = 1;
}

impl AccountSize for Receipt {
    const DATA_LEN: usize = 1 + 7 + 32 + 32 + 32 + 32 + 8 + 8; // bump + padding + escrow + depositor + mint + receipt_seed + amount + deposited_at
}

impl AccountParse for Receipt {
    fn parse_from_bytes(data: &[u8]) -> Result<Self, ProgramError> {
        if data.len() < Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        if data[0] != Self::DISCRIMINATOR {
            return Err(ProgramError::InvalidAccountData);
        }

        // Skip discriminator (byte 0) and version (byte 1)
        let data = &data[2..];

        let bump = data[0];
        // Skip padding bytes [1..8]
        let escrow = Address::new_from_array(data[8..40].try_into().unwrap());
        let depositor = Address::new_from_array(data[40..72].try_into().unwrap());
        let mint = Address::new_from_array(data[72..104].try_into().unwrap());
        let receipt_seed = Address::new_from_array(data[104..136].try_into().unwrap());
        let amount = u64::from_le_bytes(data[136..144].try_into().unwrap());
        let deposited_at = i64::from_le_bytes(data[144..152].try_into().unwrap());

        Ok(Self::new(amount, deposited_at, escrow, depositor, mint, receipt_seed, bump))
    }
}

impl AccountSerialize for Receipt {
    #[inline(always)]
    fn to_bytes_inner(&self) -> Vec<u8> {
        let mut data = Vec::with_capacity(Self::DATA_LEN);
        data.push(self.bump);
        data.extend_from_slice(&[0u8; 7]); // padding
        data.extend_from_slice(self.escrow.as_ref());
        data.extend_from_slice(self.depositor.as_ref());
        data.extend_from_slice(self.mint.as_ref());
        data.extend_from_slice(self.receipt_seed.as_ref());
        data.extend_from_slice(&self.amount.to_le_bytes());
        data.extend_from_slice(&self.deposited_at.to_le_bytes());
        data
    }
}

impl PdaSeeds for Receipt {
    const PREFIX: &'static [u8] = b"receipt";

    #[inline(always)]
    fn seeds(&self) -> Vec<&[u8]> {
        vec![
            Self::PREFIX,
            self.escrow.as_ref(),
            self.depositor.as_ref(),
            self.mint.as_ref(),
            self.receipt_seed.as_ref(),
        ]
    }

    #[inline(always)]
    fn seeds_with_bump<'a>(&'a self, bump: &'a [u8; 1]) -> Vec<Seed<'a>> {
        vec![
            Seed::from(Self::PREFIX),
            Seed::from(self.escrow.as_ref()),
            Seed::from(self.depositor.as_ref()),
            Seed::from(self.mint.as_ref()),
            Seed::from(self.receipt_seed.as_ref()),
            Seed::from(bump.as_slice()),
        ]
    }
}

impl Receipt {
    #[inline(always)]
    pub fn new(
        amount: u64,
        deposited_at: i64,
        escrow: Address,
        depositor: Address,
        mint: Address,
        receipt_seed: Address,
        bump: u8,
    ) -> Self {
        Self { amount, deposited_at, escrow, depositor, mint, receipt_seed, bump, _padding: [0u8; 7] }
    }

    #[inline(always)]
    pub fn from_account(data: &[u8], account: &AccountView, program_id: &Address) -> Result<Self, ProgramError> {
        let state = Self::parse_from_bytes(data)?;
        state.validate_pda(account, program_id, state.bump)?;
        Ok(state)
    }

    #[inline(always)]
    pub fn validate_depositor(&self, escrow: &Address, depositor: &Address) -> Result<(), ProgramError> {
        if self.escrow != *escrow {
            return Err(InvalidReceiptEscrow.into());
        }

        if self.depositor != *depositor {
            return Err(InvalidWithdrawer.into());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_receipt() -> Receipt {
        let escrow = Address::new_from_array([1u8; 32]);
        let depositor = Address::new_from_array([2u8; 32]);
        let mint = Address::new_from_array([3u8; 32]);
        let receipt_seed = Address::new_from_array([4u8; 32]);
        Receipt::new(1000, 1234567890, escrow, depositor, mint, receipt_seed, 255)
    }

    #[test]
    fn test_receipt_new() {
        let escrow = Address::new_from_array([1u8; 32]);
        let depositor = Address::new_from_array([2u8; 32]);
        let mint = Address::new_from_array([3u8; 32]);
        let receipt_seed = Address::new_from_array([4u8; 32]);

        let receipt = Receipt::new(5000, 9999, escrow, depositor, mint, receipt_seed, 200);

        assert_eq!(receipt.amount, 5000);
        assert_eq!(receipt.deposited_at, 9999);
        assert_eq!(receipt.escrow, escrow);
        assert_eq!(receipt.depositor, depositor);
        assert_eq!(receipt.mint, mint);
        assert_eq!(receipt.receipt_seed, receipt_seed);
        assert_eq!(receipt.bump, 200);
    }

    #[test]
    fn test_receipt_to_bytes_inner() {
        let receipt = create_test_receipt();
        let bytes = receipt.to_bytes_inner();

        assert_eq!(bytes.len(), Receipt::DATA_LEN);
        assert_eq!(bytes[0], 255); // bump
        assert_eq!(&bytes[1..8], &[0u8; 7]); // padding
        assert_eq!(&bytes[8..40], &[1u8; 32]); // escrow
        assert_eq!(&bytes[40..72], &[2u8; 32]); // depositor
        assert_eq!(&bytes[72..104], &[3u8; 32]); // mint
        assert_eq!(&bytes[104..136], &[4u8; 32]); // receipt_seed
        assert_eq!(&bytes[136..144], &1000u64.to_le_bytes()); // amount
        assert_eq!(&bytes[144..152], &1234567890i64.to_le_bytes()); // deposited_at
    }

    #[test]
    fn test_receipt_to_bytes() {
        let receipt = create_test_receipt();
        let bytes = receipt.to_bytes();

        assert_eq!(bytes.len(), Receipt::LEN);
        assert_eq!(bytes[0], Receipt::DISCRIMINATOR);
        assert_eq!(bytes[1], Receipt::VERSION); // version auto-prepended
        assert_eq!(bytes[2], 255); // bump
    }

    #[test]
    fn test_receipt_seeds() {
        let receipt = create_test_receipt();
        let seeds = receipt.seeds();

        assert_eq!(seeds.len(), 5);
        assert_eq!(seeds[0], Receipt::PREFIX);
        assert_eq!(seeds[1], receipt.escrow.as_ref());
        assert_eq!(seeds[2], receipt.depositor.as_ref());
        assert_eq!(seeds[3], receipt.mint.as_ref());
        assert_eq!(seeds[4], receipt.receipt_seed.as_ref());
    }

    #[test]
    fn test_receipt_seeds_with_bump() {
        let receipt = create_test_receipt();
        let bump = [255u8];
        let seeds = receipt.seeds_with_bump(&bump);

        assert_eq!(seeds.len(), 6);
    }

    #[test]
    fn test_receipt_parse_from_bytes() {
        let receipt = create_test_receipt();
        let bytes = receipt.to_bytes();

        let parsed = Receipt::parse_from_bytes(&bytes).unwrap();

        assert_eq!(parsed.amount, receipt.amount);
        assert_eq!(parsed.deposited_at, receipt.deposited_at);
        assert_eq!(parsed.escrow, receipt.escrow);
        assert_eq!(parsed.depositor, receipt.depositor);
        assert_eq!(parsed.mint, receipt.mint);
        assert_eq!(parsed.receipt_seed, receipt.receipt_seed);
        assert_eq!(parsed.bump, receipt.bump);
    }

    #[test]
    fn test_receipt_parse_from_bytes_too_short() {
        let data = [0u8; 10];
        let result = Receipt::parse_from_bytes(&data);
        assert!(result.is_err());
    }

    #[test]
    fn test_receipt_parse_from_bytes_wrong_discriminator() {
        let mut bytes = [0u8; Receipt::LEN];
        bytes[0] = 99; // wrong discriminator
        let result = Receipt::parse_from_bytes(&bytes);
        assert!(result.is_err());
    }
}
