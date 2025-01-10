mod programs; 

use crate::programs::turbin3_prereq::{Turbin3PrereqProgram, CompleteArgs};
use solana_program::system_program;
use solana_sdk::{
    signature::{Keypair, Signer},
    pubkey::Pubkey,

};
use std::io::{self, BufRead};
use std::str::FromStr;
use bs58;

use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;
const RPC_URL: &str = "https://api.devnet.solana.com";

use solana_program::system_instruction::transfer;
use solana_sdk::transaction::Transaction;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen() {
        let keypair = Keypair::new();
        println!("New Solana wallet: {}", keypair.pubkey());
        println!("\nWallet private key (save to JSON file):");
        println!("{:?}", keypair.to_bytes());
    }

    #[test]
    fn base58_to_wallet() {
        println!("Enter private key (base58):");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("Wallet file format:\n{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Enter wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_matches(|c| c == '[' || c == ']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Base58 private key:\n{}", bs58::encode(wallet).into_string());
    }

    #[test]
fn airdrop() {
    let keypair = read_keypair_file("dev-wallet.json")
        .expect("Failed to read wallet file");
    
    let client = RpcClient::new(RPC_URL);
    
    match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
        Ok(signature) => {
            println!("Success! Transaction:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", 
                    signature.to_string());
        },
        Err(e) => println!("Error: {}", e)
    }
}

#[test]
fn transfer_sol() {
    let keypair = read_keypair_file("dev-wallet.json")
        .expect("Failed to read wallet file");
    
    let to_pubkey = Pubkey::from_str("9rxVXJ12mBoRrtegEjjs7KBkHuYaDJ5YjrbY4ZEuyQPy")
        .expect("Invalid public key");
    
    let client = RpcClient::new(RPC_URL);
    
    let recent_blockhash = client
        .get_latest_blockhash()
        .expect("Failed to get blockhash");
    
    let transaction = Transaction::new_signed_with_payer(
        &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
        Some(&keypair.pubkey()),
        &vec![&keypair],
        recent_blockhash
    );
    
    match client.send_and_confirm_transaction(&transaction) {
        Ok(signature) => {
            println!("Success! Transaction:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", 
                    signature.to_string());
        },
        Err(e) => println!("Error: {}", e)
    }
}

#[test]
fn complete_prerequisites() -> Result<(), Box<dyn std::error::Error>> {
    let signer = read_keypair_file("turbin3-wallet.json")
        .expect("Failed to read wallet file");
    
    let client = RpcClient::new(RPC_URL);
    
    let prereq = Turbin3PrereqProgram::derive_program_address(&[
        b"prereq",
        signer.pubkey().to_bytes().as_ref()
    ]);
    
    // Check if account exists first
    match client.get_account(&prereq) {
        Err(_) => {
            // Account doesn't exist, create it
            let args = CompleteArgs {
                github: "francis-codex".as_bytes().to_vec()
            };
            
            let blockhash = client
                .get_latest_blockhash()
                .expect("Failed to get blockhash");
            
            let transaction = Turbin3PrereqProgram::complete(
                &[&signer.pubkey(), &prereq, &system_program::id()],
                &args,
                Some(&signer.pubkey()),
                &[&signer],
                blockhash
            );
            
            let signature = client.send_and_confirm_transaction(&transaction)?;
            println!("New account created! Transaction:");
            println!("https://explorer.solana.com/tx/{}?cluster=devnet", 
                    signature.to_string());
        },
        Ok(account) => {
            println!("Account already exists at: {}", prereq);
            println!("View account on Solana Explorer:");
            println!("https://explorer.solana.com/address/{}?cluster=devnet", 
                    prereq.to_string());
            println!("Account data size: {} bytes", account.data.len());
            println!("Account owner: {}", account.owner);
        }
    }
    
    Ok(())
}}