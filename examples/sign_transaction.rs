use std::time::{SystemTime, UNIX_EPOCH};
use wavesplatform::account::{PrivateKeyAccount, TESTNET};
use wavesplatform::base58::*;
use wavesplatform::seed::*;
use wavesplatform::transaction::*;

fn main() {
    let phrase = generate_phrase();
    let account = PrivateKeyAccount::from_seed(&phrase);
    println!(
        "My TESTNET address: {}",
        account.public_key().to_address(TESTNET).to_string()
    );

    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
        * 1000;
    let tx = Transaction::new_alias(&account.public_key(), "rhino", TESTNET, 100000, ts);
    println!("ID is {}", tx.id().to_string());
    let ptx = account.sign_transaction(tx);
    println!(
        "Proofs are {:?}",
        ptx.proofs
            .iter()
            .map(|p| p.to_base58())
            .collect::<Vec<String>>()
    );
}
