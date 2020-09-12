use crate::transaction::{ProvenTransaction, Transaction};

use base58::*;
use blake2::digest::{Input, VariableOutput};
use blake2::Blake2b;
use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use ed25519_dalek::*;
use rand;
use rand::Rng;
use sha2::{Digest, Sha256, Sha512};
use sha3::Keccak256;

const ADDRESS_VERSION: u8 = 1;
const ADDRESS_LENGTH: usize = 26;

/// MAINNET chainID
pub const MAINNET: u8 = 'W' as u8;
/// TESTNET chainID
pub const TESTNET: u8 = 'T' as u8;
/// STAGENET chainID
pub const STAGENET: u8 = 'S' as u8;

/// An account possessing a address.
pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    pub fn chain_id(&self) -> u8 {
        self.0[1]
    }

    pub fn to_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_base58()
    }

    pub fn from_string(base58: &str) -> Address {
        let mut bytes = [0u8; ADDRESS_LENGTH];
        bytes.copy_from_slice(base58.from_base58().unwrap().as_slice()); ////map unwrap, handle bad length
        Address(bytes)
    }
}

/// An account possessing a public key. Using `PublicKeyAccount` you can get the address.
pub struct PublicKeyAccount(pub [u8; PUBLIC_KEY_LENGTH]);

impl PublicKeyAccount {
    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_base58()
    }

    pub fn to_address(&self, chain_id: u8) -> Address {
        let mut buf = [0u8; ADDRESS_LENGTH];
        buf[0] = ADDRESS_VERSION;
        buf[1] = chain_id;
        buf[2..22].copy_from_slice(&secure_hash(&self.0)[..20]);
        let checksum = &secure_hash(&buf[..22])[..4];
        buf[22..].copy_from_slice(checksum);
        Address(buf)
    }
}

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

    pub fn to_string(&self) -> String {
        self.0.to_base58()
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

static INITBUF: [u8; 32] = [
    0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
];

pub(crate) fn sign(message: &[u8], secret_key: &[u8; SECRET_KEY_LENGTH]) -> [u8; SIGNATURE_LENGTH] {
    let mut hash = Sha512::default();
    hash.input(&INITBUF);

    hash.input(secret_key);
    hash.input(message);

    let mut rand = rand::thread_rng();
    let mut rndbuf: Vec<u8> = vec![0; 64];
    (0..63).for_each(|i| rndbuf[i] = rand.gen::<u8>());
    hash.input(&rndbuf);

    let rsc = Scalar::from_hash(hash);
    let r = (&rsc * &constants::ED25519_BASEPOINT_TABLE)
        .compress()
        .to_bytes();

    let ed_pubkey = &constants::ED25519_BASEPOINT_POINT * &Scalar::from_bits(*secret_key);
    let pubkey = ed_pubkey.compress().to_bytes();

    hash = Sha512::default();
    hash.input(&r);
    hash.input(&pubkey);
    hash.input(message);
    let s = &(&Scalar::from_hash(hash) * &Scalar::from_bits(*secret_key)) + &rsc;

    let sign = pubkey[31] & 0x80;
    let mut result = [0; SIGNATURE_LENGTH];
    result[..32].copy_from_slice(&r);
    result[32..].copy_from_slice(&s.to_bytes());
    result[63] &= 0x7F; // should be zero already, but just in case
    result[63] |= sign;
    result
}

pub(crate) fn blake_hash(message: &[u8]) -> Vec<u8> {
    ////mv where?
    let mut blake = <Blake2b as VariableOutput>::new(32).unwrap();
    blake.process(message);
    let mut buf = [0u8; 32];
    blake.variable_result(&mut buf).unwrap().to_vec()
}

pub(crate) fn secure_hash(message: &[u8]) -> Vec<u8> {
    ////mv where? return [u8]?
    Keccak256::digest(&blake_hash(message)).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashes() {
        let blake_in = "blake".as_bytes();
        let blake_out = "HRFQW3JNhUYcYXyKZJ1ZefKDhZkLKJk1dzzy3PzYPr3y"
            .from_base58()
            .unwrap();
        assert_eq!(blake_hash(blake_in), blake_out.as_slice());

        let secure_in = "baffled bobcat's been beaten by black bears".as_bytes();
        let secure_out = "4FSFJanCrKoB15fYmjs3FzhPLNyMj3i7xJjtynbyZtm8"
            .from_base58()
            .unwrap();
        assert_eq!(secure_hash(secure_in), secure_out.as_slice());
    }

    #[test]
    fn test_private_key_from_seed() {
        let PrivateKeyAccount(sk, acc) = PrivateKeyAccount::from_seed("test");
        assert_eq!(
            sk,
            "CuedBd7a6vBC6XXpatEj4S9ZoquLYPB7Ud17b69msZkt"
                .from_base58()
                .unwrap()
                .as_slice()
        );
        assert_eq!(
            acc.to_bytes(),
            "Cq5itmx4wbYuogySAoUp58MimLLkQrFFLr1tpJy2BYp1"
                .from_base58()
                .unwrap()
                .as_slice()
        );
        assert_eq!(
            acc.to_address(TESTNET).0,
            "3MzGEv9wnaqrYFYujAXSH5RQfHaVKNQvx3D"
                .from_base58()
                .unwrap()
                .as_slice()
        );
    }

    #[test]
    fn test_key_pair_to_string() {
        let account = PrivateKeyAccount::from_seed("test");

        assert_eq!(
            account.to_string(),
            "CuedBd7a6vBC6XXpatEj4S9ZoquLYPB7Ud17b69msZkt"
        );

        assert_eq!(
            account.public_key().to_string(),
            "Cq5itmx4wbYuogySAoUp58MimLLkQrFFLr1tpJy2BYp1"
        );

        assert_eq!(
            account.public_key().to_address(TESTNET).to_string(),
            "3MzGEv9wnaqrYFYujAXSH5RQfHaVKNQvx3D"
        );
    }
}
