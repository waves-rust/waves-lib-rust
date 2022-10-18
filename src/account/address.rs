use crate::account::ADDRESS_LENGTH;

use base58::{FromBase58, ToBase58};
use std::fmt;

/// An account possessing a address.
///
/// Address is an account attribute derived from the public key. The address also contains the chain ID that identifies the blockchain network, therefore the address on the Mainnet cannot be used on the Testnet and vice versa.
///
/// The address is a 26 byte array (see the Address Binary Format). In UIs the address is displayed as base58 encoded string.
///
/// Example address:
/// ```plain_text
/// 3PDfnPknnYrg2k2HMvkNLDb3Y1tDTtEnp9X
/// ```
///
/// Normally, the address starting with 3P refers to the Mainnet, and the address starting with 3M or 3N refers to Testnet or Stagenet.
///
/// # Usage
/// ```
/// use wavesplatform::account::{PrivateKeyAccount, TESTNET};
/// let account = PrivateKeyAccount::from_seed("seed");
/// println!(
///     "My TESTNET address: {}",
///     account.public_key().to_address(TESTNET).to_string()
/// );
/// ```
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    /// Get chain ID.
    pub fn chain_id(&self) -> u8 {
        self.0[1]
    }

    /// Decode an [`Address`] from base58 to an inner byte value.
    pub fn to_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }

    /// Create an [`Address`] from the base58 string.
    pub fn from_string(base58: &str) -> Address {
        let mut bytes = [0u8; ADDRESS_LENGTH];
        bytes.copy_from_slice(base58.from_base58().unwrap().as_slice()); ////map unwrap, handle bad length
        Address(bytes)
    }

    /// Create an [`Address`] from inner byte value.
    pub fn from_bytes(buffer: &[u8]) -> Address {
        let mut bytes = [0u8; ADDRESS_LENGTH];
        bytes.copy_from_slice(&buffer[..ADDRESS_LENGTH]);
        Address(bytes)
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_base58())
    }
}
