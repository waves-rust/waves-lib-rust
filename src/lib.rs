extern crate base58;
extern crate curve25519_dalek;
extern crate digest;
extern crate ed25519_dalek; // for LENGTH constants
extern crate rand;
extern crate sha2;

use base58::*;
use curve25519_dalek::constants;
use curve25519_dalek::montgomery::MontgomeryPoint;
use curve25519_dalek::scalar::Scalar;
use digest::Input;
use ed25519_dalek::*;
use rand::Rng;
use sha2::{Digest, Sha512};

const ISSUE: u8 = 3;
const TRANSFER: u8 = 4;
const REISSUE: u8 = 5;
const BURN: u8 = 6;
const LEASE: u8 = 8;
const LEASE_CANCEL: u8 = 9;
const ALIAS: u8 = 10;
const MASS_TRANSFER: u8 = 11;
const DATA: u8 = 12;
const SET_SCRIPT: u8 = 13;
const SPONSOR: u8 = 14;

const V2: u8 = 2;

fn sign_issue(secret_key: &[u8; SECRET_KEY_LENGTH], public_key: &[u8], chain_id: u8,
              name: &str, description: &str, quantity: u64, decimals: u8, reissuable: bool,
              script: &str, fee: u64, timestamp: u64) -> [u8; SIGNATURE_LENGTH] {
    sign_output(&|buf: &mut Buffer|
        buf.byte(ISSUE).byte(V2).byte(chain_id).bytes(public_key)
            .array(name.as_bytes())
            .array(description.as_bytes())
            .long(quantity).byte(decimals).boolean(reissuable)
            .long(fee).long(timestamp), secret_key)
}

fn sign_reissue(secret_key: &[u8; SECRET_KEY_LENGTH], public_key: &[u8], chain_id: u8,
                asset_id: &str, quantity: u64, reissuable: bool,
                fee: u64, timestamp: u64) -> [u8; SIGNATURE_LENGTH] {
    sign_output(&|buf: &mut Buffer|
        buf.byte(REISSUE).byte(V2).byte(chain_id).bytes(public_key)
            .bytes(asset_id.from_base58().unwrap().as_slice())
            .long(quantity).boolean(reissuable).long(fee).long(timestamp), secret_key)
}

fn sign_burn(secret_key: &[u8; SECRET_KEY_LENGTH], public_key: &[u8], chain_id: u8,
             asset_id: &str, quantity: u64, fee: u64, timestamp: u64) -> [u8; SIGNATURE_LENGTH] {
    sign_output(&|buf: &mut Buffer|
        buf.byte(BURN).byte(V2).byte(chain_id).bytes(public_key)
            .bytes(asset_id.from_base58().unwrap().as_slice())
            .long(quantity).long(fee).long(timestamp), secret_key)
}

//fn sign_transfer(secret_key: &[u8; SECRET_KEY_LENGTH], public_key: &[u8], recipient: &str,
//                 asset_id: Option<&[u8]>, amount: u64, fee_asset_id: Option<&[u8]>, fee: u64,
//                 attachment: &str, timestamp: u64) -> [u8; SIGNATURE_LENGTH] {
//    let mut buf = Buffer::new();
//    buf.byte(TRANSFER).byte(V2).bytes(public_key)
//        .asset_id(asset_id).asset_id(fee_asset_id)
//        .long(timestamp).long(amount).long(fee)
//        .recipient(recipient).array(attachment.as_bytes());
//    sign(buf.as_slice(), secret_key)
//}

fn sign_lease(secret_key: &[u8; SECRET_KEY_LENGTH], public_key: &[u8], chain_id: u8,
              recipient: &str, amount: u64, fee: u64, timestamp: u64) -> [u8; SIGNATURE_LENGTH] {
    sign_output(&|buf: &mut Buffer|
        buf.byte(LEASE).byte(V2).byte(0).bytes(public_key)
            .recipient(chain_id, recipient)
            .long(amount).long(fee).long(timestamp), secret_key)
}

fn sign_lease_cancel(secret_key: &[u8; SECRET_KEY_LENGTH], public_key: &[u8], chain_id: u8,
                     lease_id: &[u8], fee: u64, timestamp: u64) -> [u8; SIGNATURE_LENGTH] {
    sign_output(&|buf: &mut Buffer|
        buf.byte(LEASE_CANCEL).byte(V2).byte(chain_id).bytes(public_key)
            .long(fee).long(timestamp).bytes(lease_id), secret_key)
}

fn sign_alias(secret_key: &[u8; SECRET_KEY_LENGTH], public_key: &[u8], alias: &str,
              fee: u64, timestamp: u64) -> [u8; SIGNATURE_LENGTH] {
    sign_output(&|buf: &mut Buffer|
        buf.byte(ALIAS).byte(V2).bytes(public_key)
            .array(alias.as_bytes()).long(fee).long(timestamp), secret_key)
}

fn sign_output(writer: &Fn(&mut Buffer) -> &mut Buffer, secret_key: &[u8; SECRET_KEY_LENGTH]) -> [u8; SIGNATURE_LENGTH] {
    let mut buf = &mut Buffer::new();
    let tmp = writer(&mut buf);
    sign(tmp.as_slice(), secret_key)
}

