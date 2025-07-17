use {mollusk_svm::Mollusk, solana_account::Account, solana_pubkey::Pubkey};

pub const ID: Pubkey = solana_pubkey::pubkey!("SSwapUtytfBdBn1b9NUGG6foMVPtcWgpRU32HToDUZr");

pub const ELF: &[u8] = include_bytes!("elf/amm.so");

pub fn add_program(mollusk: &mut Mollusk) {
    // Loader v3
    mollusk.add_program_with_elf_and_loader(
        &ID,
        ELF,
        &mollusk_svm::program::loader_keys::LOADER_V3,
    );
}

fn account() -> Account {
    // Loader v3
    mollusk_svm::program::create_program_account_loader_v3(&ID)
}

/// Get the key and account for the Saros AMM program.
pub fn keyed_account() -> (Pubkey, Account) {
    (ID, account())
}
