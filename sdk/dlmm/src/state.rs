use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

use solana_program_error::ProgramError;
use solana_program_pack::{IsInitialized, Pack, Sealed};
use solana_pubkey::Pubkey;

const BIN_ARRAY_SIZE_USIZE: usize = 256;

const BIN_ARRAY_DISCRIMINATOR: [u8; 8] = [92, 142, 92, 220, 5, 148, 70, 181];
const PAIR_DISCRIMINATOR: [u8; 8] = [85, 72, 49, 176, 182, 228, 141, 82];

#[derive(Debug, Clone)]
pub struct DynamicFeeParameters {
    pub time_last_updated: u64,
    pub volatility_accumulator: u32,
    pub volatility_reference: u32,
    pub id_reference: u32,
    _space: [u8; 4],
}

impl IsInitialized for DynamicFeeParameters {
    fn is_initialized(&self) -> bool {
        true
    }
}
impl Sealed for DynamicFeeParameters {}
impl Pack for DynamicFeeParameters {
    const LEN: usize = 24;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, DynamicFeeParameters::LEN];
        let (time_last_updated, volatility_accumulator, volatility_reference, id_reference, _space) =
            mut_array_refs![output, 8, 4, 4, 4, 4];
        time_last_updated.copy_from_slice(&self.time_last_updated.to_le_bytes());
        volatility_accumulator.copy_from_slice(&self.volatility_accumulator.to_le_bytes());
        volatility_reference.copy_from_slice(&self.volatility_reference.to_le_bytes());
        id_reference.copy_from_slice(&self.id_reference.to_le_bytes());
        _space.copy_from_slice(&self._space);
    }

    fn unpack_from_slice(input: &[u8]) -> Result<DynamicFeeParameters, ProgramError> {
        let input = array_ref![input, 0, DynamicFeeParameters::LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (time_last_updated, volatility_accumulator, volatility_reference, id_reference, _space) =
            array_refs![input, 8, 4, 4, 4, 4];

        Ok(Self {
            time_last_updated: u64::from_le_bytes(*time_last_updated),
            volatility_accumulator: u32::from_le_bytes(*volatility_accumulator),
            volatility_reference: u32::from_le_bytes(*volatility_reference),
            id_reference: u32::from_le_bytes(*id_reference),
            _space: *_space,
        })
    }
}

#[derive(Debug, Clone)]
pub struct StaticFeeParameters {
    pub base_factor: u16,
    pub filter_period: u16,
    pub decay_period: u16,
    pub reduction_factor: u16,
    pub variable_fee_control: u32,
    pub max_volatility_accumulator: u32,
    pub protocol_share: u16,
    _space: [u8; 2],
}

impl Sealed for StaticFeeParameters {}
impl Pack for StaticFeeParameters {
    const LEN: usize = 20;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, StaticFeeParameters::LEN];
        let (
            base_factor,
            filter_period,
            decay_period,
            reduction_factor,
            variable_fee_control,
            max_volatility_accumulator,
            protocol_share,
            _space,
        ) = mut_array_refs![output, 2, 2, 2, 2, 4, 4, 2, 2];

        base_factor.copy_from_slice(&self.base_factor.to_le_bytes());
        filter_period.copy_from_slice(&self.filter_period.to_le_bytes());
        decay_period.copy_from_slice(&self.decay_period.to_le_bytes());
        reduction_factor.copy_from_slice(&self.reduction_factor.to_le_bytes());
        variable_fee_control.copy_from_slice(&self.variable_fee_control.to_le_bytes());
        max_volatility_accumulator.copy_from_slice(&self.max_volatility_accumulator.to_le_bytes());
        protocol_share.copy_from_slice(&self.protocol_share.to_le_bytes());
        _space.copy_from_slice(&self._space);
    }

    fn unpack_from_slice(input: &[u8]) -> Result<StaticFeeParameters, ProgramError> {
        let input = array_ref![input, 0, StaticFeeParameters::LEN];

        #[allow(clippy::ptr_offset_with_cast)]
        let (
            base_factor,
            filter_period,
            decay_period,
            reduction_factor,
            variable_fee_control,
            max_volatility_accumulator,
            protocol_share,
            _space,
        ) = array_refs![input, 2, 2, 2, 2, 4, 4, 2, 2];

        Ok(Self {
            base_factor: u16::from_le_bytes(*base_factor),
            filter_period: u16::from_le_bytes(*filter_period),
            decay_period: u16::from_le_bytes(*decay_period),
            reduction_factor: u16::from_le_bytes(*reduction_factor),
            variable_fee_control: u32::from_le_bytes(*variable_fee_control),
            max_volatility_accumulator: u32::from_le_bytes(*max_volatility_accumulator),
            protocol_share: u16::from_le_bytes(*protocol_share),
            _space: *_space,
        })
    }
}

