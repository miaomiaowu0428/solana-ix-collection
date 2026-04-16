//! 转账类型指令的统一抽象。
//!
//! 将 SOL 原生转账、Stake 提炸、SPL Token 转账等各种转账语义统一表示
//! 为 [`TransferLike`]（from / to / amount），
//! 并通过 [`ParseTransfer`] trait 支持从常见交易格式中解析。

use grpc_client::TransactionFormat;
use serde_json::de;
use solana_sdk::{pubkey, pubkey::Pubkey};

use solana_transaction_status_client_types::EncodedConfirmedTransactionWithStatusMeta;
use transaction_cache::TxDetailLocal;
use utils::{
    IndexedInstruction,
    balance_change::balance_changes_of_grpc,
    flatten_instructions,
    parse_rpc_fetched_json::{BalanceChange, balance_change_of, parse_fetched_json},
};

use crate::system_ix::transfer_like::{
    spl_program::{SplCloseAccount, SplTransfer, SplTransferChecked},
    stake_program::WithdrawIx,
    transfer::Transfer,
};

pub mod spl_program;
pub mod stake_program;
pub mod transfer;

static WSOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

/// 统一的转账语义表示，将各种不同类型的转账指令抄象为同一结构。
#[derive(Debug, Clone, Copy)]
pub struct TransferLike {
    /// 转出方地址。
    pub from: Pubkey,
    /// 转入方地址。
    pub to: Pubkey,
    /// 转账金额（lamports 或 token 最小单位）。
    pub amount: u64,
}

impl TransferLike {
    /// 判断给定地址是否为转账参与方（转出或转入）。
    pub fn contains(&self, pubkey: &Pubkey) -> bool {
        self.from == *pubkey || self.to == *pubkey
    }
}

/// 从各种交易格式中解析 [`TransferLike`] 列表。
///
/// 同时支持 [`grpc_client::TransactionFormat`] 和 [`transaction_cache::TxDetailLocal`]。
#[async_trait::async_trait]
pub trait ParseTransfer {
    async fn parse_transfer(&self) -> Vec<TransferLike>;
}

#[async_trait::async_trait]
impl ParseTransfer for TransactionFormat {
    async fn parse_transfer(&self) -> Vec<TransferLike> {
        let Ok(balance_changes) = balance_changes_of_grpc(self) else {
            return vec![];
        };
        parse_transfer_like(&flatten_instructions(self), balance_changes)
    }
}

#[async_trait::async_trait]
impl ParseTransfer for TxDetailLocal {
    async fn parse_transfer(&self) -> Vec<TransferLike> {
        let Ok(balance_changes) = balance_change_of(self.clone()).await else {
            return vec![];
        };
        let ixs = parse_fetched_json(self.clone()).await;
        parse_transfer_like(&ixs, balance_changes)
    }
}

/// 从指令列表和余额变化中收集所有转账语义操作。
///
/// 支持 SOL 原生转账、Stake 提炸、SPL Token 转账及 wSOL ATA 关闭回收等场景。
fn parse_transfer_like(ixs: &[IndexedInstruction], bc: Vec<BalanceChange>) -> Vec<TransferLike> {
    let mut transfer_likes = Vec::new();
    for ix in ixs {
        if !ix.is_main_ix() {
            continue;
        }
        if let Some(ix) = Transfer::from_indexed_instruction(ix) {
            transfer_likes.push(TransferLike {
                from: ix.from,
                to: ix.to,
                amount: ix.lamports,
            });
        }
        if let Some(ix) = WithdrawIx::from_indexed_instruction(ix) {
            transfer_likes.push(TransferLike {
                from: ix.stake_account,
                to: ix.destination,
                amount: ix.amount,
            });
        }
        if let Some(ix) = SplTransferChecked::from_indexed_instruction(&ix) {
            transfer_likes.push(TransferLike {
                from: ix.from,
                to: ix.to,
                amount: ix.units,
            });
        }
        if let Some(ix) = SplTransfer::from_indexed_instruction(ix)
            && bc
                .iter()
                .any(|bc| bc.mint == WSOL && bc.token_account == ix.from)
        {
            transfer_likes.push(TransferLike {
                from: ix.from,
                to: ix.to,
                amount: ix.units,
            });
        }
        if let Some(ix) = SplCloseAccount::from_indexed_instruction(ix) {
            if let Some(bc) = bc
                .iter()
                .find(|bc| bc.mint == WSOL && bc.token_account == ix.from)
            {
                let amount = if bc.change < 0 {
                    (-bc.change) as u64
                } else {
                    bc.change as u64
                };
                transfer_likes.push(TransferLike {
                    from: ix.from,
                    to: ix.to,
                    amount,
                });
            }
        }
    }
    transfer_likes
}
