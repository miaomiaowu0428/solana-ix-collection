use std::{fmt::Display, io::Read};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{borsh1, pubkey::Pubkey};
use solana_tx_parser::instruction;
use utils::IndexedInstruction;



instruction! {
    program_id: "Stake11111111111111111111111111111111111111",
    discriminator: [4,0,0,0],
    name: WithdrawIx,
    accounts:{
        stake_account     :{ writable: false, signer: false },
        destination       :{ writable: false, signer: false },
        sys_clock         :{ writable: false, signer: false },
        sys_history       :{ writable: false, signer: false },
        authority         :{ writable: false, signer: false },
    },
    data:{
        amount: u64,
    }
}

instruction! {
    program_id: "Stake11111111111111111111111111111111111111",
    discriminator: [0, 0, 0, 0],
    name: StakeInitializeIx,
    accounts: {
        stake_account : { writable: true,  signer: false },
        rent_sysvar   : { writable: false, signer: false },
    },
    data: {
        data: InitData, // 👈 不解析，交给你自己
    }
}

#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct InitData {
    authorized: Authorized,
    lockup: Lockup,
}
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]
pub struct Authorized {
    pub staker: Pubkey,     // 32 bytes
    pub withdrawer: Pubkey, // 32 bytes
}
#[derive(Debug, Clone, BorshDeserialize, BorshSerialize)]

pub struct Lockup {
    pub unix_timestamp: i64, // 8 bytes
    pub epoch: u64,          // 8 bytes
    pub custodian: Pubkey,   // 32 bytes
}