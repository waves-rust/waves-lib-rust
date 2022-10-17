use wavesplatform::node::{Node, MAINNET_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::from_url(MAINNET_URL);

    // Assemble all the puzzles
    let result = node.get_address_by_alias("vlzhr").await?;

    println!("vlzhr -> {}", result.address());

    Ok(())
}
