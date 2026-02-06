use super::{SetArbiterAccounts, SetArbiterData};
use crate::{impl_instruction, traits::Instruction};

/// SetArbiter instruction
pub struct SetArbiter<'a> {
    pub accounts: SetArbiterAccounts<'a>,
    pub data: SetArbiterData,
}

impl_instruction!(SetArbiter, SetArbiterAccounts, SetArbiterData);

impl<'a> Instruction<'a> for SetArbiter<'a> {
    type Accounts = SetArbiterAccounts<'a>;
    type Data = SetArbiterData;

    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }
}
