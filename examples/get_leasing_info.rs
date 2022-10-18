use wavesplatform::node::{Node, MAINNET_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::from_url(MAINNET_URL);

    let result = node
        .get_leasing_info("YwVPf35VckF4Yu5XwF18P9VwWwfQVGAQmqDp4bpgtuV")
        .await?;

    println!("{:?}", result);

    Ok(())
}
