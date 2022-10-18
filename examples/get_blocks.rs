use wavesplatform::node::{Node, MAINNET_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::from_url(MAINNET_URL);

    let result = node.get_blocks_last().await?;

    println!("{:?}", result);

    let result = node
        .get_blocks_headers_at_height(result.height() - 10)
        .await?;

    println!("{:?}", result);

    let result = node.get_blocks_headers(&result.id()).await?;

    println!("{:?}", result);

    Ok(())
}
