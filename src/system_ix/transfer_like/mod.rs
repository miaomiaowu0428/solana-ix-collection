use solana_sdk::{pubkey, pubkey::Pubkey};

use utils::IndexedInstruction;

use crate::system_ix::transfer_like::{
    spl_program::SplTransferChecked, stake_program::WithdrawIx, transfer::Transfer,
};

pub mod spl_program;
pub mod stake_program;
pub mod transfer;

static WSOL: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

pub enum TransferLike {
    Transfer(Transfer),
    Withdraw(WithdrawIx),
    WsolTransfer(SplTransferChecked),
}

impl TryFrom<&IndexedInstruction> for TransferLike {
    type Error = ();

    fn try_from(ix: &IndexedInstruction) -> Result<Self, Self::Error> {
        macro_rules! try_from_ix {
            ($ix:expr, $( $variant:ident => $ty:ty ),+ $(,)?) => {{
                $(
                    if let Some(value) = <$ty>::from_indexed_instruction($ix) {
                        return Ok(Self::$variant(value));
                    }
                )+
                // return Err(());
            }};
        }

        try_from_ix!(
            ix,
            Transfer => Transfer,
            Withdraw => WithdrawIx,
        );

        if let Some(value) = SplTransferChecked::from_indexed_instruction(ix)
            && value.mint == WSOL
        {
            return Ok(Self::WsolTransfer(value));
        }

        Err(())
    }
}

impl From<TransferLike> for Transfer {
    fn from(value: TransferLike) -> Self {
        match value {
            TransferLike::Transfer(t) => t,
            TransferLike::Withdraw(w) => Transfer {
                from: w.authority,
                to: w.destination,
                lamports: w.amount,
                remain_accounts: w.remain_accounts,
                slot: w.slot,
            },
            TransferLike::WsolTransfer(t) => Transfer {
                from: t.from,
                to: t.to,
                lamports: t.units,
                remain_accounts: t.remain_accounts,
                slot: t.slot,
            },
        }
    }
}
