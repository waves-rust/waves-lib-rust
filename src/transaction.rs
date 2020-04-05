use crate::account::{blake_hash, Address, PublicKeyAccount};
use crate::bytebuffer::Buffer;
use base58::*;

enum Type {
    Issue = 3,
    Transfer = 4,
    Reissue = 5,
    Burn = 6,
    Lease = 8,
    LeaseCancel = 9,
    Alias = 10,
    MassTransfer = 11,
    Data = 12,
    SetScript = 13,
    Sponsor = 14,
    SetAssetScript = 15,
}

enum Version {
    V1 = 1,
    V2 = 2,
}

const HASH_LENGTH: usize = 32;

pub type TransactionId = Hash;
pub type Asset = Hash;

pub struct Hash([u8; HASH_LENGTH]); //// converge w/ Address using macro

impl Hash {
    pub fn to_bytes(&self) -> [u8; HASH_LENGTH] {
        self.0
    }

    pub fn to_string(&self) -> String {
        self.0.to_base58()
    }

    pub fn new(bytes: [u8; HASH_LENGTH]) -> Hash {
        Hash(bytes)
    }

    pub fn from_string(base58: &str) -> Asset {
        let mut bytes = [0u8; HASH_LENGTH];
        bytes.copy_from_slice(base58.from_base58().unwrap().as_slice()); ////map unwrap, handle bad length
        Hash(bytes)
    }
}

pub enum DataEntry<'a> {
    Integer(&'a str, u64),
    Boolean(&'a str, bool),
    Binary(&'a str, &'a Vec<u8>),
    String(&'a str, &'a str),
}

pub enum TransactionData<'a> {
    Issue {
        name: &'a str,
        description: &'a str,
        quantity: u64,
        decimals: u8,
        reissuable: bool,
        chain_id: u8,
        script: Option<&'a [u8]>,
    },
    Transfer {
        recipient: &'a Address,
        asset: Option<&'a Asset>,
        amount: u64,
        fee_asset: Option<&'a Asset>,
        attachment: Option<&'a str>,
    },
    Reissue {
        asset: &'a Asset,
        quantity: u64,
        reissuable: bool,
        chain_id: u8,
    },
    Burn {
        asset: &'a Asset,
        quantity: u64,
        chain_id: u8,
    },
    Lease {
        recipient: &'a Address,
        amount: u64,
        chain_id: u8,
    },
    CancelLease {
        lease_id: &'a TransactionId,
        chain_id: u8,
    },
    Alias {
        alias: &'a str,
        chain_id: u8,
    },
    MassTransfer {
        asset: Option<&'a Asset>,
        transfers: Vec<(&'a Address, u64)>,
        attachment: Option<&'a str>,
    },
    Data {
        data: Vec<&'a DataEntry<'a>>,
    },
    SetScript {
        script: Option<&'a [u8]>,
        chain_id: u8,
    },
    Sponsor {
        asset: &'a Asset,
        rate: Option<u64>,
    },
    SetAssetScript {
        asset: &'a Asset,
        script: Option<&'a [u8]>,
        chain_id: u8,
    }
}

/// A transaction. Data specific to a particular transaction type are stored in the `data` field.
/// # Usage
/// ```
/// use acryl::account::{PrivateKeyAccount, TESTNET};
/// use acryl::transaction::*;
/// let account = PrivateKeyAccount::from_seed("seed");
/// let tx = Transaction::new_alias(&account.public_key(), "rhino", TESTNET, 100000, 1536000000000);
/// let signed_tx = account.sign_transaction(tx);
/// ```
pub struct Transaction<'a> {
    data: TransactionData<'a>,
    fee: u64,
    timestamp: u64,
    sender_public_key: &'a PublicKeyAccount,
    type_id: u8,
    version: u8,
}

use self::TransactionData::*;

