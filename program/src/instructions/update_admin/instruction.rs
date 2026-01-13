use super::{UpdateAdminAccounts, UpdateAdminData};
use crate::{impl_instruction, traits::Instruction};

/// UpdateAdmin instruction
pub struct UpdateAdmin<'a> {
    pub accounts: UpdateAdminAccounts<'a>,
    pub data: UpdateAdminData,
}

impl_instruction!(UpdateAdmin, UpdateAdminAccounts, UpdateAdminData);

impl<'a> Instruction<'a> for UpdateAdmin<'a> {
    type Accounts = UpdateAdminAccounts<'a>;
    type Data = UpdateAdminData;

    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }
}
