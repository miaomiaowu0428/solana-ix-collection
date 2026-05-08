use utils::PoolPriceInfo;

use crate::pump::PumpTradeIx;

/// 为 PoolPriceInfo 扩展 Pump 指令驱动的状态推进能力。
///
/// shred 侧常见用法：
/// ```ignore
/// let pool = ixs.iter()
///     .filter_map(PumpTradeIx::from_indexed_instruction)
///     .fold(PUMP_INIT_POOL, |p, ix| p.apply_pump(&ix, 0.0125));
/// ```
pub trait PumpPoolExt {
    /// 根据一条 Pump 指令模拟成交后的新 pool 状态。
    ///
    /// - `fee_rate`: 扣费比例，如 `0.0125` 代表 1.25%。shred 场景通常传固定值。
    fn apply_pump(&self, ix: &PumpTradeIx, fee_rate: f64) -> Self;
}

impl PumpPoolExt for PoolPriceInfo {
    fn apply_pump(&self, ix: &PumpTradeIx, fee_rate: f64) -> Self {
        match ix {
            // Exact-out buy: 指令携带 token 输出量
            PumpTradeIx::Buy(i) => self.after_buy_base_exact_out(i.token_amount, fee_rate),
            PumpTradeIx::BuyV2(i) => self.after_buy_base_exact_out(i.amount, fee_rate),
            // Exact-in buy: 指令携带 quote 输入量
            PumpTradeIx::BuyExactIn(i) => self.after_buy_quote_exact_in(i.sol_amount_in, fee_rate),
            PumpTradeIx::BuyExactQuoteInV2(i) => {
                self.after_buy_quote_exact_in(i.spendable_quote_in, fee_rate)
            }
            // Sell: 指令携带 base token 输入量
            PumpTradeIx::Sell(i) => self.after_sell_base_exact_in(i.token_amoutn, fee_rate),
            PumpTradeIx::SellV2(i) => self.after_sell_base_exact_in(i.amount, fee_rate),
        }
    }
}
