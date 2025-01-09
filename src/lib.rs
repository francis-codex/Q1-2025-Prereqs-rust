use solana_sdk::{
    signature::{Keypair, Signer},
};
use std::io::{self, BufRead};
use bs58;

use solana_client::rpc_client::RpcClient;
use solana_sdk::signature::read_keypair_file;

const RPC_URL: &str = "https://api.devnet.solana.com";

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
}