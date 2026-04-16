//! System Program 原生 SOL 转账指令解析。

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::borsh1;
use solana_tx_parser::instruction;
/// System Program SOL 转账指令（discriminator `[0x02, 0x00, 0x00, 0x00]`）。
///
/// `lamports` 为转帐数量（单位 lamports）。
instruction!(
    program_id: "11111111111111111111111111111111",
    name: Transfer,
    discriminator: [0x02,0x00,0x00,0x00],
    accounts: {
        from: {
            writable: true,
            signer: true
        },
        to: {
            writable: true,
            signer: false
        }
    },
    data: {
        lamports: u64,
    },
);
