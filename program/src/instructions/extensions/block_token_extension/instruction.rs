use super::{BlockTokenExtensionAccounts, BlockTokenExtensionData};
use crate::{impl_instruction, traits::Instruction};

/// BlockTokenExtension instruction
pub struct BlockTokenExtension<'a> {
    pub accounts: BlockTokenExtensionAccounts<'a>,
    pub data: BlockTokenExtensionData,
}

impl_instruction!(BlockTokenExtension, BlockTokenExtensionAccounts, BlockTokenExtensionData);

impl<'a> Instruction<'a> for BlockTokenExtension<'a> {
    type Accounts = BlockTokenExtensionAccounts<'a>;
    type Data = BlockTokenExtensionData;

    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    fn data(&self) -> &Self::Data {
        &self.data
    }
}
