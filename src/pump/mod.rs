use solana_sdk::borsh1;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Signature;
use solana_tx_parser::instruction;
use utils::{IndexedInstruction, impl_enum_getters};

use crate::constants::TOKEN_2022_PROGRAM_ID;

pub mod event;
pub mod mayhem;

instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P", // 沿用泵协议程序ID（与CreateV2一致）
    name: PumpBuyIx,
    discriminator: [102, 6, 61, 18, 1, 218, 235, 234], // 替换为Buy指令实际8字节 discriminator（从链上指令数据前8字节提取）
    accounts: {
        // 严格对应 AccountMeta 列表，writable/signer 与 new/new_readonly 一致
        global: { writable: false, signer: false },
        fee_recipient: { writable: true, signer: false },
        mint: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        associated_user: { writable: true, signer: false },
        user: { writable: true, signer: true }, // AccountMeta::new(user, true) → 可写+签名者
        system_program: { writable: false, signer: false },
        token_program: { writable: false, signer: false },
        creator_vault: { writable: true, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        global_volume_accumulator: { writable: true, signer: false },
        user_volume_accumulator: { writable: true, signer: false },
        platform_fee_config: { writable: false, signer: false }, // 对应 get_platform_fee_config 返回的账户
        pump_fee_program: { writable: false, signer: false }, // 对应 pump_fee::PROGRAM_ADDRESS
    },
    data: {
        token_amount: u64,
        max_sol_cost: u64,
    },
);

instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P", // 沿用泵协议程序ID（与CreateV2一致）
    name: PumpBuyExactInIx,
    discriminator: [56,252,116,8,158,223,205,95], // 替换为Buy指令实际8字节 discriminator（从链上指令数据前8字节提取）
    accounts: {
        // 严格对应 AccountMeta 列表，writable/signer 与 new/new_readonly 一致
        global: { writable: false, signer: false },
        fee_recipient: { writable: true, signer: false },
        mint: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        associated_user: { writable: true, signer: false },
        user: { writable: true, signer: true }, // AccountMeta::new(user, true) → 可写+签名者
        system_program: { writable: false, signer: false },
        token_program: { writable: false, signer: false },
        creator_vault: { writable: true, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        global_volume_accumulator: { writable: true, signer: false },
        user_volume_accumulator: { writable: true, signer: false },
        platform_fee_config: { writable: false, signer: false }, // 对应 get_platform_fee_config 返回的账户
        pump_fee_program: { writable: false, signer: false }, // 对应 pump_fee::PROGRAM_ADDRESS
    },
    data: {
        sol_amount_in :u64,
        min_token_out: u64
    },
);

instruction!(
    program_id: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P", // 沿用泵协议程序ID（与CreateV2一致）
    name: PumpSellIx,
    discriminator: [51,230,133,164,1,127,131,173], // 替换为Buy指令实际8字节 discriminator（从链上指令数据前8字节提取）
    accounts: {
        // 严格对应 AccountMeta 列表，writable/signer 与 new/new_readonly 一致
        global: { writable: false, signer: false },
        fee_recipient: { writable: true, signer: false },
        mint: { writable: false, signer: false },
        bonding_curve: { writable: true, signer: false },
        associated_bonding_curve: { writable: true, signer: false },
        associated_user: { writable: true, signer: false },
        user: { writable: true, signer: true }, // AccountMeta::new(user, true) → 可写+签名者
        system_program: { writable: false, signer: false },
        creator_vault: { writable: true, signer: false },
        token_program: { writable: false, signer: false },
        event_authority: { writable: false, signer: false },
        program: { writable: false, signer: false },
        platform_fee_config: { writable: false, signer: false }, // 对应 get_platform_fee_config 返回的账户
        pump_fee_program: { writable: false, signer: false }, // 对应 pump_fee::PROGRAM_ADDRESS
    },
    data: {
        token_amoutn: u64,
        min_sol_out: u64,
    },
);

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
    pub fn is_mayhem(&self) -> bool {
        match self {
            PumpCreateIxEnum::Create(ix) => false,
            PumpCreateIxEnum::CreateV2(ix) => ix.mayhem,
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
        // 使用 or_else 形成链式调用
        PumpCreateIx::from_indexed_instruction(ix)
            .map(Self::Create) // 如果是 Some，包装成 Enum
            .or_else(|| {
                // 如果是 None，尝试解析 V2
                PumpCreateV2Ix::from_indexed_instruction(ix).map(Self::CreateV2)
            })
            .ok_or(()) // 如果最后全是 None，转换成 Err(())
    }
}

#[derive(Debug, Clone)]
pub enum PumpTradeIx {
    Buy(PumpBuyIx),
    BuyExactIn(PumpBuyExactInIx),
    Sell(PumpSellIx),
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
    pub fn is_mayhem(&self) -> bool {
        self.token_program() == TOKEN_2022_PROGRAM_ID && mayhem::MAYHEM_FEE_RECV.contains(&&self.fee_recipient())
    }

    pub fn is_buy(&self) -> bool {
        match self {
            PumpTradeIx::Buy { .. } => true,
            PumpTradeIx::BuyExactIn { .. } => true,
            PumpTradeIx::Sell { .. } => false,
        }
    }
}

