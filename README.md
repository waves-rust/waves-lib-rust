# Acryl

![crates.io](https://img.shields.io/crates/v/acryl.svg)
![docs.rs](https://docs.rs/acryl/badge.svg)

A Rust interface to the [Acryl blockchain](https://acrylplatform.com)

# Usage
```rust
use base58::*;
use std::time::{SystemTime, UNIX_EPOCH};
use acryl::account::{PrivateKeyAccount, TESTNET};
use acryl::transaction::*;

fn main() {
    let account = PrivateKeyAccount::from_seed("seed");
    println!("my address: {}", account.public_key().to_address(TESTNET).to_string());

    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() * 1000;
    let tx = Transaction::new_alias(&account.public_key(), "rhino", TESTNET, 100000, ts);
    println!("id is {}", tx.id().to_string());
    let ptx = account.sign_transaction(tx);
    println!("proofs are {:?}", ptx.proofs.iter().map(|p| p.to_base58()).collect::<Vec<String>>());
}
```