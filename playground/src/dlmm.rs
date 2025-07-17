use mollusk_svm::result::Check;
use solana_keypair::Keypair;
use solana_program_pack::Pack;
use solana_signer::Signer;
use spl_associated_token_account_client::address::get_associated_token_address_with_program_id;

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
    let final_payer_sol_balance = 9_999_900_000u64; // 9.999 SOL
    let final_payer_saros_balance = 63_591u64;
    let amount_in = 100_000; // 0.01 SOL
    let amount_out = 0; // who cares about safety?

    let payer_sol_token_account =
        setup::wsol_token_account(&payer.pubkey(), initial_payer_sol_balance);
    let payer_saros_token_account =
        setup::token_account(&keys::SAROS_MINT, &payer.pubkey(), 0);

    let mut pair_data = vec![0; saros_dlmm_sdk::state::Pair::get_packed_len()];
    let mut bin_array_lower_data = vec![0; saros_dlmm_sdk::state::BinArray::get_packed_len()];
    let mut bin_array_upper_data = vec![0; saros_dlmm_sdk::state::BinArray::get_packed_len()];

    saros_dlmm_sdk::state::make_pair(
        spl_token_interface::native_mint::ID.into(),
        keys::SAROS_MINT,
        8_388_164,
    )
    .pack_into_slice(&mut pair_data);
    saros_dlmm_sdk::state::make_bin_array(keys::SAROS_DLMM_PAIR, 32_766)
        .pack_into_slice(&mut bin_array_lower_data);
    saros_dlmm_sdk::state::make_bin_array(keys::SAROS_DLMM_PAIR, 32_767)
        .pack_into_slice(&mut bin_array_upper_data);

    // Act
    // swap 1 SOL -> Saros
    let ix = saros_dlmm_sdk::instructions::Swap {
        pair: keys::SAROS_DLMM_PAIR,
        token_mint_x: spl_token_interface::native_mint::ID.into(),
        token_mint_y: keys::SAROS_MINT,
        bin_array_lower: keys::SAROS_DLMM_BIN_ARRAY_LOWER,
        bin_array_upper: keys::SAROS_DLMM_BIN_ARRAY_UPPER,
        token_vault_x: keys::SAROS_DLMM_TOKEN_VAULT_X,
        token_vault_y: keys::SAROS_DLMM_TOKEN_VAULT_Y,
        user_vault_x: payer_sol_ata,
        user_vault_y: payer_saros_ata,
        user: payer.pubkey(),
        token_program_x: spl_token_interface::program::ID.into(),
        token_program_y: spl_token_interface::program::ID.into(),
        memo_program: spl_memo_client::ID,
        event_authority: keys::SAROS_DLMM_EVENT_AUTHORITY,
        program: saros_programs::dlmm::ID,
        amount: amount_in,
        other_amount_threshold: amount_out,
        swap_for_y: true,
        swap_type: saros_dlmm_sdk::instructions::SwapType::ExactInput,
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
        // programs
        mollusk_svm_programs_token::token::keyed_account(),
        mollusk_svm_programs_memo::memo::keyed_account(),
        saros_programs::dlmm::keyed_account(),
        // payer
        (
            payer.pubkey(),
            setup::system_account_with_lamports(5_000_000),
        ),
        (payer_sol_ata, payer_sol_token_account),
        (payer_saros_ata, payer_saros_token_account),
        // saros market
        (
            keys::SAROS_DLMM_PAIR,
            setup::account_from_data_and_owner(pair_data, saros_programs::dlmm::ID),
        ),
        (
            keys::SAROS_DLMM_BIN_ARRAY_LOWER,
            setup::account_from_data_and_owner(bin_array_lower_data, saros_programs::dlmm::ID),
        ),
        (
            keys::SAROS_DLMM_BIN_ARRAY_UPPER,
            setup::account_from_data_and_owner(bin_array_upper_data, saros_programs::dlmm::ID),
        ),
        (
            keys::SAROS_DLMM_EVENT_AUTHORITY,
            setup::system_account_with_lamports(100_000),
        ),
        (
            keys::SAROS_DLMM_TOKEN_VAULT_X,
            setup::wsol_token_account(&keys::SAROS_DLMM_PAIR, 100_000_000_000),
        ),
        (
            keys::SAROS_DLMM_TOKEN_VAULT_Y,
            setup::token_account(
                &keys::SAROS_MINT,
                &keys::SAROS_DLMM_PAIR,
                900_000_000_000,
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