pub struct Pair {
    _discriminator: [u8; 8],
    pub bump: [u8; 1],
    pub liquidity_book_config: Pubkey,
    pub bin_step: u8,
    pub bin_step_seed: [u8; 1],
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub static_fee_parameters: StaticFeeParameters,
    pub active_id: u32,
    pub dynamic_fee_parameters: DynamicFeeParameters,
    pub protocol_fees_x: u64,
    pub protocol_fees_y: u64,
    pub hook: Option<Pubkey>,
}

impl IsInitialized for Pair {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for Pair {}

impl Pack for Pair {
    const LEN: usize = 204;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, Pair::LEN];

        let (
            discriminator,
            bump,
            liquidity_book_config,
            bin_step,
            bin_step_seed,
            token_mint_x,
            token_mint_y,
            static_fee_parameters,
            active_id,
            dynamic_fee_parameters,
            protocol_fees_x,
            protocol_fees_y,
            hook_flag_dst,
            hook_pubkey_dst,
        ) = mut_array_refs![output, 8, 1, 32, 1, 1, 32, 32, 20, 4, 24, 8, 8, 1, 32];

        discriminator.copy_from_slice(&PAIR_DISCRIMINATOR);
        bump.copy_from_slice(&self.bump);
        liquidity_book_config.copy_from_slice(self.liquidity_book_config.as_ref());
        bin_step.copy_from_slice(&self.bin_step.to_le_bytes());
        bin_step_seed.copy_from_slice(&self.bin_step_seed);
        token_mint_x.copy_from_slice(self.token_mint_x.as_ref());
        token_mint_y.copy_from_slice(self.token_mint_y.as_ref());
        self.static_fee_parameters
            .pack_into_slice(&mut static_fee_parameters[..]);
        active_id.copy_from_slice(&self.active_id.to_le_bytes());
        self.dynamic_fee_parameters
            .pack_into_slice(&mut dynamic_fee_parameters[..]);
        protocol_fees_x.copy_from_slice(&self.protocol_fees_x.to_le_bytes());
        protocol_fees_y.copy_from_slice(&self.protocol_fees_y.to_le_bytes());

        match &self.hook {
            Some(hook) => {
                hook_flag_dst[0] = 1;
                hook_pubkey_dst.copy_from_slice(hook.as_ref());
            }
            None => {
                hook_flag_dst[0] = 0;
                hook_pubkey_dst.fill(0);
            }
        }
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, Pair::LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (
            discriminator,
            bump,
            liquidity_book_config,
            bin_step,
            bin_step_seed,
            token_mint_x,
            token_mint_y,
            static_fee_parameters,
            active_id,
            dynamic_fee_parameters,
            protocol_fees_x,
            protocol_fees_y,
            hook_flag_dst,
            hook_pubkey_dst,
        ) = array_refs![input, 8, 1, 32, 1, 1, 32, 32, 20, 4, 24, 8, 8, 1, 32];

        Ok(Self {
            _discriminator: *discriminator,
            bump: *bump,
            liquidity_book_config: Pubkey::new_from_array(*liquidity_book_config),
            bin_step: u8::from_le_bytes(*bin_step),
            bin_step_seed: *bin_step_seed,
            token_mint_x: Pubkey::new_from_array(*token_mint_x),
            token_mint_y: Pubkey::new_from_array(*token_mint_y),
            static_fee_parameters: StaticFeeParameters::unpack_from_slice(static_fee_parameters)?,
            active_id: u32::from_le_bytes(*active_id),
            dynamic_fee_parameters: DynamicFeeParameters::unpack_from_slice(
                dynamic_fee_parameters,
            )?,
            protocol_fees_x: u64::from_le_bytes(*protocol_fees_x),
            protocol_fees_y: u64::from_le_bytes(*protocol_fees_y),
            hook: match hook_flag_dst {
                [0] => None,
                [1] => Some(Pubkey::new_from_array(*hook_pubkey_dst)),
                _ => return Err(ProgramError::InvalidAccountData),
            },
        })
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Bin {
    pub total_supply: u128,
    pub reserve_x: u64,
    pub reserve_y: u64,
}

impl IsInitialized for Bin {
    fn is_initialized(&self) -> bool {
        true
    }
}
impl Sealed for Bin {}

impl Pack for Bin {
    const LEN: usize = 16 + 8 + 8;
    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, Bin::LEN];
        let (total_supply_dst, reserve_x_dst, reserve_y_dst) = mut_array_refs![output, 16, 8, 8];

