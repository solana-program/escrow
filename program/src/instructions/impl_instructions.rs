use crate::define_instruction;

use super::allow_mint::{AllowMintAccounts, AllowMintData};
use super::block_mint::{BlockMintAccounts, BlockMintData};
use super::create_escrow::{CreateEscrowAccounts, CreateEscrowData};
use super::deposit::{DepositAccounts, DepositData};
use super::extensions::{
    add_timelock::{AddTimelockAccounts, AddTimelockData},
    block_token_extension::{BlockTokenExtensionAccounts, BlockTokenExtensionData},
    remove_extension::{RemoveExtensionAccounts, RemoveExtensionData},
    set_arbiter::{SetArbiterAccounts, SetArbiterData},
    set_hook::{SetHookAccounts, SetHookData},
    unblock_token_extension::{UnblockTokenExtensionAccounts, UnblockTokenExtensionData},
};
use super::set_immutable::{SetImmutableAccounts, SetImmutableData};
use super::update_admin::{UpdateAdminAccounts, UpdateAdminData};
use super::withdraw::{WithdrawAccounts, WithdrawData};

define_instruction!(AllowMint, AllowMintAccounts, AllowMintData);
define_instruction!(BlockMint, BlockMintAccounts, BlockMintData);
define_instruction!(CreateEscrow, CreateEscrowAccounts, CreateEscrowData);
define_instruction!(Deposit, DepositAccounts, DepositData);
define_instruction!(AddTimelock, AddTimelockAccounts, AddTimelockData);
define_instruction!(BlockTokenExtension, BlockTokenExtensionAccounts, BlockTokenExtensionData);
define_instruction!(RemoveExtension, RemoveExtensionAccounts, RemoveExtensionData);
define_instruction!(SetArbiter, SetArbiterAccounts, SetArbiterData);
define_instruction!(SetHook, SetHookAccounts, SetHookData);
define_instruction!(UnblockTokenExtension, UnblockTokenExtensionAccounts, UnblockTokenExtensionData);
define_instruction!(SetImmutable, SetImmutableAccounts, SetImmutableData);
define_instruction!(UpdateAdmin, UpdateAdminAccounts, UpdateAdminData);
define_instruction!(Withdraw, WithdrawAccounts, WithdrawData);
