//! Pump 协议指令解析。
//!
//! 包含 Pump bonding curve 的全套交互指令：
//! - 买入：[`PumpBuyIx`]（精确 token）、[`PumpBuyExactInIx`]（精确 SOL）
//! - 卖出：[`PumpSellIx`]
//! - 创建：[`PumpCreateIx`] / [`PumpCreateV2Ix`]，以及统一枚举 [`PumpCreateIxEnum`]
//! - 迁移：[`PumpMigrateIx`]（bonding curve 升移到 AMM 池）
//! - 交易枥询：[`PumpTradeIx`]（买卖统一枚举）

use solana_sdk::borsh1;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_tx_parser::instruction;
use utils::{IndexedInstruction, impl_enum_getters};

use crate::constants::TOKEN_2022_PROGRAM_ID;

pub mod event;
pub mod helpers;
pub mod mayhem;

/// Pump 买入指令（精确 token 输出模式）。
///
/// `token_amount` 为期望购得的 token 数量，`max_sol_cost` 为最大可接受 SOL 入费量（lamports）。
instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    name: PumpBuyIx,
    discriminator: [102, 6, 61, 18, 1, 218, 235, 234],
    accounts: {
        global: { writable: false, signer: false },
        fee_recipient: { writable: true, signer: false },
        mint: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        associated_user: { writable: true, signer: false },
        user: { writable: true, signer: true },
        system_program: { writable: false, signer: false },
        token_program: { writable: false, signer: false },
        creator_vault: { writable: true, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        global_volume_accumulator: { writable: true, signer: false },
        user_volume_accumulator: { writable: true, signer: false },
        platform_fee_config: { writable: false, signer: false },
        pump_fee_program: { writable: false, signer: false },
    },
    data: {
        token_amount: u64,
        max_sol_cost: u64,
    },
);

/// Pump 买入指令（精确 SOL 输入模式）。
///
/// `sol_amount_in` 为精确注入的 SOL lamports（gross，含 1% 手续费），
/// `min_token_out` 为最少应返回的 token 数量（滑点保护）。
instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    name: PumpBuyExactInIx,
    discriminator: [56,252,116,8,158,223,205,95],
    accounts: {
        global: { writable: false, signer: false },
        fee_recipient: { writable: true, signer: false },
        mint: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        associated_user: { writable: true, signer: false },
        user: { writable: true, signer: true },
        system_program: { writable: false, signer: false },
        token_program: { writable: false, signer: false },
        creator_vault: { writable: true, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        global_volume_accumulator: { writable: true, signer: false },
        user_volume_accumulator: { writable: true, signer: false },
        platform_fee_config: { writable: false, signer: false },
        pump_fee_program: { writable: false, signer: false },
    },
    data: {
        sol_amount_in :u64,
        min_token_out: u64
    },
);

/// Pump 卖出指令。
///
/// `token_amoutn`（注意：链上存在拼写错误，已原样保留）为卖出 token 数量，
/// `min_sol_out` 为最少应返回的 SOL lamports（滑点保护）。
instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    name: PumpSellIx,
    discriminator: [51,230,133,164,1,127,131,173],
    accounts: {
        global: { writable: false, signer: false },
        fee_recipient: { writable: true, signer: false },
        mint: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        associated_user: { writable: true, signer: false },
        user: { writable: true, signer: true },
        system_program: { writable: false, signer: false },
        creator_vault: { writable: true, signer: false },
        token_program: { writable: false, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        platform_fee_config: { writable: false, signer: false },
        pump_fee_program: { writable: false, signer: false },
    },
    data: {
        token_amoutn: u64,
        min_sol_out: u64,
    },
);

/// Pump 创建 token（v1）指令（标准 SPL Token）。
///
/// 创建 bonding curve 并初始化 token metadata，使用标准 SPL Token 程序。
instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    name: PumpCreateIx,
    discriminator: [24,30,200,40,5,28,7,119],
    accounts: {
        mint: { writable: true, signer: true },
        mint_authority: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        global: { writable: false, signer: false },
        mpl_token_metadata: { writable: false, signer: false },
        metadata: { writable: true, signer: false },
        user: { writable: true, signer: true },
        system_program: { writable: false, signer: false },
        token_program: { writable: false, signer: false },
        associated_token_program: { writable: false, signer: false },
        rent: { writable: false, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
    },
    data: {
        name: String,
        symbol: String,
        uri: String,
        creator: solana_sdk::pubkey::Pubkey,
    }
);

/// Pump 创建 token（v2）指令（支持 Mayhem / Token-2022）。
///
/// 在 v1 基础上增加了 `mayhem` 开关，
/// mayhem 模式使用 Token-2022 程序发行 token。
instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    name: PumpCreateV2Ix,
    discriminator: [214,144,76,236,95,139,49,180],
    accounts: {
        mint: { writable: true, signer: true },
        mint_authority: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        global: { writable: false, signer: false },
        user: { writable: true, signer: true},
        system: { writable: false, signer: false},
        token_program: { writable: false, signer: false},
    },
    data: {
        name: String,
        symbol: String,
        uri: String,
        creator: solana_sdk::pubkey::Pubkey,
        mayhem: bool
    }
);

/// Pump 创建指令统一枚举，同时支持 v1 和 v2。
#[derive(Debug, Clone)]
pub enum PumpCreateIxEnum {
    Create(PumpCreateIx),
    CreateV2(PumpCreateV2Ix),
}

