use super::{AddTimelockAccounts, AddTimelockData};
use crate::{impl_instruction, traits::Instruction};

/// AddTimelock instruction
pub struct AddTimelock<'a> {
    pub accounts: AddTimelockAccounts<'a>,
    pub data: AddTimelockData,
}

impl_instruction!(AddTimelock, AddTimelockAccounts, AddTimelockData);

impl<'a> Instruction<'a> for AddTimelock<'a> {
    type Accounts = AddTimelockAccounts<'a>;
    type Data = AddTimelockData;

    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }
}
