# Solana Enrollment dApp

A Rust-based Solana development project that demonstrates basic blockchain interactions including keypair generation, token airdrops, and smart contract interactions on Solana's devnet.

## Project Overview

This project implements a series of Solana blockchain interactions:
1. Keypair generation and wallet management
2. Devnet SOL token airdrops
3. Token transfers between wallets
4. Integration with the Turbin3 prerequisite program for course enrollment

## Prerequisites

- Rust and Cargo installed
- Solana CLI tools
- A Solana wallet (can be generated using this project)
- Basic understanding of Solana development concepts

## Dependencies

Add these to your `Cargo.toml`:
```toml
[dependencies]
solana-sdk = "1.15.2"
solana-client = "1.15.2"
solana-program = "1.15.2"
borsh = "0.10.3"
solana-idlgen = { git = "https://github.com/deanmlittle/solana-idlgen.git" }
bs58 = "latest"
```

## Project Structure

```
src/
├── lib.rs
├── programs/
│   ├── mod.rs
│   └── Turbin3_prereq.rs
```

## Setup Instructions

1. Initialize a new Rust library:
   ```bash
   cargo init --lib
   ```

2. Create required directories and files:
   ```bash
   mkdir src/programs
   touch ./src/programs/mod.rs
   touch ./src/programs/Turbin3_prereq.rs
   ```

3. Add a `.gitignore` file:
   ```
   *wallet.json
   ```

## Features

### 1. Wallet Management
- Generate new Solana keypairs
- Convert between wallet formats (byte array and base58)
- Save wallet credentials securely

### 2. Token Operations
- Request SOL airdrop from devnet
- Transfer SOL between wallets
- Calculate and handle transaction fees
- Empty wallet functionality

### 3. Turbin3 Program Integration
- Interact with the Turbin3 prerequisite program
- Handle Program Derived Addresses (PDAs)
- Submit course completion verification

## Usage Examples

### Generate a New Keypair
```rust
#[test]
fn keygen() {
    let kp = Keypair::new();
    println!("You've generated a new Solana wallet: {}", kp.pubkey().to_string());
    println!("Private key: {:?}", kp.to_bytes());
}
```

### Request Devnet Airdrop
```rust
let client = RpcClient::new(RPC_URL);
client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64);
```

### Transfer SOL
```rust
let transaction = Transaction::new_signed_with_payer(
    &[transfer(
        &keypair.pubkey(),
        &to_pubkey,
        amount_in_lamports
    )],
    Some(&keypair.pubkey()),
    &vec![&keypair],
    recent_blockhash
);
```

## Important Notes

- Always keep your private keys secure and never commit them to version control
- Use devnet for testing and development
- Monitor transaction fees when performing operations
- Remember to handle errors appropriately in production code

## Program Details

The Turbin3 prerequisite program is deployed on Solana devnet at:
`ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa`

## Contributing

Feel free to submit issues and enhancement requests.
