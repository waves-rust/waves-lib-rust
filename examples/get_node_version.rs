use wavesplatform::node::{Node, MAINNET_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::from_url(MAINNET_URL);

    let result = node.get_node_version().await?;

    println!("Version: {}", result.version());

    Ok(())
}
