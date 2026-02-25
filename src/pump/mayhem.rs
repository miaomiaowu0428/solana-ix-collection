use std::sync::LazyLock;

use solana_sdk::{pubkey, pubkey::Pubkey};
use spl_associated_token_account::get_associated_token_address_with_program_id;

pub static FEE_RECV: Pubkey = pubkey!("GesfTA3X2arioaHp8bbKdjG9vJtskViWACZoYvxp4twS");

pub static PROGRAM: Pubkey = pubkey!("MAyhSmzXzV1pTf7LsNkrNwkWKTo4ougAJ1PPg47MD4e");

pub static GLOBAL_PRARAMS: Pubkey = pubkey!("13ec7XdrjF3h3YcqBTFDSReRcUFwbCnJaAQspM4j6DDJ");

pub static SOL_VAULT: Pubkey = pubkey!("BwWK17cbHxwWBKZkUYvzxLcNQ1YVyaFezduWbtm2de6s");

pub const TOKEN_2022_PROGRAM_ID: Pubkey = pubkey!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");
pub fn token_vault_of(mint: &Pubkey) -> Pubkey {
    get_associated_token_address_with_program_id(&SOL_VAULT, mint, &TOKEN_2022_PROGRAM_ID)
}

pub static MAYHEM_FEE_RECV: LazyLock<Vec<Pubkey>> = LazyLock::new(|| {
    vec![
        pubkey!("GesfTA3X2arioaHp8bbKdjG9vJtskViWACZoYvxp4twS"),
        pubkey!("4budycTjhs9fD6xw62VBducVTNgMgJJ5BgtKq7mAZwn6"),
        pubkey!("8SBKzEQU4nLSzcwF4a74F2iaUDQyTfjGndn6qUWBnrpR"),
        pubkey!("4UQeTP1T39KZ9Sfxzo3WR5skgsaP6NZa87BAkuazLEKH"),
        pubkey!("8sNeir4QsLsJdYpc9RZacohhK1Y5FLU3nC5LXgYB4aa6"),
        pubkey!("Fh9HmeLNUMVCvejxCtCL2DbYaRyBFVJ5xrWkLnMH6fdk"),
        pubkey!("463MEnMeGyJekNZFQSTUABBEbLnvMTALbT6ZmsxAbAdq"),
        pubkey!("6AUH3WEHucYZyC61hqpqYUWVto5qA5hjHuNQ32GNnNxA"),
    ]
});
