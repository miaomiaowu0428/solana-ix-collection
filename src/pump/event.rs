use solana_sdk::borsh1;
use solana_tx_parser::instruction;
use solana_sdk::pubkey::Pubkey;

// 模仿 instruction! 宏结构的 TradeEvent 定义（统一事件/指令处理风格）
instruction! {
    // 1. 程序ID：与指令保持一致
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    // 2. 事件名称：对应原 TradeEvent
    name: TradeEventIx,
    // 3. 事件鉴别符（与原 discriminator 一致）
    discriminator: [228, 69, 165, 46, 81, 203, 154, 29,189,219,127,211,78,230,97,238],
    // 4. 事件无实际 accounts（指令的 accounts 对应账户列表，事件无此概念，留空或注释说明）
    accounts: {},
    // 5. 事件所有字段归入 data 块（与指令的 data 结构对齐）
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
        // 保留原有拼写错误：track_valume（不修正）
        track_valume: bool,
        total_unclaimed_tokens: u64,
        total_claimed_tokens: u64,
        current_sol_volume: u64,
        // 保留原有命名不一致：last_updated（不修正）
        last_updated: i64,
        ix_name: String,
        // 保留原有命名不一致：is_mayhem（不修正）
        is_mayhem: bool,
        // 仅新增：现金返还手续费基准点
        cashback_fee_basis_points: u64,
        // 仅新增：现金返还金额
        cashback: u64
    },
}