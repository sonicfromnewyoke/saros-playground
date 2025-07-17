use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum SwapType {
    ExactInput,
    ExactOutput,
}

pub struct Swap {
    pub pair: Pubkey,
    pub token_mint_x: Pubkey,
    pub token_mint_y: Pubkey,
    pub bin_array_lower: Pubkey,
    pub bin_array_upper: Pubkey,
    pub token_vault_x: Pubkey,
    pub token_vault_y: Pubkey,
    pub user_vault_x: Pubkey,
    pub user_vault_y: Pubkey,
    pub user: Pubkey,
    pub token_program_x: Pubkey,
    pub token_program_y: Pubkey,
    pub memo_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,

    pub amount: u64,
    pub other_amount_threshold: u64,
    pub swap_for_y: bool,
    pub swap_type: SwapType,
}

impl Swap {
    pub fn build(&self) -> Instruction {
        let mut data = Vec::with_capacity(26);

        data.extend_from_slice(&[248, 198, 158, 145, 225, 117, 135, 200]);

        data.extend_from_slice(&self.amount.to_le_bytes());
        data.extend_from_slice(&self.other_amount_threshold.to_le_bytes());
        data.extend_from_slice(&[self.swap_for_y as u8]);
        data.extend_from_slice(&[self.swap_type as u8]);

        Instruction {
            program_id: saros_programs::dlmm::ID,
            accounts: vec![
                AccountMeta::new(self.pair, false),
                AccountMeta::new_readonly(self.token_mint_x, false),
                AccountMeta::new_readonly(self.token_mint_y, false),
                AccountMeta::new(self.bin_array_lower, false),
                AccountMeta::new(self.bin_array_upper, false),
                AccountMeta::new(self.token_vault_x, false),
                AccountMeta::new(self.token_vault_y, false),
                AccountMeta::new(self.user_vault_x, false),
                AccountMeta::new(self.user_vault_y, false),
                AccountMeta::new_readonly(self.user, true),
                AccountMeta::new_readonly(self.token_program_x, false),
                AccountMeta::new_readonly(self.token_program_y, false),
                AccountMeta::new_readonly(self.memo_program, false),
                AccountMeta::new_readonly(self.event_authority, false),
                AccountMeta::new_readonly(self.program, false),
            ],
            data,
        }
    }
}
