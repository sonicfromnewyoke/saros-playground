use core::cell::RefCell;
use mollusk_svm::Mollusk;
use solana_account::Account;
use solana_log_collector::LogCollector;
use solana_program_pack::Pack;
use solana_pubkey::Pubkey;
use solana_rent::Rent;
use spl_token_2022::state::{Account as TokenAccount, AccountState, Mint};
use std::rc::Rc;

pub(crate) fn mollusk() -> Mollusk {
    let logger = Rc::new(RefCell::new(LogCollector::default()));
    let mut mollusk = Mollusk {
        logger: Some(logger.clone()),
        ..Default::default()
    };

    mollusk_svm_programs_token::associated_token::add_program(&mut mollusk);
    mollusk_svm_programs_token::token::add_program(&mut mollusk);
    mollusk_svm_programs_token::token2022::add_program(&mut mollusk);
    mollusk_svm_programs_memo::memo::add_program(&mut mollusk);
    saros_programs::amm::add_program(&mut mollusk);
    saros_programs::dlmm::add_program(&mut mollusk);

    mollusk
}

pub(crate) fn mint_account(
    mint_authority: Option<&Pubkey>,
    freeze_authority: Option<&Pubkey>,
    supply: u64,
    decimals: u8,
) -> Account {
    let data = {
        let mut data = vec![0; Mint::LEN];
        let state = Mint {
            mint_authority: mint_authority.copied().into(),
            supply,
            decimals,
            is_initialized: true,
            freeze_authority: freeze_authority.copied().into(),
        };
        state.pack_into_slice(&mut data);
        data
    };

    let space = data.len();
    let lamports = Rent::default().minimum_balance(space);

    Account {
        lamports,
        data,
        owner: spl_token_interface::program::ID.into(),
        ..Default::default()
    }
}

pub(crate) fn wsol_mint_account() -> Account {
    mint_account(None, None, 1_000_000_000_000_000, 9)
}

pub(crate) fn system_account_with_lamports(lamports: u64) -> Account {
    Account::new(lamports, 0, &solana_sdk_ids::system_program::id())
}

pub(crate) fn token_account(mint: &Pubkey, owner: &Pubkey, amount: u64) -> Account {
    let data = {
        let mut data = vec![0; TokenAccount::LEN];
        let state = TokenAccount {
            mint: *mint,
            owner: *owner,
            amount,
            delegate: None.into(),
            state: AccountState::Initialized,
            is_native: None.into(),
            delegated_amount: 0,
            close_authority: None.into(),
        };
        state.pack_into_slice(&mut data);
        data
    };

    let space = data.len();
    let lamports = Rent::default().minimum_balance(space);

    Account {
        lamports,
        data,
        owner: spl_token_interface::program::ID.into(),
        ..Default::default()
    }
}

pub(crate) fn wsol_token_account(owner: &Pubkey, amount: u64) -> Account {
    let rent = Rent::default().minimum_balance(165);
    let data = {
        let mut data = vec![0; TokenAccount::LEN];
        let state = TokenAccount {
            mint: spl_token_interface::native_mint::ID.into(),
            owner: *owner,
            amount,
            delegate: None.into(),
            state: AccountState::Initialized,
            is_native: Some(rent).into(),
            delegated_amount: 0,
            close_authority: None.into(),
        };
        state.pack_into_slice(&mut data);
        data
    };

    let lamports = rent + amount;

    Account {
        lamports,
        data,
        owner: spl_token_interface::program::ID.into(),
        ..Default::default()
    }
}

pub(crate) fn account_from_data_and_owner(data: Vec<u8>, owner: Pubkey) -> Account {
    let space = data.len();
    let lamports = Rent::default().minimum_balance(space);

    Account {
        lamports,
        data,
        owner,
        ..Default::default()
    }
}
