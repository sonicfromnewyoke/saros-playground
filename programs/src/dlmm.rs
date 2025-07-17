use {mollusk_svm::Mollusk, solana_account::Account, solana_pubkey::Pubkey};

pub const ID: Pubkey = solana_pubkey::pubkey!("1qbkdrr3z4ryLA7pZykqxvxWPoeifcVKo6ZG9CfkvVE");

pub const ELF: &[u8] = include_bytes!("elf/dlmm.so");

pub fn add_program(mollusk: &mut Mollusk) {
    // Loader v3
    mollusk.add_program_with_elf_and_loader(
        &ID,
        ELF,
        &mollusk_svm::program::loader_keys::LOADER_V3,
    );
}

pub fn account() -> Account {
    // Loader v3
    mollusk_svm::program::create_program_account_loader_v3(&ID)
}

/// Get the key and account for the Saros DLMM program.
pub fn keyed_account() -> (Pubkey, Account) {
    (ID, account())
}
