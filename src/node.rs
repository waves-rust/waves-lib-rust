/// Module with a set of node answer struct
pub mod response;

use response::*;

/// Mainnet node REST API
pub const MAINNET_URL: &str = "https://nodes.wavesnodes.com";
/// Testnet node REST API
pub const TESTNET_URL: &str = "https://nodes-testnet.wavesnodes.com";
/// Stagenet node REST API
pub const STAGENET_URL: &str = "https://nodes-stagenet.wavesnodes.com";
/// Local node REST API
pub const LOCAL_URL: &str = "http://127.0.0.1:6869";

/// [`Node`] client for executing asynchronous requests.
///
/// [`Node`] client has url as the configuration value, but the default is set to what is usually the most commonly desired value. Use [`Node::from_url()`] to create the node client.
pub struct Node<'a> {
    url: &'a str,
}

impl<'a> Default for Node<'a> {
    fn default() -> Self {
        Node { url: MAINNET_URL }
    }
}

impl<'a> Node<'a> {
    /// Create an [`Node`] from url string.
    pub fn from_url(url: &'a str) -> Self {
        Node { url }
    }

    /// Get the regular balance in WAVES at a given address
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    /// use wavesplatform::util::Amount;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node
    ///         .get_balance("3PEktVux2RhchSN63DsDo4b4mz4QqzKSeDv")
    ///         .await?;
    ///
    ///     let balance = Amount::from_wavelet(result.balance());
    ///
    ///     println!("Balance: {} WAVES", balance);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_balance(
        &self,
        address: &str,
    ) -> Result<ResponseBalance, Box<dyn std::error::Error>> {
        let url = format!("{}/addresses/balance/{}", self.url, address);

        let res = reqwest::get(url).await?.json::<ResponseBalance>().await?;

        Ok(res)
    }

    /// Get the available, regular, generating, and effective balance
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    /// use wavesplatform::util::Amount;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node
    ///         .get_balance_details("3PEktVux2RhchSN63DsDo4b4mz4QqzKSeDv")
    ///         .await?;
    ///
    ///     let balance = Amount::from_wavelet(result.regular());
    ///
    ///     println!("Regular balance: {} WAVES", balance);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_balance_details(
        &self,
        address: &str,
    ) -> Result<ResponseBalanceDetails, Box<dyn std::error::Error>> {
        let url = format!("{}/addresses/balance/details/{}", self.url, address);

        let res = reqwest::get(url)
            .await?
            .json::<ResponseBalanceDetails>()
            .await?;

        Ok(res)
    }

    /// Get an address associated with a given alias.
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node.get_address_by_alias("vlzhr").await?;
    ///
    ///     println!("vlzhr -> {}", result.address());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_address_by_alias(
        &self,
        alias: &str,
    ) -> Result<ResponseAddress, Box<dyn std::error::Error>> {
        let url = format!("{}/alias/by-alias/{}", self.url, alias);

        let res = reqwest::get(url).await?.json::<ResponseAddress>().await?;

        Ok(res)
    }

    /// Get detailed information about given asset
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node
    ///         .get_assets_details("34N9YcEETLWn93qYQ64EsP1x89tSruJU44RrEMSXXEPJ")
    ///         .await?;
    ///
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_assets_details(
        &self,
        asset_id: &str,
    ) -> Result<ResponseAsset, Box<dyn std::error::Error>> {
        let url = format!("{}/assets/details/{}", self.url, asset_id);

        let res = reqwest::get(url).await?.json::<ResponseAsset>().await?;

        Ok(res)
    }

    /// Get headers of a given block
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node
    ///         .get_blocks_headers("3cBRMpKHjPNKUXkgGJNGAaPviY4LmE8urTwd4B2J8v9M")
    ///         .await?;
    ///
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_blocks_headers(
        &self,
        id: &str,
    ) -> Result<ResponseBlock, Box<dyn std::error::Error>> {
        let url = format!("{}/blocks/headers/{}", self.url, id);

        let res = reqwest::get(url).await?.json::<ResponseBlock>().await?;

        Ok(res)
    }

    /// Get headers of a given block
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node.get_blocks_headers_at_height(3341874).await?;
    ///
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_blocks_headers_at_height(
        &self,
        height: u64,
    ) -> Result<ResponseBlock, Box<dyn std::error::Error>> {
        let url = format!("{}/blocks/headers/at/{}", self.url, height);

        let res = reqwest::get(url).await?.json::<ResponseBlock>().await?;

        Ok(res)
    }

    /// Get the block at the current blockchain height
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node.get_blocks_last().await?;
    ///
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_blocks_last(&self) -> Result<ResponseBlock, Box<dyn std::error::Error>> {
        let url = format!("{}/blocks/last", self.url);

        let res = reqwest::get(url).await?.json::<ResponseBlock>().await?;

        Ok(res)
    }

    /// Get lease parameters by lease ID
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node
    ///         .get_leasing_info("YwVPf35VckF4Yu5XwF18P9VwWwfQVGAQmqDp4bpgtuV")
    ///         .await?;
    ///
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_leasing_info(
        &self,
        id: &str,
    ) -> Result<ResponseLease, Box<dyn std::error::Error>> {
        let url = format!("{}/leasing/info/{}", self.url, id);

        let res = reqwest::get(url).await?.json::<ResponseLease>().await?;

        Ok(res)
    }

    /// Get node version
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node.get_node_version().await?;
    ///
    ///     println!("Version: {}", result.version());
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_node_version(
        &self,
    ) -> Result<ResponseNodeVersion, Box<dyn std::error::Error>> {
        let url = format!("{}/node/version", self.url);

        let res = reqwest::get(url)
            .await?
            .json::<ResponseNodeVersion>()
            .await?;

        Ok(res)
    }

    /// Get a transaction by its ID
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node
    ///         .get_transactions_info("YwVPf35VckF4Yu5XwF18P9VwWwfQVGAQmqDp4bpgtuV")
    ///         .await?;
    ///
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_transactions_info(
        &self,
        id: &str,
    ) -> Result<ResponseTransaction, Box<dyn std::error::Error>> {
        let url = format!("{}/transactions/info/{}", self.url, id);

        let res = reqwest::get(url)
            .await?
            .json::<ResponseTransaction>()
            .await?;

        Ok(res)
    }

    /// Get transaction status by its ID
    /// ```no_run
    /// use wavesplatform::node::{Node, MAINNET_URL};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let node = Node::from_url(MAINNET_URL);
    ///
    ///     let result = node
    ///         .get_transactions_status("YwVPf35VckF4Yu5XwF18P9VwWwfQVGAQmqDp4bpgtuV")
    ///         .await?;
    ///
    ///     println!("{:?}", result);
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn get_transactions_status(
        &self,
        id: &str,
    ) -> Result<ResponseTransactionStatus, Box<dyn std::error::Error>> {
        let url = format!("{}/transactions/status/{}", self.url, id);

        let res = reqwest::get(url)
            .await?
            .json::<ResponseTransactionStatus>()
            .await?;

        Ok(res)
    }
}