impl_enum_getters!(
    PumpCreateIxEnum,
    variants = [Create,CreateV2],
    fields = [
        mint: Pubkey,
        creator: Pubkey,
        token_program: Pubkey,
        slot:u64,
    ]
);

impl PumpCreateIxEnum {
    /// 返回该 token 的 symbol 字符串。
    pub fn symbol(&self) -> &str {
        match self {
            PumpCreateIxEnum::Create(ix) => &ix.symbol,
            PumpCreateIxEnum::CreateV2(ix) => &ix.symbol,
        }
    }

    /// 判断该创建指令是否为 Mayhem 模式（Token-2022 发行）。
    pub fn is_mayhem(&self) -> bool {
        match self {
            PumpCreateIxEnum::Create(ix) => false,
            PumpCreateIxEnum::CreateV2(ix) => ix.mayhem,
        }
    }

    /// 判断该创建指令是否启用了 cashback（返利）功能。
    ///
    /// 通过读取 `remain_data` 首字节非零来判定。
    pub fn cashback_enabled(&self) -> bool {
        match self {
            PumpCreateIxEnum::Create(_) => false,
            PumpCreateIxEnum::CreateV2(c) => c.remain_data.first().unwrap_or(&0) != &0,
        }
    }
}
impl TryFrom<IndexedInstruction> for PumpCreateIxEnum {
    type Error = ();
    fn try_from(ix: IndexedInstruction) -> Result<Self, Self::Error> {
        Self::try_from(&ix)
    }
}

impl TryFrom<&IndexedInstruction> for PumpCreateIxEnum {
    type Error = ();
    fn try_from(ix: &IndexedInstruction) -> Result<Self, Self::Error> {
        PumpCreateIx::from_indexed_instruction(ix)
            .map(Self::Create)
            .or_else(|| PumpCreateV2Ix::from_indexed_instruction(ix).map(Self::CreateV2))
            .ok_or(())
    }
}

/// Pump 交易指令统一枚举，覆盖买入（两种模式）和卖出。
///
/// 可遇过 [`PumpTradeIx::from_indexed_instruction`] 从链上指令自动识别并构建。
#[derive(Debug, Clone)]
pub enum PumpTradeIx {
    Buy(PumpBuyIx),
    BuyExactIn(PumpBuyExactInIx),
    Sell(PumpSellIx),
}

impl PumpTradeIx {
    /// 尝试从单条 [`IndexedInstruction`] 解析出一个 [`PumpTradeIx`]。
    ///
    /// 依次尝试 `Buy` → `BuyExactIn` → `Sell`，首个匹配成功即返回。
    pub fn from_indexed_instruction(ix: &IndexedInstruction) -> Option<Self> {
        if let Some(ix) = PumpBuyIx::from_indexed_instruction(ix) {
            return Some(Self::Buy(ix));
        }
        if let Some(ix) = PumpBuyExactInIx::from_indexed_instruction(ix) {
            return Some(Self::BuyExactIn(ix));
        }
        if let Some(ix) = PumpSellIx::from_indexed_instruction(ix) {
            return Some(Self::Sell(ix));
        }
        None
    }
}

impl_enum_getters!(
    PumpTradeIx,
    variants = [Buy,BuyExactIn,Sell],
    fields = [
        slot: u64,
        fee_recipient: Pubkey,
        token_program: Pubkey,
        user: Pubkey,
        mint: Pubkey,
        bonding_curve: Pubkey,
    ]
);

impl PumpTradeIx {
    /// 判断该交易是否为 Mayhem 模式（Token-2022 + Mayhem 手续费账户）。
    pub fn is_mayhem(&self) -> bool {
        self.token_program() == TOKEN_2022_PROGRAM_ID
            && mayhem::MAYHEM_FEE_RECV.contains(&&self.fee_recipient())
    }

    /// 判断该交易是否为买入方向。
    pub fn is_buy(&self) -> bool {
        match self {
            PumpTradeIx::Buy { .. } => true,
            PumpTradeIx::BuyExactIn { .. } => true,
            PumpTradeIx::Sell { .. } => false,
        }
    }
}

/// Pump bonding curve 升移到 AMM 池的迁移指令。
///
/// 当 bonding curve 到达升移阈值时就会触发此指令，它将汁动性迁移到 Pump AMM 池。
instruction! {
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
    name: PumpMigrateIx,
    discriminator: [155, 234, 231, 146, 236, 158, 162, 30],
    accounts: {
        global: { writable: false, signer: false },
        withdraw_authority: { writable: true, signer: false },
        mint: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        user: { writable: false, signer: true },
        system_program: { writable: false, signer: false },
        token_program: { writable: false, signer: false },
        pump_amm: { writable: false, signer: false },
        pool: { writable: true, signer: false },
        pool_authority: { writable: true, signer: false },
        pool_authority_mint_account: { writable: true, signer: false },
        pool_authority_wsol_account: { writable: true, signer: false },
        amm_global_config: { writable: false, signer: false },
        wsol_mint: { writable: false, signer: false },
        lp_mint: { writable: true, signer: false },
        creator_vault: { writable: true, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        user_acc_target: { writable: true, signer: false },
        user_acc_wsol: { writable: true, signer: false },
        token_2022_program: { writable: false, signer: false },
        associated_token_program: { writable: false, signer: false }
    },
    data: {}
}
