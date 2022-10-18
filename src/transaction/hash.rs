use base58::{FromBase58, ToBase58};
use std::fmt;

/// The Transaction ID is a [`struct@Hash`]
pub type TransactionId = Hash;
/// The Asset ID is a [`struct@Hash`]
pub type Asset = Hash;

pub(crate) const HASH_LENGTH: usize = 32;

/// The [`struct@Hash`] struct is a representation of hashes.
///
/// Example hash:
/// ```plain_text
/// YwVPf35VckF4Yu5XwF18P9VwWwfQVGAQmqDp4bpgtuV
/// ```
#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Hash([u8; HASH_LENGTH]); //// converge w/ Address using macro

impl Hash {
    /// Decode an [`struct@Hash`] from base58 to an inner byte value.
    pub fn to_bytes(&self) -> [u8; HASH_LENGTH] {
        self.0
    }

    /// Create an [`struct@Hash`] from inner byte value.
    pub fn new(bytes: [u8; HASH_LENGTH]) -> Hash {
        Hash(bytes)
    }

    /// Create an [`struct@Hash`] from the base58 string.
    pub fn from_string(base58: &str) -> Asset {
        let mut bytes = [0u8; HASH_LENGTH];
        bytes.copy_from_slice(base58.from_base58().unwrap().as_slice()); ////map unwrap, handle bad length
        Hash(bytes)
    }
}

impl fmt::Display for Hash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_base58())
    }
}
