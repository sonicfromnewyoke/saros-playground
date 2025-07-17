use mollusk_svm::result::Check;
use solana_keypair::Keypair;
use solana_signer::Signer;
use spl_associated_token_account_client::address::get_associated_token_address_with_program_id;
use spl_token_swap::state::SwapVersion;

use crate::{keys, setup};

pub(crate) fn swap() {
    // Arrange
    let mollusk = setup::mollusk();

    let payer = Keypair::new();

    let payer_sol_ata = get_associated_token_address_with_program_id(
        &payer.pubkey(),
        &spl_token_interface::native_mint::ID.into(),
        &spl_token_interface::program::ID.into(),
    );
    let payer_saros_ata = get_associated_token_address_with_program_id(
        &payer.pubkey(),
        &keys::SAROS_MINT,
        &spl_token_interface::program::ID.into(),
    );

    let initial_payer_sol_balance = 10_000_000_000; // 10 SOL
    let final_payer_sol_balance = 9_000_000_000u64; // 9 SOL
    let final_payer_saros_balance = 8_884_422_309u64; // 8.884k
    let amount_in = 1_000_000_000; // 1 SOL
    let amount_out = 0; // who cares about safety?

    let payer_sol_token_account =
        setup::wsol_token_account(&payer.pubkey(), initial_payer_sol_balance);
    let payer_saros_token_account = setup::token_account(&keys::SAROS_MINT, &payer.pubkey(), 0);

    let mut market_data = vec![0; SwapVersion::LATEST_LEN];
    SwapVersion::pack(
        saros_amm_sdk::state::make_constant_product_pool(
            255, // bump seed
            spl_token_interface::native_mint::ID.into(),
            keys::SAROS_MINT,
            keys::SAROS_AMM_POOL_1,
            keys::SAROS_AMM_POOL_2,
            keys::SAROS_AMM_POOL_MINT,
            keys::SAROS_AMM_POOL_FEE,
        ),
        &mut market_data,
    )
    .expect("Failed to pack market data");

    // Act
    // swap 1 SOL -> Saros
    let ix = saros_amm_sdk::instructions::Swap {
        pool: keys::SAROS_AMM_MARKET,
        authority: keys::SAROS_AMM_POOL_AUTHORITY,
        payer: payer.pubkey(),
        source: payer_sol_ata,
        swap_source: keys::SAROS_AMM_POOL_1,
        swap_destination: keys::SAROS_AMM_POOL_2,
        destination: payer_saros_ata,
        mint: keys::SAROS_AMM_POOL_MINT,
        fees: keys::SAROS_AMM_POOL_FEE,
        token_program: spl_token_interface::program::ID.into(),
        amount_in,
        amount_out,
    }
    .build();

    let accounts = vec![
        // mints
        (
            spl_token_interface::native_mint::ID.into(),
            setup::wsol_mint_account(),
        ),
        (
            keys::SAROS_MINT,
            setup::mint_account(
                Some(&keys::SAROS_MINT_AUTHORITY),
                None,
                1_000_000_000_000_000,
                6,
            ),
        ),
        (
            keys::SAROS_AMM_POOL_MINT,
            setup::mint_account(
                Some(&keys::SAROS_AMM_POOL_AUTHORITY),
                None,
                1_000_000_000_000_000,
                6,
            ),
        ),
        // programs
        mollusk_svm_programs_token::token::keyed_account(),
        saros_programs::amm::keyed_account(),
        // payer
        (
            payer.pubkey(),
            setup::system_account_with_lamports(5_000_000),
        ),
        (payer_sol_ata, payer_sol_token_account),
        (payer_saros_ata, payer_saros_token_account),
        // saros market
        (
            keys::SAROS_AMM_MARKET,
            setup::account_from_data_and_owner(market_data, saros_programs::amm::ID),
        ),
        (
            keys::SAROS_AMM_POOL_AUTHORITY,
            setup::system_account_with_lamports(100_000),
        ),
        (
            keys::SAROS_AMM_POOL_1,
            setup::wsol_token_account(&keys::SAROS_AMM_POOL_AUTHORITY, 100_000_000_000),
        ),
        (
            keys::SAROS_AMM_POOL_2,
            setup::token_account(
                &keys::SAROS_MINT,
                &keys::SAROS_AMM_POOL_AUTHORITY,
                900_000_000_000,
            ),
        ),
        (
            keys::SAROS_AMM_POOL_FEE,
            setup::token_account(
                &keys::SAROS_AMM_POOL_MINT,
                &keys::SAROS_AMM_POOL_FEE_OWNER,
                5_000_000_000_000,
            ),
        ),
    ];

    // Assert
    mollusk.process_and_validate_instruction(
        &ix,
        &accounts,
        &[
            Check::success(),
            Check::account(&payer_sol_ata)
                .data_slice(64, final_payer_sol_balance.to_le_bytes().as_ref())
                .build(),
            Check::account(&payer_saros_ata)
                .data_slice(64, final_payer_saros_balance.to_le_bytes().as_ref())
                .build(),
        ],
    );
}
