use bytebuffer::Buffer;

use base58::*;
use curve25519_dalek::constants;
use curve25519_dalek::montgomery::MontgomeryPoint;
use curve25519_dalek::scalar::Scalar;
use ed25519_dalek::*;
use rand;
use rand::Rng;
use blake2::Blake2b;
use blake2::digest::{Input, VariableOutput};
use sha2::{Digest, Sha512};
use sha3::Keccak256;

const ADDRESS_VERSION: u8 = 1;
const ADDRESS_LENGTH: usize = 26;
const HASH_LENGTH: usize = 32;

pub const MAINNET: u8 = 'W' as u8;
pub const TESTNET: u8 = 'T' as u8;

pub struct Address([u8; ADDRESS_LENGTH]);

impl Address {
    pub fn to_bytes(&self) -> &[u8; ADDRESS_LENGTH] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_base58()
    }
}

pub struct PublicKeyAccount([u8; PUBLIC_KEY_LENGTH], u8);

impl PublicKeyAccount {
    pub fn to_bytes(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        &self.0
    }

    pub fn to_address(&self) -> Address {
        let mut buf= [0u8; ADDRESS_LENGTH];
        buf[0] = ADDRESS_VERSION;
        buf[1] = self.1;
        buf[2..22].copy_from_slice(&secure_hash(&self.0)[..20]);
        let checksum = &secure_hash(&buf[..22])[..4];
        buf[22..].copy_from_slice(checksum);
        Address(buf)
    }
}

pub struct PrivateKeyAccount([u8; SECRET_KEY_LENGTH], pub PublicKeyAccount);

impl PrivateKeyAccount {
    pub fn from_key_pair(sk: [u8; SECRET_KEY_LENGTH], pk: [u8; PUBLIC_KEY_LENGTH], chain_id: u8) -> PrivateKeyAccount {
        PrivateKeyAccount(sk, PublicKeyAccount(pk, chain_id))
    }

    pub fn sign_bytes(&self, data: &[u8]) -> [u8; SIGNATURE_LENGTH] {
        sign(data, &self.0)
    }

//    fn sign_transaction<'a>(&self, tx: &'a Transaction<'a>) -> ProvenTransaction<'a> {
//        let signature = self.sign_bytes(&tx.to_bytes());
//        ProvenTransaction { tx, proofs: vec![signature.to_vec()] }
//    }
}

impl PrivateKeyAccount {
    fn public_key(&self) -> &[u8; PUBLIC_KEY_LENGTH] {
        self.1.to_bytes()
    }
}

static INITBUF: [u8; 32] = [
    0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

fn sign(message: &[u8], secret_key: &[u8; SECRET_KEY_LENGTH]) -> [u8; SIGNATURE_LENGTH] {
    let mut hash = Sha512::default();
    hash.input(&INITBUF);

    hash.input(secret_key);
    hash.input(message);

    let mut rand = rand::thread_rng();
    let mut rndbuf: Vec<u8> = vec![0; 64];
    (0..63).for_each(|i| rndbuf[i] = rand.gen::<u8>());
    hash.input(&rndbuf);

    let rsc = Scalar::from_hash(hash);
    let r = (&rsc * &constants::ED25519_BASEPOINT_TABLE).compress().to_bytes();

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

fn sig_verify(message: &[u8], public_key: &[u8; PUBLIC_KEY_LENGTH], signature: &[u8; SIGNATURE_LENGTH]) -> bool {
    let sign = signature[63] & 0x80;
    let mut sig = [0u8; SIGNATURE_LENGTH];
    sig.copy_from_slice(signature);
    sig[63] &= 0x7f;

    let mut ed_pubkey = MontgomeryPoint(*public_key).to_edwards(sign).unwrap().compress().to_bytes();
    ed_pubkey[31] &= 0x7F;  // should be zero already, but just in case
    ed_pubkey[31] |= sign;

    PublicKey::from_bytes(&ed_pubkey).unwrap()
        .verify::<Sha512>(message,&Signature::from_bytes(&sig).unwrap())
        .is_ok()
}

fn secure_hash(message: &[u8]) -> Vec<u8> {
    let mut blake = <Blake2b as VariableOutput>::new(32).unwrap();
    blake.process(message);
    let mut buf = [0u8; 32];
    blake.variable_result(&mut buf).unwrap();
    Keccak256::digest(&buf).to_vec()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sig_verify() {
        let msg = "bagira".as_bytes();
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice(
            "GqpLEy65XtMzGNrsfj6wXXeffLduEt1HKhBfgJGSFajX".from_base58().unwrap().as_slice());
        let mut sig = [0u8; SIGNATURE_LENGTH];
        sig.copy_from_slice(
            "62Nc9BbpuJziRuuXvnYttT8hfWXsUPH1kAUfc2fBhLeuCV5szWW7GGFRtqRxbQd92p8cDaHKfUqXdkwcefXSHdp7"
                .from_base58().unwrap().as_slice());
        assert!(sig_verify(msg, &pk, &sig));
    }

    #[test]
    fn test_sig_roundtrip() {
        let msg = "bagira".as_bytes();
        let mut sk = [0u8; SECRET_KEY_LENGTH];
        sk.copy_from_slice(&"25Um7fKYkySZnweUEVAn9RLtxN5xHRd7iqpqYSMNQEeT".from_base58().unwrap().as_slice());
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice("GqpLEy65XtMzGNrsfj6wXXeffLduEt1HKhBfgJGSFajX".from_base58().unwrap().as_slice());
        let sig = sign(msg, &sk);
        println!("sig = {}", sig.to_base58());
        assert!(sig_verify(msg, &pk, &sig));
    }

    #[test]
    fn test_secure_hash() {
        let msg = "baffled bobcat's been beaten by black bears".as_bytes();
        let expected = "4FSFJanCrKoB15fYmjs3FzhPLNyMj3i7xJjtynbyZtm8".from_base58().unwrap();
        assert_eq!(secure_hash(msg), expected.as_slice());
    }

    #[test]
    fn test_public_key_to_address() {
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice("GqpLEy65XtMzGNrsfj6wXXeffLduEt1HKhBfgJGSFajX".from_base58().unwrap().as_slice());
        let account = PublicKeyAccount(pk, TESTNET);
        let Address(addr) = account.to_address();
        assert_eq!(addr, "3N9gDFq8tKFhBDBTQxR3zqvtpXjw5wW3syA".from_base58().unwrap().as_slice())
    }
}
