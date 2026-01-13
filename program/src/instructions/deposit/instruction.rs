use super::{DepositAccounts, DepositData};
use crate::{impl_instruction, traits::Instruction};

/// Deposit instruction combining accounts and data
pub struct Deposit<'a> {
    pub accounts: DepositAccounts<'a>,
    pub data: DepositData,
}

impl_instruction!(Deposit, DepositAccounts, DepositData);

impl<'a> Instruction<'a> for Deposit<'a> {
    type Accounts = DepositAccounts<'a>;
    type Data = DepositData;

    #[inline(always)]
    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    #[inline(always)]
    fn data(&self) -> &Self::Data {
        &self.data
    }
}
