mod alias;
mod amount;

use curve25519_dalek::montgomery::MontgomeryPoint;
use ed25519_dalek::*;

pub use alias::*;
pub use amount::*;

/// Signature verify function
pub fn sig_verify(
    message: &[u8],
    public_key: &[u8; PUBLIC_KEY_LENGTH],
    signature: &[u8; SIGNATURE_LENGTH],
) -> bool {
    let sign = signature[63] & 0x80;
    let mut sig = [0u8; SIGNATURE_LENGTH];
    sig.copy_from_slice(signature);
    sig[63] &= 0x7f;

    let mut ed_pubkey = MontgomeryPoint(*public_key)
        .to_edwards(sign)
        .unwrap()
        .compress()
        .to_bytes();
    ed_pubkey[31] &= 0x7F; // should be zero already, but just in case
    ed_pubkey[31] |= sign;

    VerifyingKey::from_bytes(&ed_pubkey)
        .unwrap()
        .verify(message, &Signature::from_bytes(&sig))
        .is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::*;
    use base58::*;

    #[test]
    fn test_sig_roundtrip() {
        let msg = "uncle".as_bytes();
        let mut sk = [0u8; SECRET_KEY_LENGTH];
        sk.copy_from_slice(
            &"6zFSymZAoaua3gtJPbAUwM584tRETdKYdEG9BeEnZaGW"
                .from_base58()
                .unwrap()
                .as_slice(),
        );
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice(
            "4KxUVD9NtyRJjU3BCvPgJSttoJX7cb3DMdDTNucLN121"
                .from_base58()
                .unwrap()
                .as_slice(),
        );
        let sig = sign(msg, &sk);
        println!("sig = {}", sig.to_base58());
        assert!(sig_verify(msg, &pk, &sig));
    }

    #[test]
    fn test_sig_verify() {
        let msg = "uncle".as_bytes();
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice(
            "4KxUVD9NtyRJjU3BCvPgJSttoJX7cb3DMdDTNucLN121"
                .from_base58()
                .unwrap()
                .as_slice(),
        );
        let mut sig = [0u8; SIGNATURE_LENGTH];
        sig.copy_from_slice(
            "B4ViRpS6wZ73hhTtP4hhrfV46rR3uoUn7jgsH5yfkKMpbJUxMmu48jf3QSdibRkQBN7Tkx9jReKDq1Rmp9acxPG"
                .from_base58().unwrap().as_slice());
        assert!(sig_verify(msg, &pk, &sig));
    }
}
