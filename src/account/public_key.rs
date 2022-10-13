use crate::account::{secure_hash, Address, ADDRESS_LENGTH, ADDRESS_VERSION};

use base58::ToBase58;
use ed25519_dalek::PUBLIC_KEY_LENGTH;
use std::fmt;

/// An account possessing a public key. Using `PublicKeyAccount` you can get the address.
pub struct PublicKeyAccount(pub [u8; PUBLIC_KEY_LENGTH]);

impl PublicKeyAccount {
    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }

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
