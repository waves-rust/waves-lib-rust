# Acryl

![crates.io](https://img.shields.io/crates/v/acryl.svg)
![docs.rs](https://docs.rs/acryl/badge.svg)

A Rust interface to the [Acryl blockchain](https://acrylplatform.com)

# Usage
```rust
use base58::*;
use std::time::{SystemTime, UNIX_EPOCH};
use acryl::account::{PrivateKeyAccount, TESTNET};
use acryl::seed::*;
use acryl::transaction::*;

fn main() {
    let phrase = generate_phrase();
    let account = PrivateKeyAccount::from_seed(phrase);
    println!("My TESTNET address: {}", account.public_key().to_address(TESTNET).to_string());

    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
    let tx = Transaction::new_alias(&account.public_key(), "rhino", TESTNET, 100000, ts);
    println!("ID is {}", tx.id().to_string());
    let ptx = account.sign_transaction(tx);
    println!("Proofs are {:?}", ptx.proofs.iter().map(|p| p.to_base58()).collect::<Vec<String>>());
}
```