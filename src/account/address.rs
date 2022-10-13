use crate::account::ADDRESS_LENGTH;

use base58::{FromBase58, ToBase58};
use std::fmt;

/// An account possessing a address.
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    pub fn chain_id(&self) -> u8 {
        self.0[1]
    }

    pub fn to_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }

    pub fn from_string(base58: &str) -> Address {
        let mut bytes = [0u8; ADDRESS_LENGTH];
        bytes.copy_from_slice(base58.from_base58().unwrap().as_slice()); ////map unwrap, handle bad length
        Address(bytes)
    }

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
