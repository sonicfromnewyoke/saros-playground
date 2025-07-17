use solana_pubkey::Pubkey;
use spl_token_swap::{
    curve::{
        base::{CurveType::ConstantProduct, SwapCurve},
        constant_product::ConstantProductCurve,
        fees::Fees,
    },
    state::SwapVersion,
};
use std::sync::Arc;

pub fn make_constant_product_pool(
    bump_seed: u8,
    token_a_mint: Pubkey,
    token_b_mint: Pubkey,
    token_a: Pubkey,
    token_b: Pubkey,
    pool_mint: Pubkey,
    pool_fee_account: Pubkey,
) -> SwapVersion {
    SwapVersion::SwapV1(spl_token_swap::state::SwapV1 {
        is_initialized: true,
        bump_seed,
        token_program_id: spl_token_interface::program::ID.into(),
        token_a,
        token_b,
        pool_mint,
        token_a_mint,
        token_b_mint,
        pool_fee_account,
        fees: Fees {
            trade_fee_numerator: 25,
            trade_fee_denominator: 10_000,
            owner_trade_fee_numerator: 5,
            owner_trade_fee_denominator: 10_000,
            owner_withdraw_fee_numerator: 0,
            owner_withdraw_fee_denominator: 0,
            host_fee_numerator: 20,
            host_fee_denominator: 100,
        },
        swap_curve: SwapCurve {
            curve_type: ConstantProduct,
            calculator: Arc::new(ConstantProductCurve),
        },
    })
}