        total_supply_dst.copy_from_slice(&self.total_supply.to_le_bytes());
        reserve_x_dst.copy_from_slice(&self.reserve_x.to_le_bytes());
        reserve_y_dst.copy_from_slice(&self.reserve_y.to_le_bytes());
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, Bin::LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (total_supply_src, reserve_x_src, reserve_y_src) = array_refs![input, 16, 8, 8];
        Ok(Self {
            total_supply: u128::from_le_bytes(*total_supply_src),
            reserve_x: u64::from_le_bytes(*reserve_x_src),
            reserve_y: u64::from_le_bytes(*reserve_y_src),
        })
    }
}

#[derive(Debug, Clone)]
pub struct BinArray {
    _discriminator: [u8; 8],
    pub pair: Pubkey,
    pub bins: [Bin; 256],
    pub index: u32,
    _space: [u8; 12],
}

impl IsInitialized for BinArray {
    fn is_initialized(&self) -> bool {
        true
    }
}

impl Sealed for BinArray {}

impl Pack for BinArray {
    const LEN: usize = 8 + 32 + BIN_ARRAY_SIZE_USIZE * 32 + 4 + 12;

    fn pack_into_slice(&self, output: &mut [u8]) {
        let output = array_mut_ref![output, 0, BinArray::LEN];
        let (discriminator_dst, pair_dst, bins_dst, index_dst, _space_dst) =
            mut_array_refs![output, 8, 32, 8192, 4, 12];

        discriminator_dst.copy_from_slice(&BIN_ARRAY_DISCRIMINATOR);
        pair_dst.copy_from_slice(self.pair.as_ref());
        for (i, bin) in self.bins.iter().enumerate() {
            bin.pack_into_slice(&mut bins_dst[i * Bin::LEN..]);
        }
        index_dst.copy_from_slice(&self.index.to_le_bytes());
    }

    fn unpack_from_slice(input: &[u8]) -> Result<Self, ProgramError> {
        let input = array_ref![input, 0, BinArray::LEN];
        #[allow(clippy::ptr_offset_with_cast)]
        let (discriminator_src, pair_src, bins_src, index_src, _space_src) =
            array_refs![input, 8, 32, BIN_ARRAY_SIZE_USIZE * 32, 4, 12];

        let mut bins = [Bin::default(); BIN_ARRAY_SIZE_USIZE];
        for (i, bin) in bins.iter_mut().enumerate() {
            *bin = Bin::unpack_from_slice(&bins_src[i * Bin::LEN..])?;
        }

        Ok(Self {
            _discriminator: *discriminator_src,
            pair: Pubkey::new_from_array(*pair_src),
            bins,
            index: u32::from_le_bytes(*index_src),
            _space: [0; 12],
        })
    }
}

pub fn make_bin_array(pair: Pubkey, index: u32) -> BinArray {
    BinArray {
        _discriminator: BIN_ARRAY_DISCRIMINATOR,
        pair,
        bins: [Bin {
            total_supply: 1_000_000,
            reserve_x: 20_000,
            reserve_y: 4_000,
        }; BIN_ARRAY_SIZE_USIZE], // 256 the same bins
        index,
        _space: [0; 12],
    }
}

pub fn make_pair(token_mint_x: Pubkey, token_mint_y: Pubkey, active_id: u32) -> Pair {
    Pair {
        _discriminator: PAIR_DISCRIMINATOR,
        bump: [255],
        liquidity_book_config: Pubkey::from_str_const(
            "BqPmjcPbAwE7mH23BY8q8VUEN4LSjhLUv41W87GsXVn8",
        ),
        bin_step: 10,
        bin_step_seed: [10],
        token_mint_x,
        token_mint_y,
        static_fee_parameters: StaticFeeParameters {
            base_factor: 10_000,
            filter_period: 120,
            decay_period: 600,
            reduction_factor: 5_000,
            variable_fee_control: 40_000,
            max_volatility_accumulator: 200_000,
            protocol_share: 2_000,
            _space: [0; 2],
        },
        active_id,
        dynamic_fee_parameters: DynamicFeeParameters {
            time_last_updated: 0, // bypass clock check,
            volatility_accumulator: 0,
            volatility_reference: 0,
            id_reference: active_id,
            _space: [0; 4],
        },
        protocol_fees_x: 70_000_000,
        protocol_fees_y: 150_000_000,
        hook: None,
    }
}
