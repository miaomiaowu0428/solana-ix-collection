//! Stake Program 指令解析。
//!
//! 提供 [`WithdrawIx`]（Stake 提炸）和 [`StakeInitializeIx`]（权益初始化）的解析支持。

use std::{fmt::Display, io::Read};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{borsh1, pubkey::Pubkey};
use solana_tx_parser::instruction;
use utils::IndexedInstruction;

/// Stake Program 提炸指令（discriminator `[4, 0, 0, 0]`）。
///
/// 将 `amount` lamports 从 `stake_account` 提炸到 `destination`。
instruction! {
    program_id: "Stake11111111111111111111111111111111111111",
    discriminator: [4,0,0,0],
    name: WithdrawIx,
    accounts:{
        stake_account     :{ writable: false, signer: false },
        destination       :{ writable: false, signer: false },
        sys_clock         :{ writable: false, signer: false },
        sys_history       :{ writable: false, signer: false },
        authority         :{ writable: false, signer: false },
    },
    data:{
        amount: u64,
    }
}

/// Stake Program 权益初始化指令（discriminator `[0, 0, 0, 0]`）。
///
/// 初始化 stake 账户的 staker / withdrawer 权益，
/// `data` 字段均使用 [`InitData`] 原样存储不做解析。
instruction! {
    program_id: "Stake11111111111111111111111111111111111111",
    discriminator: [0, 0, 0, 0],
    name: StakeInitializeIx,
    accounts: {
        stake_account : { writable: true,  signer: false },
        rent_sysvar   : { writable: false, signer: false },
    },
    data: {
        data: InitData, // 👈 不解析，交给你自己
    }
}

/// Stake 初始化指令的数据内容，包括 staker/withdrawer 公钥及锁定信息。
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct InitData {
    authorized: Authorized,
    lockup: Lockup,
}
/// Stake 授权信息，包含 staker 和 withdrawer 公钥。
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct Authorized {
    pub staker: Pubkey,     // 32 bytes
    pub withdrawer: Pubkey, // 32 bytes
}
/// Stake 锁定配置信息。
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct Lockup {
    pub unix_timestamp: i64, // 8 bytes
    pub epoch: u64,          // 8 bytes
    pub custodian: Pubkey,   // 32 bytes
}
