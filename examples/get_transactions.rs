use wavesplatform::node::{Node, MAINNET_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let node = Node::from_url(MAINNET_URL);

    let result = node
        .get_transactions_info("YwVPf35VckF4Yu5XwF18P9VwWwfQVGAQmqDp4bpgtuV")
        .await?;

    println!("{:?}", result);

    let result = node
        .get_transactions_status("YwVPf35VckF4Yu5XwF18P9VwWwfQVGAQmqDp4bpgtuV")
        .await?;

    println!("{}", result.status());

    Ok(())
}
