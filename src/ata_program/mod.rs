//! SPL Associated Token Account 程序指令解析。
//!
//! # discriminator 是 1 字节（不是 Anchor 的 8 字节）
//!
//! ATA 程序的账户全部通过 `accounts` 列表传入，`data` 里**只有 1 个字节**
//! 表示操作类型。实测（见 `pumpswap-snipe` 的 `pump::test::ata_discriminator`，
//! 用 5 个不同随机 payer 构造并比对 data）：
//!
//! | 指令 | data |
//! |---|---|
//! | [`Create`] | `[0]` |
//! | [`CreateIdempotent`] | `[1]` |
//!
//! 因为 data 只有 1 字节，**没有任何字段可解析** —— 两种指令的区别只在
//! 「已存在时是否报错」，且都可能带/不带 `token_program` 账户。
//!
//! # ⚠️ 账户列表
//!
//! `instruction!` 宏的 `accounts:` 只用于**校验账户数量**，字段名仅供可读性。
//! 这里两个指令的账户数都可能变化（见下），所以**不声明 `accounts:`**，
//! 只靠 `program_id` + `discriminator` 匹配。
//!
//! Solana 实际支持 3 种账户布局：
//!
//! - `[payer, ata, owner, mint]` —— 用默认 Token 程序（4 个）
//! - `[payer, ata, owner, mint, system_program]` —— 5 个
//! - `[payer, ata, owner, mint, system_program, token_program]` —— 6 个
//!
//! 本模块**不限制账户数**，只要能按 disc 匹配上即可。

use solana_sdk::borsh1;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_tx_parser::instruction;
use utils::{IndexedInstruction, impl_enum_getters};

/// `create`：创建 ATA。**若已存在则报错**（disc `[0]`）。
instruction!(
    program_id: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
    name: Create,
    discriminator: [0],
    accounts: {},
    data: {},
);

/// `create_idempotent`：创建 ATA，**若已存在则静默成功**（disc `[1]`）。
instruction!(
    program_id: "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL",
    name: CreateIdempotent,
    discriminator: [1],
    accounts: {},
    data: {},
);
