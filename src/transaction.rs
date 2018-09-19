use account::{Address, PublicKeyAccount, TESTNET, secure_hash};
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

const V1: u8 = 1;
const V2: u8 = 2;

const HASH_LENGTH: usize = 32;

pub type TransactionId = Hash;
pub type Asset = Hash;

pub struct Hash([u8; HASH_LENGTH]);//// converge w/ Address using macro

impl Hash {
    pub fn to_bytes(&self) -> [u8; HASH_LENGTH] {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_base58()
    }

    pub fn new(bytes: &[u8; HASH_LENGTH]) -> Hash {
        Hash(*bytes)
    }

    pub fn from_string(base58: &str) -> Asset {
        let mut bytes = [0u8; HASH_LENGTH];
        bytes.copy_from_slice(base58.from_base58().unwrap().as_slice());////map unwrap, handle bad length
        Hash(bytes)
    }
}

pub enum DataEntry<'a> {
    Integer(&'a str, u64),
    Boolean(&'a str, bool),
    Binary(&'a str, &'a [u8]),
    String(&'a str, &'a str)
}

pub enum TransactionData<'a> {
    Issue { name: &'a str, description: &'a str, quantity: u64, decimals: u8, reissuable: bool, chain_id: u8 },
    Reissue { asset: &'a Asset, quantity: u64, reissuable: bool, chain_id: u8 },
    Burn { asset: &'a Asset, quantity: u64, chain_id: u8 },
    Transfer { recipient: &'a Address, asset: Option<&'a Asset>, amount: u64,
               fee_asset: Option<&'a Asset>, attachment: Option<&'a [u8]> },
    Lease { recipient: &'a Address, amount: u64, chain_id: u8 },
    CancelLease { lease_id: &'a TransactionId, chain_id: u8 },
    Alias { alias: &'a str },
    MassTransfer { asset: Option<&'a Asset>, transfers: Vec<(Address, u64)>, attachment: Option<&'a [u8]> },
    Data { data: Vec<DataEntry<'a>> },
    SetScript { script: Option<&'a [u8]>, chain_id: u8 },
    Sponsor { asset: &'a Asset, rate: Option<u64> },
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
    pub tx: Transaction<'a>,
    pub proofs: Vec<Vec<u8>>
}

impl <'a> Transaction<'a> {
    pub fn new_issue(sender_public_key: &'a PublicKeyAccount, name: &'a str, description: &'a str,
                     quantity: u64, decimals: u8, reissuable: bool, chain_id: u8,
                     fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Issue { name, description, quantity, decimals, reissuable, chain_id },
            fee, timestamp, sender_public_key,
            type_id: ISSUE,
            version: V2
        }
    }

    pub fn new_reissue(sender_public_key: &'a PublicKeyAccount, asset: &'a Asset, quantity: u64,
                       reissuable: bool, chain_id: u8, fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Reissue { asset, quantity, reissuable, chain_id },
            fee, timestamp, sender_public_key,
            type_id: REISSUE,
            version: V2
        }
    }

    pub fn new_burn(sender_public_key: &'a PublicKeyAccount, asset: &'a Asset, quantity: u64,
                    chain_id: u8, fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Burn { asset, quantity, chain_id },
            fee, timestamp, sender_public_key,
            type_id: BURN,
            version: V2
        }
    }

    pub fn new_transfer(sender_public_key: &'a PublicKeyAccount, recipient: &'a Address,
                        asset: Option<&'a Asset>, amount: u64, fee_asset: Option<&'a Asset>, fee: u64,
                        attachment: Option<&'a [u8]>, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Transfer { recipient, asset, amount, fee_asset, attachment },
            fee, timestamp, sender_public_key,
            type_id: TRANSFER,
            version: V2
        }
    }

    pub fn new_lease(sender_public_key: &'a PublicKeyAccount, recipient: &'a Address, amount: u64,
                     chain_id: u8, fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Lease { recipient, amount, chain_id },
            fee, timestamp, sender_public_key,
            type_id: LEASE,
            version: V2
        }
    }

    pub fn new_lease_cancel(sender_public_key: &'a PublicKeyAccount, lease_id: &'a TransactionId,
                            chain_id: u8, fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::CancelLease { lease_id, chain_id },
            fee, timestamp, sender_public_key,
            type_id: LEASE_CANCEL,
            version: V2
        }
    }

    pub fn new_alias(sender_public_key: &'a PublicKeyAccount, alias: &'a str,
                     fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Alias { alias },
            fee, timestamp, sender_public_key,
            type_id: ALIAS,
            version: V2
        }
    }

    pub fn new_mass_transfer(sender_public_key: &'a PublicKeyAccount, asset: Option<&'a Asset>,
                             transfers: Vec<(Address, u64)>, attachment: Option<&'a [u8]>,
                             fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::MassTransfer { asset, transfers, attachment },
            fee, timestamp, sender_public_key,
            type_id: MASS_TRANSFER,
            version: V1
        }
    }

    pub fn new_data(sender_public_key: &'a PublicKeyAccount, data: Vec<DataEntry<'a>>,
                    fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Data { data },
            fee, timestamp, sender_public_key,
            type_id: DATA,
            version: V1
        }
    }

    pub fn new_script(sender_public_key: &'a PublicKeyAccount, script: Option<&'a [u8]>,
                      chain_id: u8, fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::SetScript { script, chain_id },
            fee, timestamp, sender_public_key,
            type_id: SET_SCRIPT,
            version: V1
        }
    }

    pub fn new_sponsor(sender_public_key: &'a PublicKeyAccount, asset: &'a Asset,
                       rate: Option<u64>, fee: u64, timestamp: u64) -> Transaction<'a> {
        Transaction {
            data: TransactionData::Sponsor { asset, rate },
            fee, timestamp, sender_public_key,
            type_id: SPONSOR,
            version: V1
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
            TransactionData::Reissue { asset, quantity, reissuable, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .asset(&asset).long(quantity).boolean(reissuable)
                    .long(self.fee).long(self.timestamp),
            TransactionData::Burn { asset, quantity, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .asset(&asset).long(quantity)
                    .long(self.fee).long(self.timestamp),
            TransactionData::Transfer { recipient, ref asset, amount, ref fee_asset, attachment } =>
                buf.bytes(self.sender_public_key.to_bytes())
                    .asset_opt(&asset).asset_opt(&fee_asset)
                    .long(self.timestamp).long(amount).long(self.fee)
                    .recipient(recipient.chain_id(), &recipient.to_string()).array_opt(attachment),
            TransactionData::Lease { recipient, amount, chain_id } =>
                buf.byte(0).bytes(self.sender_public_key.to_bytes())
                    .recipient(chain_id, &recipient.to_string())
                    .long(amount).long(self.fee).long(self.timestamp),
            TransactionData::CancelLease { lease_id, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .long(self.fee).long(self.timestamp).bytes(&lease_id.to_bytes()),
            TransactionData::Alias { alias } =>
                buf.bytes(self.sender_public_key.to_bytes()).array(alias.as_bytes())
                    .long(self.fee).long(self.timestamp),
            TransactionData::MassTransfer { ref asset, ref transfers, attachment } => {
                buf.bytes(self.sender_public_key.to_bytes()).asset_opt(&asset);
                for (addr, amt) in transfers {
                    buf.bytes(addr.to_bytes()).long(*amt);
                }
                buf.long(self.timestamp).long(self.fee).array_opt(attachment)
            },
            TransactionData::Data { ref data } => {
                buf.bytes(self.sender_public_key.to_bytes());
                for e in data {
                    buf.data_entry(e);
                }
                buf.long(self.timestamp).long(self.fee)
            },
            TransactionData::SetScript { script, chain_id } =>
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes())
                    .array_opt(script)
                    .long(self.fee).long(self.timestamp),
            TransactionData::Sponsor { asset, rate } =>
                buf.bytes(self.sender_public_key.to_bytes())
                    .asset(asset)
                    .long(rate.unwrap_or(0))
                    .long(self.fee).long(self.timestamp),
        };
        Vec::from(buf.as_slice())
    }

    pub fn id(&self) -> TransactionId {
        let mut id = [0u8; HASH_LENGTH];
        id.copy_from_slice(&secure_hash(&self.to_bytes()));
        Hash(id)
    }

    pub fn with_proofs(self, proofs: Vec<Vec<u8>>) -> ProvenTransaction<'a> {
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

        let ProvenTransaction { tx, proofs } = sender.sign_transaction(tx);
        assert_eq!(proofs.len(), 1);
        let sig = proofs.get(0).unwrap();
        assert_eq!(sig.len(), SIGNATURE_LENGTH);

        let ProvenTransaction { tx, proofs } = tx.with_proofs(vec![vec![1, 2, 3]]);
        assert_eq!(proofs.len(), 1);
        let sig = proofs.get(0).unwrap();
        assert_eq!(*sig, vec![1, 2, 3]);
    }
}