static INITBUF: [u8; 32] = [
    0xfe, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
    0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff];

trait Flushable {
    fn flush(&self, sink: &mut Input) -> ();
}

struct Buffer {
    buf: Vec<u8>
}

impl Buffer {
    fn new() -> Buffer {
        Buffer { buf: Vec::new() }
    }

    fn from_bytes(b: &[u8]) -> Buffer {
        Buffer { buf: Vec::from(b) }
    }

    fn bytes(self: &mut Buffer, b: &[u8]) -> &mut Buffer {
        self.buf.extend_from_slice(b);
        self
    }

    fn byte(self: &mut Buffer, b: u8) -> &mut Buffer {
        self.buf.push(b);
        self
    }

    fn size(&mut self, n: usize) -> &mut Buffer {
        let bytes = [((n >> 8) & 0xff) as u8, (n & 0xff) as u8];
        self.bytes(&bytes)
    }

    fn long(&mut self, n: u64) -> &mut Buffer {
        let bytes = [
            ((n >> 56) & 0xff) as u8, ((n >> 48) & 0xff) as u8,
            ((n >> 40) & 0xff) as u8, ((n >> 32) & 0xff) as u8,
            ((n >> 24) & 0xff) as u8, ((n >> 16) & 0xff) as u8,
            ((n >> 8) & 0xff) as u8, (n & 0xff) as u8];
        self.bytes(&bytes)
    }

    fn boolean(&mut self, b: bool) -> &mut Buffer {
        let val = if b {1} else {0};
        self.buf.push(val);
        self
    }

    fn recipient(&mut self, chain_id: u8, recipient: &str) -> &mut Buffer {
        if recipient.len() <= 30 {
            // assume an alias
            self.byte(0x02).byte(chain_id).size(recipient.len()).bytes(&recipient.as_bytes())
        } else {
            self.bytes(&recipient.from_base58().unwrap().as_slice())
        }
    }

    fn array(&mut self, arr: &[u8]) -> &mut Buffer {
        self.size(arr.len()).bytes(arr)
    }

    fn asset_id(&mut self, asset_id: Option<&[u8]>) -> &mut Buffer {
        match asset_id {
            Some(id) => self.byte(1).bytes(id),
            None => self.byte(0)
        }
    }

    fn as_slice(&self) -> &[u8] {
        self.buf.as_slice()
    }
}

fn sign(message: &[u8], secret_key: &[u8; SECRET_KEY_LENGTH]) -> [u8; SIGNATURE_LENGTH] {
    let mut rand= rand::thread_rng();

    let mut hash = Sha512::default();
    hash.input(&INITBUF);

    hash.input(secret_key);
    hash.input(message);

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

#[cfg(test)]
mod tests {
    use super::*;
    use base58::*;

    #[test]
    fn test_signatures() {
        for _ in 1..50 {
            let msg: [u8; 32] = rand::thread_rng().gen();
            let mut buf = Buffer::from_bytes(&msg);
            let mut sk = [0u8; 32];
            sk.copy_from_slice(&"25Um7fKYkySZnweUEVAn9RLtxN5xHRd7iqpqYSMNQEeT".from_base58().unwrap().as_slice());
            let sig = sign(buf.as_slice(), &sk);
            println ! ("(\"{}\", \"{}\", \"{}\"),", msg.to_base58(), sk.to_base58(), sig.to_base58());
        }
        assert!(true);
    }

    #[test]
    fn test_signature() {
        let msg = "bagira".as_bytes();
        let mut buf = Buffer::from_bytes(msg);
        let mut sk = [0u8; SECRET_KEY_LENGTH];
        sk.copy_from_slice(&"25Um7fKYkySZnweUEVAn9RLtxN5xHRd7iqpqYSMNQEeT".from_base58().unwrap().as_slice());
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice("GqpLEy65XtMzGNrsfj6wXXeffLduEt1HKhBfgJGSFajX".from_base58().unwrap().as_slice());
        let sig = sign(buf.as_slice(), &sk);
        println!("sig = {}", sig.to_base58());
        assert!(sig_verify(msg, &pk, &sig))
    }

    #[test]
    fn test_verify() {
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
    fn test_sign_lease() {
        let mut sk = [0u8; SECRET_KEY_LENGTH];
        sk.copy_from_slice(&"25Um7fKYkySZnweUEVAn9RLtxN5xHRd7iqpqYSMNQEeT".from_base58().unwrap().as_slice());
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice("GqpLEy65XtMzGNrsfj6wXXeffLduEt1HKhBfgJGSFajX".from_base58().unwrap().as_slice());
        let sig = sign_lease(&sk, &pk, 84 as u8, "3MzZCGFyuxgC4ZmtKRS7vpJTs75ZXdkbp1K", 100000, 100000, 1500000000000);
        println!("sig = {}", sig.to_base58());
        assert!(sig.len() == 2);
    }
}
