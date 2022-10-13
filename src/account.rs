mod address;
mod private_key;
mod public_key;

use blake2::digest::{Update, VariableOutput};
use blake2::VarBlake2b;
use curve25519_dalek::constants;
use curve25519_dalek::scalar::Scalar;
use ed25519_dalek::*;
use rand;
use rand::Rng;
use sha2::{Digest, Sha512};
use sha3::Keccak256;

pub use address::*;
pub use private_key::*;
pub use public_key::*;

const ADDRESS_VERSION: u8 = 1;
const ADDRESS_LENGTH: usize = 26;

/// MAINNET chainID
pub const MAINNET: u8 = b'W';
/// TESTNET chainID
pub const TESTNET: u8 = b'T';
/// STAGENET chainID
pub const STAGENET: u8 = b'S';

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
    let mut blake = VarBlake2b::new(32).unwrap();
    blake.update(message);
    let mut buf = [0u8; 32];
    blake.finalize_variable(|out| buf.copy_from_slice(out));
    buf.to_vec()
}

pub(crate) fn secure_hash(message: &[u8]) -> Vec<u8> {
    ////mv where? return [u8]?
    Keccak256::digest(&blake_hash(message)).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    use base58::FromBase58;

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
        // let PrivateKeyAccount(sk, acc) = PrivateKeyAccount::from_seed("test");
        let private_key_account = PrivateKeyAccount::from_seed("test");
        let sk = private_key_account.private_key();
        let acc = private_key_account.public_key();
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
            acc.to_address(TESTNET).to_bytes(),
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
