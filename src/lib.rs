use solana_sdk::{
    signature::{Keypair, Signer},
};
use std::io::{self, BufRead};
use bs58;

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
}