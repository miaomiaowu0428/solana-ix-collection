//! Pump 指令辅助函数。

use solana_sdk::signature::Signature;
use utils::IndexedInstruction;

use crate::pump::{event::PumpTradeEvent, PumpTradeIx};

/// 从已展平的指令列表中配对收集 [`PumpTradeIx`] 和 [`PumpTradeEvent`]。
///
/// 按顺序将交易指令与对应的链上事件 zip 在一起并返回。
/// 当指令数量与事件数量不匹配时，多余的尾部会被扎啊。
pub fn collect_pump_trade_ix_event_pairs(
    flattened_ixs: &[IndexedInstruction],
) -> Vec<(PumpTradeIx, PumpTradeEvent)> {
    let processed_ixs = flattened_ixs
        .iter()
        .filter_map(PumpTradeIx::from_indexed_instruction);
    let processed_events = flattened_ixs
        .iter()
        .filter_map(PumpTradeEvent::from_indexed_instruction);

    // 4. Zip 和收集 (使用 .into_iter() 消耗 Vec，避免复制)
    let res: Vec<(PumpTradeIx, PumpTradeEvent)> = processed_ixs
        .zip(processed_events) // Zip 两个消耗性迭代器
        .collect();

    // 您现在可以使用 res 变量进行后续处理
    res
}
