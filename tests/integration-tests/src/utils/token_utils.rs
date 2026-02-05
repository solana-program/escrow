use solana_program::program_option::COption;
use solana_program::program_pack::Pack;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use spl_associated_token_account::{get_associated_token_address, get_associated_token_address_with_program_id};
use spl_token_2022::{
    extension::{
        mint_close_authority::MintCloseAuthority, non_transferable::NonTransferable, pausable::PausableConfig,
        permanent_delegate::PermanentDelegate, transfer_fee::TransferFeeConfig, BaseStateWithExtensionsMut,
        ExtensionType, StateWithExtensionsMut,
    },
    state::Mint as Token2022Mint,
    ID as TOKEN_2022_PROGRAM_ID,
};
use spl_token_interface::state::{Account as TokenAccount, AccountState, Mint};
use spl_token_interface::ID as TOKEN_PROGRAM_ID;

use super::TestContext;

impl TestContext {
    pub fn create_mint(&mut self, mint: &Keypair, mint_authority: &Pubkey, decimals: u8) {
        let mint_state = Mint {
            mint_authority: COption::Some(*mint_authority),
            supply: 0,
            decimals,
            is_initialized: true,
            freeze_authority: COption::None,
        };

        let mut data = vec![0u8; Mint::LEN];
        mint_state.pack_into_slice(&mut data);

        self.svm
            .set_account(
                mint.pubkey(),
                Account {
                    lamports: self.svm.minimum_balance_for_rent_exemption(Mint::LEN),
                    data,
                    owner: TOKEN_PROGRAM_ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();
    }

    pub fn create_token_account(&mut self, owner: &Pubkey, mint: &Pubkey) -> Pubkey {
        self.create_token_account_with_balance(owner, mint, 0)
    }

    pub fn create_token_account_with_balance(&mut self, owner: &Pubkey, mint: &Pubkey, amount: u64) -> Pubkey {
        let ata = get_associated_token_address(owner, mint);

        let token_account = TokenAccount {
            mint: *mint,
            owner: *owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };

        let mut data = vec![0u8; TokenAccount::LEN];
        token_account.pack_into_slice(&mut data);

        self.svm
            .set_account(
                ata,
                Account {
                    lamports: self.svm.minimum_balance_for_rent_exemption(TokenAccount::LEN),
                    data,
                    owner: TOKEN_PROGRAM_ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();

        ata
    }

    pub fn set_token_balance(&mut self, token_account: &Pubkey, amount: u64) {
        let mut account = self.svm.get_account(token_account).expect("Token account not found");
        account.data[64..72].copy_from_slice(&amount.to_le_bytes());
        self.svm.set_account(*token_account, account).unwrap();
    }

    pub fn get_token_balance(&self, token_account: &Pubkey) -> u64 {
        let account = self.svm.get_account(token_account).expect("Token account not found");
        u64::from_le_bytes(account.data[64..72].try_into().unwrap())
    }

    pub fn create_token_2022_mint(&mut self, mint: &Keypair, mint_authority: &Pubkey, decimals: u8) {
        let mint_state = Mint {
            mint_authority: COption::Some(*mint_authority),
            supply: 0,
            decimals,
            is_initialized: true,
            freeze_authority: COption::None,
        };

        let mut data = vec![0u8; Mint::LEN];
        mint_state.pack_into_slice(&mut data);

        self.svm
            .set_account(
                mint.pubkey(),
                Account {
                    lamports: self.svm.minimum_balance_for_rent_exemption(Mint::LEN),
                    data,
                    owner: TOKEN_2022_PROGRAM_ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();
    }

    pub fn create_token_2022_account(&mut self, owner: &Pubkey, mint: &Pubkey) -> Pubkey {
        self.create_token_2022_account_with_balance(owner, mint, 0)
    }

    pub fn create_token_2022_account_with_balance(&mut self, owner: &Pubkey, mint: &Pubkey, amount: u64) -> Pubkey {
        let ata = get_associated_token_address_with_program_id(owner, mint, &TOKEN_2022_PROGRAM_ID);

        let token_account = TokenAccount {
            mint: *mint,
            owner: *owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };

        let mut data = vec![0u8; TokenAccount::LEN];
        token_account.pack_into_slice(&mut data);

        self.svm
            .set_account(
                ata,
                Account {
                    lamports: self.svm.minimum_balance_for_rent_exemption(TokenAccount::LEN),
                    data,
                    owner: TOKEN_2022_PROGRAM_ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();

        ata
    }

    pub fn create_token_2022_mint_with_extension(
        &mut self,
        mint: &Keypair,
        mint_authority: &Pubkey,
        decimals: u8,
        extension_type: ExtensionType,
    ) {
        let extensions = &[extension_type];
        let space = ExtensionType::try_calculate_account_len::<Token2022Mint>(extensions).unwrap();

        let mut data = vec![0u8; space];

        // Initialize the mint with extensions using the proper Token2022 initialization
        let mut state = StateWithExtensionsMut::<Token2022Mint>::unpack_uninitialized(&mut data).unwrap();

        // Initialize the base mint state first
        state.base.mint_authority = COption::Some(*mint_authority);
        state.base.supply = 0;
        state.base.decimals = decimals;
        state.base.is_initialized = true;
        state.base.freeze_authority = COption::None;

        // Pack the base state
        state.pack_base();

        // Set the account type discriminator for mints with extensions (byte 82)
        state.init_account_type().unwrap();

        // Now initialize the extension
        match extension_type {
            ExtensionType::PermanentDelegate => {
                state.init_extension::<PermanentDelegate>(true).unwrap();
            }
            ExtensionType::NonTransferable => {
                state.init_extension::<NonTransferable>(true).unwrap();
            }
            ExtensionType::Pausable => {
                state.init_extension::<PausableConfig>(true).unwrap();
            }
            ExtensionType::TransferFeeConfig => {
                state.init_extension::<TransferFeeConfig>(true).unwrap();
            }
            ExtensionType::MintCloseAuthority => {
                let ext = state.init_extension::<MintCloseAuthority>(true).unwrap();
                ext.close_authority = COption::Some(*mint_authority).try_into().unwrap();
            }
            _ => panic!("Unsupported extension type for test helper"),
        }

        self.svm
            .set_account(
                mint.pubkey(),
                Account {
                    lamports: self.svm.minimum_balance_for_rent_exemption(space),
                    data,
                    owner: TOKEN_2022_PROGRAM_ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();
    }

    /// Create a token account at a specific address (for wrong ATA tests).
    /// This allows testing ATA validation by creating a valid token account
    /// at an address that doesn't match the expected ATA derivation.
    pub fn create_token_account_at_address(&mut self, address: &Pubkey, owner: &Pubkey, mint: &Pubkey, amount: u64) {
        let token_account = TokenAccount {
            mint: *mint,
            owner: *owner,
            amount,
            delegate: COption::None,
            state: AccountState::Initialized,
            is_native: COption::None,
            delegated_amount: 0,
            close_authority: COption::None,
        };

        let mut data = vec![0u8; TokenAccount::LEN];
        token_account.pack_into_slice(&mut data);

        self.svm
            .set_account(
                *address,
                Account {
                    lamports: self.svm.minimum_balance_for_rent_exemption(TokenAccount::LEN),
                    data,
                    owner: TOKEN_PROGRAM_ID,
                    executable: false,
                    rent_epoch: 0,
                },
            )
            .unwrap();
    }
}
