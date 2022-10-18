use wavesplatform::node::{Node, MAINNET_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::from_url(MAINNET_URL);

    // Tether USD token
    let result = node
        .get_assets_details("34N9YcEETLWn93qYQ64EsP1x89tSruJU44RrEMSXXEPJ")
        .await?;

    println!("{:?}", result);

    Ok(())
}
