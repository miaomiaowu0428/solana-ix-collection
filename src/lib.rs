//! # solana-ix-collection
//!
//! 封装 Solana 链上常用指令的解析与构造，涵盖：
//!
//! - [`system_ix`]：系统程序（转账）、ComputeBudget 指令
//! - [`token_program`]：SPL Token 标准程序转账指令
//! - [`token_program_2022`]：Token-2022 程序转账指令
//! - [`ata_program`]：SPL Associated Token Account 的 `create` / `create_idempotent`
//! - [`constants`]：各程序 ID 及常用公钥常量
//!
//! **Pump 协议相关模块已移除**（原 `pump` 模块）。需要 pump 能力的下游
//! 请自行引入对应依赖。

pub mod ata_program;
pub mod constants;
pub mod system_ix;
pub mod token_program;
pub mod token_program_2022;
