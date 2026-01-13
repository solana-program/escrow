use super::{BlockMintAccounts, BlockMintData};
use crate::{impl_instruction, traits::Instruction};

/// BlockMint instruction combining accounts and data
pub struct BlockMint<'a> {
    pub accounts: BlockMintAccounts<'a>,
    pub data: BlockMintData,
}

impl_instruction!(BlockMint, BlockMintAccounts, BlockMintData);

impl<'a> Instruction<'a> for BlockMint<'a> {
    type Accounts = BlockMintAccounts<'a>;
    type Data = BlockMintData;

    #[inline(always)]
    fn accounts(&self) -> &Self::Accounts {
        &self.accounts
    }

    #[inline(always)]
    fn data(&self) -> &Self::Data {
        &self.data
    }
}
