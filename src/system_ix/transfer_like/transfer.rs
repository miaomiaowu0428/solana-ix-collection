use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::borsh1;
use solana_tx_parser::instruction;

// 来自solana_tx_parser的instruction宏
instruction!(
    // 转账指令是与system program交互
    program_id: "11111111111111111111111111111111",
    // 转账指令名称（可自定义）
    name: Transfer,
    // 转账指令的前4字节是固定的标识符
    discriminator: [0x02,0x00,0x00,0x00],
    // 转账指令的账户顺序定义
    accounts: {
        from: {
            writable: true,
            signer: true
        },
        to: {
            writable: true,
            signer: false
        }
    },
    // 转账指令的数据类型定义
    data: {
        lamports: u64,
    },
);

