use super::{AllowMintAccounts, AllowMintData};
use crate::{impl_instruction, traits::Instruction};

/// AllowMint instruction combining accounts and data
pub struct AllowMint<'a> {
    pub accounts: AllowMintAccounts<'a>,
    pub data: AllowMintData,
}

impl_instruction!(AllowMint, AllowMintAccounts, AllowMintData);

impl<'a> Instruction<'a> for AllowMint<'a> {
    type Accounts = AllowMintAccounts<'a>;
    type Data = AllowMintData;

    #[inline(always)]
    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    #[inline(always)]
    fn data(&self) -> &Self::Data {
        &self.data
    }
}
