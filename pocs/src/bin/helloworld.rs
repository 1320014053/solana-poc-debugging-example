use std::{env, str::FromStr};

use poc_framework::{
    keypair, solana_sdk::signer::Signer, Environment, LocalEnvironment, PrintableTransaction,
};
// Anchor
//use anchor_client::solana_sdk::system_instruction;
//use anchor_client::{RequestBuilder, RequestNamespace};
//use helloworld::accounts as helloworld_accounts;
//use helloworld::instruction as helloworld_instruction;

use solana_program::{native_token::sol_to_lamports, pubkey::Pubkey, system_program};

use solana_program::instruction::{AccountMeta, Instruction};

pub fn main() {
    let _env = setup();
}

fn setup() -> LocalEnvironment {
    let mut dir = env::current_exe().unwrap();
    let path_hello_world_binary = {
        dir.pop();
        dir.pop();
        dir.pop();
        dir.push("liblevel0.so");
        dir.to_str()
    }
    .unwrap();

    let helloworld_program =
        Pubkey::from_str("EXBuYPNgBUXMTsjCbezENRUtFQzjUNZxvPGTd11Pznk5").unwrap();
    let payer = keypair(0);
    let greeting_account = keypair(1);
    let data: [u8; 4] = [0; 4];

    let mut env = LocalEnvironment::builder()
        .add_program(helloworld_program, path_hello_world_binary)
        .add_account_with_lamports(payer.pubkey(), system_program::ID, sol_to_lamports(1.0))
        .add_account_with_data(greeting_account.pubkey(), helloworld_program, &data, false)
        .build();

    env.execute_as_transaction(
        &[Instruction {
            program_id: helloworld_program,
            accounts: vec![AccountMeta::new(greeting_account.pubkey(), true)],
            data: vec![0xa7,0x34, 0xe2 ,0xad ,0xfd ,0xe9 ,0xbf ,0x3e ,0x61 ,0x5a ,0x12 ,0x63 ,0x80 ,0x90 ,0x20 ,0x29,0x00 ,0x00 ,0x00 ,0x00 ,0xe1 ,0xfd ,0x1b ,0x50 ,0xb6 ,0x52 ,0xdc ,0x85 ,0xc9 ,0x47 ,0xa1 ,0x33,0xee ,0x34 ,0xf6 ,0xec ,0x8e ,0xe2 ,0x9e ,0x1e ,0xda ,0xe7 ,0x1a ,0x6a ,0x05 ,0x2d ,0x2d ,0xea,0x12 ,0xd8 ,0xd1 ,0xc1 ],
        }],
        &[&greeting_account],
    )
    .print();

    env
}
