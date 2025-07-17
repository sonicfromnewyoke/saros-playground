use solana_instruction::{AccountMeta, Instruction};
use solana_pubkey::Pubkey;

pub struct Swap {
    pub pool: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub source: Pubkey,
    pub swap_source: Pubkey,
    pub swap_destination: Pubkey,
    pub destination: Pubkey,
    pub mint: Pubkey,
    pub fees: Pubkey,
    pub token_program: Pubkey,

    pub amount_in: u64,
    pub amount_out: u64,
}

impl Swap {
    pub fn build(&self) -> Instruction {
        let mut data = Vec::with_capacity(17);

        data.push(1);
        data.extend_from_slice(&self.amount_in.to_le_bytes());
        data.extend_from_slice(&self.amount_out.to_le_bytes());

        Instruction {
            program_id: saros_programs::amm::ID,
            accounts: vec![
                AccountMeta::new_readonly(self.pool, false),
                AccountMeta::new_readonly(self.authority, false),
                AccountMeta::new_readonly(self.payer, true),
                AccountMeta::new(self.source, false),
                AccountMeta::new(self.swap_source, false),
                AccountMeta::new(self.swap_destination, false),
                AccountMeta::new(self.destination, false),
                AccountMeta::new(self.mint, false),
                AccountMeta::new(self.fees, false),
                AccountMeta::new_readonly(self.token_program, false),
            ],
            data,
        }
    }
}
