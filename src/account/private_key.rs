use crate::account::{secure_hash, sign, PublicKeyAccount};
use crate::transaction::{ProvenTransaction, Transaction};

use base58::ToBase58;
use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH};
use sha2::{Digest, Sha256};
use std::fmt;

/// An account possessing a private key. `PrivateKeyAccount` is tied to an address and can sign transactions.
///
/// Waves uses an asymmetric cryptographic system based on the elliptic curve Curve25519-ED25519 with X25519 keys.
///
/// Unlike centralized applications, users do not have usernames and passwords on the blockchain. User identification and validation of their actions are performed using a cryptographically bound key pair:
///
/// * The private key is used to sign transactions or orders.
/// * The public key allows to verify the digital signature.
///
/// Example private key in base58:
/// ```plain_text
/// 6yCStrsBs4VgTmYcSgF37pmQhCo6t9LZk5bQqUyUNSAs
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
pub struct PrivateKeyAccount([u8; SECRET_KEY_LENGTH], pub PublicKeyAccount);

impl PrivateKeyAccount {
    /// Get [`PublicKeyAccount`] struct.
    pub fn public_key(&self) -> &PublicKeyAccount {
        &self.1
    }

    /// Gets the internal byte value of [`PrivateKeyAccount`].
    pub fn private_key(&self) -> [u8; SECRET_KEY_LENGTH] {
        self.0
    }

    /// Create an [`PrivateKeyAccount`] from internal byte values.
    pub fn from_key_pair(
        sk: [u8; SECRET_KEY_LENGTH],
        pk: [u8; PUBLIC_KEY_LENGTH],
    ) -> PrivateKeyAccount {
        PrivateKeyAccount(sk, PublicKeyAccount(pk))
    }

    /// Create an [`PrivateKeyAccount`] from seed string.
    pub fn from_seed(seed: &str) -> PrivateKeyAccount {
        let seed_bytes = seed.as_bytes().to_vec();
        let nonce = [0, 0, 0, 0].to_vec();

        let mut sk = [0u8; SECRET_KEY_LENGTH];

        let acc_seed = secure_hash([nonce, seed_bytes].concat().as_slice());
        let hash_seed = &Sha256::digest(acc_seed.as_slice());

        sk.copy_from_slice(hash_seed);
        sk[0] &= 248;
        sk[31] &= 127;
        sk[31] |= 64;

        let ed_pk = &Scalar::from_bits(sk) * &constants::ED25519_BASEPOINT_TABLE;
        let pk = ed_pk.to_montgomery().to_bytes();
        PrivateKeyAccount(sk, PublicKeyAccount(pk))
    }

    /// Signs internal byte values.
    pub fn sign_bytes(&self, data: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        sign(data, &self.0)
    }

    /// Signs [`Transaction`] struct.
    pub fn sign_transaction<'a>(&self, tx: Transaction<'a>) -> ProvenTransaction<'a> {
        let signature = self.sign_bytes(&tx.to_bytes());
        ProvenTransaction {
            tx,
            proofs: vec![signature.to_vec()],
        }
    }
}

impl fmt::Display for PrivateKeyAccount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_base58())
    }
}
