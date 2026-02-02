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

#[derive(Debug, Clone, Copy)]
pub struct TransferLike {
    pub from: Pubkey,
    pub to: Pubkey,
    pub amount: u64,
}

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
        // balance_change_of 需要值类型
        let Ok(balance_changes) = balance_change_of(self.clone()).await else {
            return vec![];
        };
        // parse_fetched_json 也是异步函数
        let ixs = parse_fetched_json(self.clone()).await;
        parse_transfer_like(&ixs, balance_changes)
    }
}

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
            // Find the balance change for this token account (WSOL) and use its change as amount.
            // BalanceChange.change is i128 (after - pre), take absolute value and cast to u64.
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