impl<'a> Transaction<'a> {
    pub fn new_issue(
        sender_public_key: &'a PublicKeyAccount,
        name: &'a str,
        description: &'a str,
        quantity: u64,
        decimals: u8,
        reissuable: bool,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
        script: Option<&'a [u8]>,
    ) -> Transaction<'a> {
        Transaction {
            data: Issue {
                name,
                description,
                quantity,
                decimals,
                reissuable,
                chain_id,
                script,
            },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Issue as u8,
            version: Version::V2 as u8,
        }
    }

    pub fn new_transfer(
        sender_public_key: &'a PublicKeyAccount,
        recipient: &'a Address,
        asset: Option<&'a Asset>,
        amount: u64,
        fee_asset: Option<&'a Asset>,
        fee: u64,
        attachment: Option<&'a str>,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: Transfer {
                recipient,
                asset,
                amount,
                fee_asset,
                attachment,
            },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Transfer as u8,
            version: Version::V2 as u8,
        }
    }

    pub fn new_reissue(
        sender_public_key: &'a PublicKeyAccount,
        asset: &'a Asset,
        quantity: u64,
        reissuable: bool,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: Reissue {
                asset,
                quantity,
                reissuable,
                chain_id,
            },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Reissue as u8,
            version: Version::V2 as u8,
        }
    }

    pub fn new_burn(
        sender_public_key: &'a PublicKeyAccount,
        asset: &'a Asset,
        quantity: u64,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: Burn {
                asset,
                quantity,
                chain_id,
            },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Burn as u8,
            version: Version::V2 as u8,
        }
    }

    pub fn new_lease(
        sender_public_key: &'a PublicKeyAccount,
        recipient: &'a Address,
        amount: u64,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: Lease {
                recipient,
                amount,
                chain_id,
            },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Lease as u8,
            version: Version::V2 as u8,
        }
    }

    pub fn new_lease_cancel(
        sender_public_key: &'a PublicKeyAccount,
        lease_id: &'a TransactionId,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: CancelLease { lease_id, chain_id },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::LeaseCancel as u8,
            version: Version::V2 as u8,
        }
    }

    pub fn new_alias(
        sender_public_key: &'a PublicKeyAccount,
        alias: &'a str,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: Alias { alias, chain_id },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Alias as u8,
            version: Version::V2 as u8,
        }
    }

    pub fn new_mass_transfer(
        sender_public_key: &'a PublicKeyAccount,
        asset: Option<&'a Asset>,
        transfers: Vec<(&'a Address, u64)>,
        attachment: Option<&'a str>,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: MassTransfer {
                asset,
                transfers,
                attachment,
            },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::MassTransfer as u8,
            version: Version::V1 as u8,
        }
    }

    pub fn new_data(
        sender_public_key: &'a PublicKeyAccount,
        data: Vec<&'a DataEntry<'a>>,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: Data { data },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Data as u8,
            version: Version::V1 as u8,
        }
    }

    pub fn new_script(
        sender_public_key: &'a PublicKeyAccount,
        script: Option<&'a [u8]>,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: SetScript { script, chain_id },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::SetScript as u8,
            version: Version::V1 as u8,
        }
    }

    pub fn new_sponsor(
        sender_public_key: &'a PublicKeyAccount,
        asset: &'a Asset,
        rate: Option<u64>,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: Sponsor { asset, rate },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::Sponsor as u8,
            version: Version::V1 as u8,
        }
    }

    pub fn new_set_asset_script(
        sender_public_key: &'a PublicKeyAccount,
        asset: &'a Asset,
        script: Option<&'a [u8]>,
        chain_id: u8,
        fee: u64,
        timestamp: u64,
    ) -> Transaction<'a> {
        Transaction {
            data: SetAssetScript {
                asset,
                script,
                chain_id,
            },
            fee,
            timestamp,
            sender_public_key,
            type_id: Type::SetAssetScript as u8,
            version: Version::V1 as u8,
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Buffer::new();
        buf.byte(self.type_id).byte(self.version);
        match self.data {
            Issue {
                name,
                description,
                quantity,
                decimals,
                reissuable,
                chain_id,
                script,
            } => {
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes()).array(name.as_bytes())
                    .array(description.as_bytes())
                    .long(quantity)
                    .byte(decimals)
                    .boolean(reissuable)
                    .long(self.fee)
                    .long(self.timestamp);
                    match script {
                        Some(bytes) => buf.byte(1).array(bytes),
                        None => buf.byte(0),
                    }
            }
            Transfer {
                recipient,
                ref asset,
                amount,
                ref fee_asset,
                attachment,
            } => buf
                .bytes(self.sender_public_key.to_bytes())
                .asset_opt(&asset)
                .asset_opt(&fee_asset)
                .long(self.timestamp)
                .long(amount)
                .long(self.fee)
                .recipient(recipient.chain_id(), &recipient.to_string())
                .array_opt(attachment.map(|s| s.as_bytes())),
            Reissue {
                asset,
                quantity,
                reissuable,
                chain_id,
            } => buf
                .byte(chain_id)
                .bytes(self.sender_public_key.to_bytes())
                .asset(&asset)
                .long(quantity)
                .boolean(reissuable)
                .long(self.fee)
                .long(self.timestamp),
            Burn {
                asset,
                quantity,
                chain_id,
            } => buf
                .byte(chain_id)
                .bytes(self.sender_public_key.to_bytes())
                .asset(&asset)
                .long(quantity)
                .long(self.fee)
                .long(self.timestamp),
            Lease {
                recipient,
                amount,
                chain_id,
            } => buf
                .byte(0)
                .bytes(self.sender_public_key.to_bytes())
                .recipient(chain_id, &recipient.to_string())
                .long(amount)
                .long(self.fee)
                .long(self.timestamp),
            CancelLease { lease_id, chain_id } => buf
                .byte(chain_id)
                .bytes(self.sender_public_key.to_bytes())
                .long(self.fee)
                .long(self.timestamp)
                .bytes(&lease_id.to_bytes()),
            Alias { alias, chain_id } => buf
                .bytes(self.sender_public_key.to_bytes())
                .size(alias.len() + 4)
                .byte(2)
                .byte(chain_id)
                .array(alias.as_bytes())
                .long(self.fee)
                .long(self.timestamp),
            MassTransfer {
                ref asset,
                ref transfers,
                attachment,
            } => {
                buf.bytes(self.sender_public_key.to_bytes())
                    .asset_opt(&asset)
                    .size(transfers.len());
                for (addr, amt) in transfers {
                    buf.bytes(addr.to_bytes()).long(*amt);
                }
                buf.long(self.timestamp)
                    .long(self.fee)
                    .array_opt(attachment.map(|s| s.as_bytes()))
            }
            Data { ref data } => {
                buf.bytes(self.sender_public_key.to_bytes())
                    .size(data.len());
                for e in data {
                    buf.data_entry(e);
                }
                buf.long(self.timestamp).long(self.fee)
            }
            SetScript { script, chain_id } => {
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes());
                match script {
                    Some(bytes) => buf.byte(1).array(bytes),
                    None => buf.byte(0),
                };
                buf.long(self.fee).long(self.timestamp)
            }
            Sponsor { asset, rate } => buf
                .bytes(self.sender_public_key.to_bytes())
                .asset(asset)
                .long(rate.unwrap_or(0))
                .long(self.fee)
                .long(self.timestamp),
            SetAssetScript {
                asset,
                script,
                chain_id,
            } => {
                buf.byte(chain_id).bytes(self.sender_public_key.to_bytes()).asset(asset);
                match script {
                    Some(bytes) => buf.byte(1).array(bytes),
                    None => buf.byte(0),
                };
                buf.long(self.fee).long(self.timestamp)
            }
        };
        Vec::from(buf.as_slice())
    }

    /// Returns transaction ID
    pub fn id(&self) -> TransactionId {
        let bytes = match self.data {
            Alias { alias, chain_id } => {
                let mut buf = Buffer::new();
                Vec::from(
                    buf.byte(self.type_id)
                        .byte(2)
                        .byte(chain_id)
                        .array(alias.as_bytes())
                        .as_slice(),
                )
            }
            _ => self.to_bytes(),
        };
        let mut id = [0u8; HASH_LENGTH];
        id.copy_from_slice(&blake_hash(&bytes));
        TransactionId::new(id)
    }

    /// Returns a ProvenTransaction with the given proofs
    pub fn with_proofs(self, proofs: Vec<Vec<u8>>) -> ProvenTransaction<'a> {
        ProvenTransaction { tx: self, proofs }
    }
}

