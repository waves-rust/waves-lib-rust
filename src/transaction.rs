use account::{Address, PublicKeyAccount, TESTNET};
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

const ASSET_LENGTH: usize = 32;

pub struct Asset([u8; ASSET_LENGTH]);

impl Asset {//// converge w/ Address
    pub fn to_bytes(&self) -> &[u8; ASSET_LENGTH] {
        &self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_base58()
    }

    pub fn from_string(base58: &str) -> Asset {
        let mut bytes = [0u8; ASSET_LENGTH];
        bytes.copy_from_slice(base58.from_base58().unwrap().as_slice());////map unwrap, handle bad length
        Asset(bytes)
    }
}

enum TransactionData<'a> {
    Issue { name: &'a str, description: &'a str, quantity: u64, decimals: u8, reissuable: bool, chain_id: u8 },
    Reissue { asset_id: &'a Asset, quantity: u64, reissuable: bool, chain_id: u8 },
    Burn { asset_id: &'a Asset, quantity: u64, chain_id: u8 },
//    Transfer { recipient: &'a Address, asset_id: Option<Asset>, amount: u64,
//               fee_asset_id: Option<Asset>, attachment: Option<&'a [u8]> },
    Lease { recipient: &'a Address, amount: u64, chain_id: u8 },
    LeaseCancel { lease_id: &'a str, chain_id: u8 },////str->TxId
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
    pub tx: &'a Transaction<'a>,////encapsulate
    pub proofs: Vec<Vec<u8>>
}

impl <'a> Transaction<'a> {
    pub fn new_lease(sender_public_key: &'a PublicKeyAccount, recipient: &'a Address, amount: u64,
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
                    .bytes(asset_id.to_bytes()).long(quantity).boolean(reissuable)
                    .long(self.fee).long(self.timestamp),
            TransactionData::Burn { asset_id, quantity, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .bytes(asset_id.to_bytes()).long(quantity)
                    .long(self.fee).long(self.timestamp),
//            TransactionData::Transfer { recipient, ref asset_id, amount, ref fee_asset_id, attachment } =>
//                buf.bytes(self.sender_public_key.to_bytes())
//                    .asset_id(asset_id.map(|id|id.to_bytes()))
//                    .asset_id(fee_asset_id.map(|id|id.to_bytes()))
//                    .long(self.timestamp).long(amount).long(self.fee)
//                    .recipient(recipient.chain_id(), &recipient.to_string())
//                    .array(attachment.unwrap_or(&[])),
            TransactionData::Lease { recipient, amount, chain_id } =>
                buf.byte(0).bytes(self.sender_public_key.to_bytes())
                    .recipient(chain_id, &recipient.to_string())
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

        let sender = PrivateKeyAccount::from_key_pair(sk, pk);
        let recipient = Address::from_string("3MzZCGFyuxgC4ZmtKRS7vpJTs75ZXdkbp1K");
        let tx = Transaction::new_lease(&sender.1, &recipient, 100000, 84, 100000, 1500000000000);

        let ProvenTransaction { tx, proofs } = sender.sign_transaction(&tx);
        assert_eq!(proofs.len(), 1);
        let sig = proofs.get(0).unwrap();
        assert_eq!(sig.len(), SIGNATURE_LENGTH);

        let ProvenTransaction { tx, proofs } = tx.with_proofs(vec![vec![1, 2, 3]]);
        assert_eq!(proofs.len(), 1);
        let sig = proofs.get(0).unwrap();
        assert_eq!(*sig, vec![1, 2, 3]);
    }
}