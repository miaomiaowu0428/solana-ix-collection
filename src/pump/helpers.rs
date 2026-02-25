use solana_sdk::signature::Signature;
use utils::IndexedInstruction;

use crate::pump::{PumpTradeIx, event::PumpTradeEvent};

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