/// Transaction with proofs. Proofs are byte vectors at most 64 bytes long, and maximum number of
/// proofs is 8.
pub struct ProvenTransaction<'a> {
    pub tx: Transaction<'a>,
    pub proofs: Vec<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::{Address, PrivateKeyAccount, TESTNET};
    use ed25519_dalek::*;

    #[test]
    fn test_tx_ids() {
        let pk = PublicKeyAccount([1u8; 32]);
        let asset = Asset::new([2u8; 32]);
        let lease = TransactionId::new([3u8; 32]);
        let recipient = Address::from_string("3JD4thvVVmzRRynaYHbdnUpv9QF7b1bFf4Y");
        let fee = 100000;
        let ts: u64 = 1536000000000;

        fn check_hash(tx: &Transaction, hash: &str) -> () {
            assert_eq!(tx.id().0, hash.from_base58().unwrap().as_slice());
        }

        check_hash(
            &Transaction::new_issue(
                &pk, "coin", "coin", 100000000, 8, false, TESTNET, 100000, ts, None,
            ),
            "AkgNv2ULydQSFSaxrDj2ufsZmMfd8qD6VGNKsSiPZwxC",
        );
        check_hash(
            &Transaction::new_reissue(&pk, &asset, 100000000, false, TESTNET, fee, ts),
            "8KC9vS4WswhfccZygjZVHVzsgo8HtuuxmFCAa6BQvmQK",
        );
        check_hash(
            &Transaction::new_burn(&pk, &asset, 100000000, TESTNET, fee, ts),
            "8kwwFWMKuPbR5bi8sf27dQegifxRNXjVx89URgMjYAjJ",
        );
        check_hash(
            &Transaction::new_transfer(
                &pk,
                &recipient,
                Some(&asset),
                10,
                None,
                fee,
                Some("atta ch me"),
                ts,
            ),
            "4xKPSytfy69czvTJdHH1tcduvkjNP6CY7Zcqpz3mgZPd",
        );
        check_hash(
            &Transaction::new_lease(&pk, &recipient, 10, TESTNET, fee, ts),
            "GWz6Wyc7dMbLv6cAHrdMeMYUhq4iNAPiTCnJu9jz2cL8",
        );
        check_hash(
            &Transaction::new_lease_cancel(&pk, &lease, TESTNET, fee, ts),
            "7KjBpRnNWEWW4gVcQWrMa9k9qydvYCohHR8ZFMJBfkaq",
        );
        let alias = Transaction::new_alias(&pk, "lilias", TESTNET, fee, ts);
        check_hash(&alias, "FRTvY7Amme8czANsqa7Gj3CM1bM4MtDeFuoLx8JL8bwu");

        let transfers = vec![(&recipient, 10), (&recipient, 10)];
        check_hash(
            &Transaction::new_mass_transfer(
                &pk,
                Some(&asset),
                transfers,
                Some("mass trans"),
                fee,
                ts,
            ),
            "53mKjWQW8iHugRAvUfVpptj4czehfTRfEN1Uw2hxe3JN",
        );

        let arr = vec![4u8; 32];
        let bin_entry = DataEntry::Binary("bin", &arr);
        let data = vec![
            &DataEntry::Integer("int", 1),
            &DataEntry::Boolean("bool", true),
            &bin_entry,
            &DataEntry::String("str", "str"),
        ];
        check_hash(
            &Transaction::new_data(&pk, data, fee, ts),
            "6fGLB7yxzkWPBb4fv32Fs7d5si6xifenj69Da9yHvwgx",
        );

        let script = vec![1, 6, 183, 111, 203, 71];
        check_hash(
            &Transaction::new_script(&pk, Some(script.as_slice()), TESTNET, fee, ts),
            "9hAcaE3yDMMbwXMcZpdANC9YNXPgqK6Exyqd5U9jXhUq",
        );
        check_hash(
            &Transaction::new_script(&pk, None, TESTNET, fee, ts),
            "AphhKVVhyBH2SFYTaSzWbGL7NPHZ1F9RUkFN4uhMZ9QE",
        );
        check_hash(
            &Transaction::new_sponsor(&pk, &asset, Some(100), fee, ts),
            "9zmHx3fyXz7pW6bRazPP28PGjnM8XjoHuyjzXCMHE2PY",
        );
    }

    #[test]
    fn test_sign() {
        let sender = PrivateKeyAccount::from_seed("seed");
        let recipient = Address::from_string("3MzZCGFyuxgC4ZmtKRS7vpJTs75ZXdkbp1K");
        let tx = Transaction::new_lease(&sender.1, &recipient, 100000, 84, 100000, 1500000000000);

        let ProvenTransaction { tx, proofs } = sender.sign_transaction(tx);
        assert_eq!(proofs.len(), 1);
        let sig = proofs.get(0).unwrap();
        assert_eq!(sig.len(), SIGNATURE_LENGTH);

        let ProvenTransaction { tx: _, proofs } = tx.with_proofs(vec![vec![1, 2, 3]]);
        assert_eq!(proofs.len(), 1);
        let sig = proofs.get(0).unwrap();
        assert_eq!(*sig, vec![1, 2, 3]);
    }
}
