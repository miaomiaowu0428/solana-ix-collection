//! # solana-ix-collection
//!
//! 封装 Solana 链上常用指令的解析与构造，涵盖：
//!
//! - [`pump`]：Pump 协议（bonding curve）买/卖/创建/迁移指令及链上事件
//! - [`system_ix`]：系统程序（转账）、ComputeBudget 指令
//! - [`token_program`]：SPL Token 标准程序转账指令
//! - [`token_program_2022`]：Token-2022 程序转账指令
//! - [`constants`]：各程序 ID 及常用公钥常量

pub mod constants;
pub mod pump;
pub mod system_ix;
pub mod token_program;
pub mod token_program_2022;
