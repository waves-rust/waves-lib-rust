use curve25519_dalek::montgomery::MontgomeryPoint;
use ed25519_dalek::*;

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

    PublicKey::from_bytes(&ed_pubkey)
        .unwrap()
        .verify(message, &Signature::from_bytes(&sig).unwrap())
        .is_ok()
}
