use super::{WithdrawAccounts, WithdrawData};
use crate::{impl_instruction, traits::Instruction};

/// Withdraw instruction combining accounts and data
pub struct Withdraw<'a> {
    pub accounts: WithdrawAccounts<'a>,
    pub data: WithdrawData,
}

impl_instruction!(Withdraw, WithdrawAccounts, WithdrawData);

impl<'a> Instruction<'a> for Withdraw<'a> {
    type Accounts = WithdrawAccounts<'a>;
    type Data = WithdrawData;

    #[inline(always)]
    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    #[inline(always)]
    fn data(&self) -> &Self::Data {
        &self.data
    }
}
