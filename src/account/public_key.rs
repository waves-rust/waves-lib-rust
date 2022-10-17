use crate::account::{secure_hash, Address, ADDRESS_LENGTH, ADDRESS_VERSION};

use base58::ToBase58;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use std::fmt;

/// An account possessing a public key. Using `PublicKeyAccount` you can get the address.
///
/// Waves uses an asymmetric cryptographic system based on the elliptic curve Curve25519-ED25519 with X25519 keys.
///
/// Each transaction contains the public key of the sender account. The sender generates a digital signature of the transaction using the account's private key. The signature and the sender's public key are used to verify the authenticity of the transaction data and to check that the signature of the transaction matches the public key.
///
/// Example public key in base58:
/// ```plain_text
/// 5cqzmxsmFPBHm4tb7D8DMA7s5eutLXTDnnNMQKy2AYxh
/// ```
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
pub struct PublicKeyAccount(pub [u8; PUBLIC_KEY_LENGTH]);

impl PublicKeyAccount {
    /// Gets the internal byte value of [`PublicKeyAccount`].
    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }

    /// Converting [`PublicKeyAccount`] to [`Address`] struct
    pub fn to_address(&self, chain_id: u8) -> Address {
        let mut buf = [0u8; ADDRESS_LENGTH];
        buf[0] = ADDRESS_VERSION;
        buf[1] = chain_id;
        buf[2..22].copy_from_slice(&secure_hash(&self.0)[..20]);
        let checksum = &secure_hash(&buf[..22])[..4];
        buf[22..].copy_from_slice(checksum);
        Address::from_bytes(&buf)
    }
}

impl fmt::Display for PublicKeyAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_base58())
    }
}
