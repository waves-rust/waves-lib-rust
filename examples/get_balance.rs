use wavesplatform::node::{Node, MAINNET_URL};
use wavesplatform::util::Amount;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::from_url(MAINNET_URL);

    // If it looks like a duck, swims like a duck, and quacks like a duck, then it probably is a duck.
    let result = node
        .get_balance("3PEktVux2RhchSN63DsDo4b4mz4QqzKSeDv")
        .await?;

    let balance = Amount::from_wavelet(result.balance());

    println!("Balance: {} WAVES", balance);

    let result = node
        .get_balance_details("3PEktVux2RhchSN63DsDo4b4mz4QqzKSeDv")
        .await?;

    let balance = Amount::from_wavelet(result.regular());

    println!("Regular balance: {} WAVES", balance);

    Ok(())
}
