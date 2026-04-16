//! ComputeBudget 程序指令解析。
//!
//! 提供 [`SetComputUnitPrice`] 和 [`SetComputUnitLimit`] 两种指令的解析支持。

use solana_sdk::borsh1;
use solana_tx_parser::instruction;

/// 设置交易的 Compute Unit 价格（micro-lamports/CU）。
///
/// 价格越高，交易被打包优先级越高。
instruction!(
    program_id: "ComputeBudget111111111111111111111111111111",
    name: SetComputUnitPrice,
    discriminator: [3],
    accounts: {},
    data: {
        micro_lamports:u64,
    },
);

/// 设置交易可使用的最大 Compute Unit 数（上限为 1,400,000）。
instruction!(
    program_id: "ComputeBudget111111111111111111111111111111",
    name: SetComputUnitLimit,
    discriminator: [2],
    accounts: {},
    data: {
        units:u32,
    },
);
