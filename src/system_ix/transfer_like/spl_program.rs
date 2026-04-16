//! SPL Token 程序转账类指令解析。
//!
//! 提供 [`SplTransferChecked`]、[`SplTransfer`]、[`SplCloseAccount`] 三种指令的解析支持。

use std::{fmt::Display, io::Read};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{borsh1, pubkey::Pubkey};
use solana_tx_parser::instruction;

/// SPL Token TransferChecked 指令（discriminator `[12]`）。
///
/// 需要指定 mint 并附带精度验证，比标准 [`SplTransfer`] 更安全。
instruction!(
    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    name: SplTransferChecked,
    discriminator: [12],
    accounts: {
        from: {
            writable: true,
            signer: true
        },
        mint: {
            writable: true,
            signer: false
        },
        to: {
            writable: true,
            signer: false
        },
        authority: {
            writable: false,
            signer: true
        },
    },
    data: {
        units: u64,
    },
);

/// SPL Token Transfer 指令（discriminator `[3]`）。
///
/// 无需指定 mint，差异与 [`SplTransferChecked`] 类似。
/// 此处仅解析 wSOL 交易中的转账指令。
instruction!(
    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    name: SplTransfer,
    discriminator: [3],
    accounts: {
        from: {
            writable: true,
            signer: true
        },
        to: {
            writable: true,
            signer: false
        },
        auth: {
            writable: true,
            signer: false
        },
    },
    data: {
        units: u64,
    },
);

/// SPL Token CloseAccount 指令（discriminator `[9]`）。
///
/// 关闭代币账户并将剩余 SOL 租金进行回收，此处用来检测 wSOL 幐账中的 SOL 转出。
instruction!(
    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    name: SplCloseAccount,
    discriminator: [9],
    accounts: {
        from: {
            writable: true,
            signer: true
        },
        to: {
            writable: true,
            signer: false
        },
        owner: {
            writable: true,
            signer: false
        },
    },
    data: {
    },
);
