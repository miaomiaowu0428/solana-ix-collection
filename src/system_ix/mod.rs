//! 系统级指令模块。
//!
//! - [`cu_budget`]：ComputeBudget 程序指令（设置 CU 上限 / 精气单价）
//! - [`transfer_like`]：封装各种转账类型指令的统一抽象

pub mod cu_budget;
pub mod transfer_like;
