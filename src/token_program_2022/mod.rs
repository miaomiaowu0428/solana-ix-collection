use solana_sdk::borsh1;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_tx_parser::instruction;
use utils::{IndexedInstruction, impl_enum_getters};

use crate::constants::TOKEN_2022_PROGRAM_ID;

instruction!(
    program_id: "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb",
    name: Transfer,
    discriminator: [3],
    accounts: {
        source: { writable: false, signer: false },
        destination: { writable: true, signer: false },
        authority: { writable: false, signer: false },
    },
    data: {
        amount: u64,
    },
);
