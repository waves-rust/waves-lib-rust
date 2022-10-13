use crate::account::{secure_hash, sign, PublicKeyAccount};
use crate::transaction::{ProvenTransaction, Transaction};

use base58::ToBase58;
use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use ed25519_dalek::{PUBLIC_KEY_LENGTH, SECRET_KEY_LENGTH, SIGNATURE_LENGTH};
use sha2::{Digest, Sha256};
use std::fmt;

/// An account possessing a private key. `PrivateKeyAccount` is tied to an address and can sign
/// transactions.
/// # Usage
/// ```
/// use wavesplatform::account::{PrivateKeyAccount, TESTNET};
/// let account = PrivateKeyAccount::from_seed("seed");
/// println!("my address: {}", account.public_key().to_address(TESTNET).to_string());
/// ```
pub struct PrivateKeyAccount([u8; SECRET_KEY_LENGTH], pub PublicKeyAccount);

impl PrivateKeyAccount {
    pub fn public_key(&self) -> &PublicKeyAccount {
        &self.1
    }

    pub fn private_key(&self) -> [u8; SECRET_KEY_LENGTH] {
        self.0
    }

    pub fn from_key_pair(
        sk: [u8; SECRET_KEY_LENGTH],
        pk: [u8; PUBLIC_KEY_LENGTH],
    ) -> PrivateKeyAccount {
        PrivateKeyAccount(sk, PublicKeyAccount(pk))
    }

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

    pub fn sign_bytes(&self, data: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        sign(data, &self.0)
    }

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
