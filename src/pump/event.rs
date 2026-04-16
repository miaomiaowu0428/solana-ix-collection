//! Pump 链上交易事件解析。
//!
//! [`PumpTradeEvent`] 是 Pump 协议在每笔买卖交易中射出的链上日志事件，
//! 包含储备余额、成交金额、手续费等完整交易信息。

use solana_sdk::borsh1;
use solana_sdk::pubkey::Pubkey;
use solana_tx_parser::instruction;
/// Pump 链上交易事件。
///
/// 包含一笔 buy/sell 交易的全量信息，包括虚拟储备、实际储备、手续费、
/// creator 返利等。字段名称已按链上实际布局原样保留（含拼写错误）。
instruction! {
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    name: PumpTradeEvent,
    discriminator: [228, 69, 165, 46, 81, 203, 154, 29,189,219,127,211,78,230,97,238],
    accounts: {},
    data: {
        mint: Pubkey,
        sol_amount: u64,
        token_amount: u64,
        is_buy: bool,
        user: Pubkey,
        timestamp: i64,
        virtual_sol_reserves: u64,
        virtual_token_reserves: u64,
        real_sol_reserves: u64,
        real_token_reserves: u64,
        fee_recipient: Pubkey,
        fee_basis_points: u64,
        fee: u64,
        creator: Pubkey,
        creator_fee_basis_points: u64,
        creator_fee: u64,
        track_valume: bool,
        total_unclaimed_tokens: u64,
        total_claimed_tokens: u64,
        current_sol_volume: u64,
        last_updated: i64,
        ix_name: String,
        is_mayhem: bool,
        cashback_fee_basis_points: u64,
        cashback: u64
    },
}
