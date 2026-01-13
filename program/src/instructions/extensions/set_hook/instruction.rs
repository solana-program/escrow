use super::{SetHookAccounts, SetHookData};
use crate::{impl_instruction, traits::Instruction};

/// SetHook instruction
pub struct SetHook<'a> {
    pub accounts: SetHookAccounts<'a>,
    pub data: SetHookData,
}

impl_instruction!(SetHook, SetHookAccounts, SetHookData);

impl<'a> Instruction<'a> for SetHook<'a> {
    type Accounts = SetHookAccounts<'a>;
    type Data = SetHookData;

    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }
}
