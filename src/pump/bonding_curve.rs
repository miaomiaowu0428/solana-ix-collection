//! Pump BondingCurve 账户数据解析。
//!
//! 账户数据格式：`[8 字节 discriminator][borsh BondingCurve]`
//!
//! Discriminator: `[23, 183, 248, 55, 96, 216, 172, 96]`
//!
//! 配合 `transaction-monitor::get_account` 使用:
//! ```ignore
//! let bc = disp.get_account(&addr, |buf| BondingCurve::try_parse(buf));
//! ```

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::borsh1;
use solana_sdk::pubkey::Pubkey;
use utils::PoolPriceInfo;

/// Pump.fun BondingCurve 的 8 字节 discriminator
pub const DISCRIMINATOR: [u8; 8] = [23, 183, 248, 55, 96, 216, 172, 96];

/// Pump.fun BondingCurve 账户数据
#[derive(Debug, Clone, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct BondingCurve {
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
    pub creator: Pubkey,
    pub is_mayhem_mode: bool,
}

impl BondingCurve {
    /// 从原始账户数据（含 8 字节 discriminator 前缀）解析。
    pub fn try_from(data: &[u8]) -> Option<Self> {
        if data.len() < 8 || data[..8] != DISCRIMINATOR {
            return None;
        }
        borsh1::try_from_slice_unchecked(&data[8..]).ok()
    }

    /// 当前 SOL 本位价格 = virtual_sol_reserves / virtual_token_reserves
    /// 仅在 virtual_token_reserves > 0 时返回有效值。
    pub fn price(&self) -> f64 {
        let mut p = PoolPriceInfo{
            base_reserve: self.virtual_token_reserves,
            quote_reserve: self.virtual_sol_reserves,
            ..Default::default()
        };
        p.update_price();
        p.base_price_in_quote
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_bonding_curve() {
        // TODO: 替换为链上实际 bonding curve 的 hex
        let hex = "17b7f83760d8ac600c11f440e3cf0300c6b823fc060000000c79e1f451d10200c60c0000000000000080c6a47e8d030000965811ab360412740dbd9f92544bdaccf2195c26d2f0729148bac4e2da64f25100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";
        let data = hex::decode(hex).expect("invalid hex");
        let bc = BondingCurve::try_from(&data);
        assert!(bc.is_some(), "parse should succeed");
        let bc = bc.unwrap();
        assert!(bc.virtual_token_reserves > 0);
        assert!(bc.virtual_sol_reserves > 0);
        assert_ne!(bc.creator, Pubkey::default());
    }

    #[test]
    fn test_wrong_discriminator() {
        assert!(BondingCurve::try_from(&[0u8; 64]).is_none());
    }

    #[test]
    fn test_too_short() {
        assert!(BondingCurve::try_from(&[0u8; 4]).is_none());
    }
}
