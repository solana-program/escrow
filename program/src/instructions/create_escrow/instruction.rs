use super::{CreateEscrowAccounts, CreateEscrowData};
use crate::{impl_instruction, traits::Instruction};

/// CreateEscrow instruction combining accounts and data
pub struct CreateEscrow<'a> {
    pub accounts: CreateEscrowAccounts<'a>,
    pub data: CreateEscrowData,
}

impl_instruction!(CreateEscrow, CreateEscrowAccounts, CreateEscrowData);

impl<'a> Instruction<'a> for CreateEscrow<'a> {
    type Accounts = CreateEscrowAccounts<'a>;
    type Data = CreateEscrowData;

    #[inline(always)]
    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    #[inline(always)]
    fn data(&self) -> &Self::Data {
        &self.data
    }
}
