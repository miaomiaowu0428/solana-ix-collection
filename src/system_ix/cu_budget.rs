use solana_sdk::borsh1;
use solana_tx_parser::instruction;

instruction!(
    program_id: "ComputeBudget111111111111111111111111111111",
    name: SetComputUnitPrice,
    discriminator: [3],
    accounts: {},
    data: {
        micro_lamports:u64,
    },
);

instruction!(
    program_id: "ComputeBudget111111111111111111111111111111",
    name: SetComputUnitLimit,
    discriminator: [2],
    accounts: {},
    data: {
        units:u32,
    },
);
