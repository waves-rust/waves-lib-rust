use account::{PublicKeyAccount, TESTNET};
use bytebuffer::Buffer;
use base58::*;

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

enum TransactionData<'a> {
    Issue { name: &'a str, description: &'a str, quantity: u64, decimals: u8, reissuable: bool, chain_id: u8 },
    Reissue { asset_id: &'a str, quantity: u64, reissuable: bool, chain_id: u8 },
    Burn { asset_id: &'a str, quantity: u64, chain_id: u8 },
    //    Transfer { },
    Lease { recipient: &'a str, amount: u64, chain_id: u8 },////str->Address
    LeaseCancel { lease_id: &'a str, chain_id: u8 },
    Alias { alias: &'a str },
}

pub struct Transaction<'a> {
    data: TransactionData<'a>,
    fee: u64,
    timestamp: u64,
    sender_public_key: &'a PublicKeyAccount,
    type_id: u8,
    version: u8,
}

pub struct ProvenTransaction<'a> {
    tx: &'a Transaction<'a>,
    proofs: Vec<Vec<u8>>
}

impl <'a> Transaction<'a> {
    pub fn new_lease(sender_public_key: &'a PublicKeyAccount, recipient: &'a str, amount: u64,
                     chain_id: u8, fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Lease { recipient, amount, chain_id },
            fee,
            timestamp,
            sender_public_key,
            type_id: LEASE,
            version: V2
        }
    }

    pub fn new_alias(sender_public_key: &'a PublicKeyAccount, alias: &'a str,
                     fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Alias { alias },
            fee,
            timestamp,
            sender_public_key,
            type_id: ALIAS,
            version: V2
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let buf = &mut Buffer::new();
        buf.byte(self.type_id).byte(self.version);
        match self.data {
            TransactionData::Issue { name, description, quantity, decimals, reissuable, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .array(name.as_bytes())
                    .array(description.as_bytes())
                    .long(quantity).byte(decimals).boolean(reissuable)
                    .long(self.fee).long(self.timestamp),
            TransactionData::Reissue { asset_id, quantity, reissuable, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .bytes(asset_id.from_base58().unwrap().as_slice())
                    .long(quantity).boolean(reissuable)
                    .long(self.fee).long(self.timestamp),
            TransactionData::Burn { asset_id, quantity, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .bytes(asset_id.from_base58().unwrap().as_slice()).long(quantity)
                    .long(self.fee).long(self.timestamp),
            TransactionData::Lease { recipient, amount, chain_id } =>
                buf.byte(0).bytes(self.sender_public_key.to_bytes()).recipient(chain_id, recipient)
                    .long(amount).long(self.fee).long(self.timestamp),
            TransactionData::LeaseCancel { lease_id, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .long(self.fee).long(self.timestamp)
                    .bytes(lease_id.from_base58().unwrap().as_slice()),
            TransactionData::Alias { alias } =>
                buf.bytes(self.sender_public_key.to_bytes()).array(alias.as_bytes())
                    .long(self.fee).long(self.timestamp),
        };
        Vec::from(buf.as_slice())
    }

    pub fn with_proofs(&'a self, proofs: Vec<Vec<u8>>) -> ProvenTransaction<'a> {
        ProvenTransaction { tx: self, proofs }
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use account::PrivateKeyAccount;
    use base58::*;
    use ed25519_dalek::*;

    #[test]
    fn test_sign_lease() {
        let mut sk = [0u8; SECRET_KEY_LENGTH];
        sk.copy_from_slice(&"25Um7fKYkySZnweUEVAn9RLtxN5xHRd7iqpqYSMNQEeT".from_base58().unwrap().as_slice());
        let mut pk = [0u8; PUBLIC_KEY_LENGTH];
        pk.copy_from_slice("GqpLEy65XtMzGNrsfj6wXXeffLduEt1HKhBfgJGSFajX".from_base58().unwrap().as_slice());

        let acc = PrivateKeyAccount::from_key_pair(sk, pk, TESTNET);
        let tx = Transaction::new_lease(&acc.1, "3MzZCGFyuxgC4ZmtKRS7vpJTs75ZXdkbp1K", 100000, 84, 100000, 1500000000000);
        let sig = acc.sign_bytes(&tx.to_bytes());
//        let ProvenTransaction { tx, proofs } = acc.sign_transaction(&tx);
//        let sig = proofs.get(0).unwrap();
        println!("sig = {}", sig.to_base58());
        assert!(sig.len() == SIGNATURE_LENGTH);
    }
}