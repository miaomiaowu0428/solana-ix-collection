//! Token-2022 程序指令解析。
//!
//! 目前提供 [`Transfer`] 指令的解析支持，程序地址为 Token-2022。

use solana_sdk::borsh1;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_tx_parser::instruction;
use utils::{IndexedInstruction, impl_enum_getters};

use crate::constants::TOKEN_2022_PROGRAM_ID;

/// Token-2022 Transfer 指令（discriminator `[3]`）。
///
/// `amount` 为转账数量（token 最小单位）。
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
