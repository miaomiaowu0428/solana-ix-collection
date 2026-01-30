use std::{fmt::Display, io::Read};

use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{borsh1, pubkey::Pubkey};
use solana_tx_parser::instruction;

instruction!(
    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    name: SplTransferChecked,
    discriminator: [12],
    accounts: {
        from: {
            writable: true,
            signer: true
        },
        mint: {
            writable: true,
            signer: false
        },
        to: {
            writable: true,
            signer: false
        },
        authority: {
            writable: false,
            signer: true
        },
    },
    data: {
        units: u64,
    },
);


instruction!(
    program_id: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    name: SplTransfer,
    discriminator: [3],
    accounts: {
        from: {
            writable: true,
            signer: true
        },
        to: {
            writable: true,
            signer: false
        },
        auth: {
            writable: true,
            signer: false
        },
    },
    data: {
        units: u64,
    },
);