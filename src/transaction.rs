mod data_entry;
mod hash;
mod transaction_data;
mod type_id;
mod version;

use crate::account::{blake_hash, Address, PublicKeyAccount};
use crate::bytebuffer::Buffer;

pub use data_entry::*;
pub use hash::*;
pub use transaction_data::*;
pub use type_id::*;
pub use version::*;

/// Transaction data. Data specific to a particular transaction type are stored in the `data` field.
/// # Usage
/// ```
/// use wavesplatform::account::{PrivateKeyAccount, TESTNET};
/// use wavesplatform::transaction::*;
/// let account = PrivateKeyAccount::from_seed("seed");
/// let tx = Transaction::new_alias(
///     &account.public_key(),
///     "rhino",
///     TESTNET,
///     100000,
///     1536000000000,
/// );
/// let signed_tx = account.sign_transaction(tx);
/// ```
#[derive(Debug)]
pub struct Transaction<'a> {
    data: TransactionData<'a>,
    fee: u64,
    timestamp: u64,
    sender_public_key: &'a PublicKeyAccount,
    type_id: u8,
    version: u8,
}

use transaction_data::TransactionData::*;

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
                buf.byte(chain_id)
                    .bytes(self.sender_public_key.to_bytes())
                    .array(name.as_bytes())
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
                buf.byte(chain_id)
                    .bytes(self.sender_public_key.to_bytes())
                    .asset(asset);
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

    use base58::FromBase58;
    use ed25519_dalek::*;

    #[test]
    fn test_tx_ids() {
        let pk = PublicKeyAccount([1u8; 32]);
        let asset = Asset::new([2u8; 32]);
        let lease = TransactionId::new([3u8; 32]);
        let recipient = Address::from_string("3MzGEv9wnaqrYFYujAXSH5RQfHaVKNQvx3D");
        let fee = 100000;
        let ts: u64 = 1536000000000;

        fn check_hash(tx: &Transaction, hash: &str) -> () {
            assert_eq!(tx.id().to_bytes(), hash.from_base58().unwrap().as_slice());
        }

        check_hash(
            &Transaction::new_issue(
                &pk, "coin", "coin", 100000000, 8, false, TESTNET, 100000, ts, None,
            ),
            "GHqHz8xot1Yo7fivjPBYiqgtJNW3jR6bvpNh2WH66tEM",
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
            "E4Jc1vMh4TqryNzajU7onTHLLFkDmjNzo7aSedX4Rpad",
        );
        check_hash(
            &Transaction::new_reissue(&pk, &asset, 100000000, false, TESTNET, fee, ts),
            "83WaG6AAzxF3NFormpqrJr9Bi8eSdwyp3DEB67N7avvM",
        );
        check_hash(
            &Transaction::new_burn(&pk, &asset, 100000000, TESTNET, fee, ts),
            "CfsAEtEAwe4NFKjezeCssaUPevTX56rBsuMeMKRERd6Y",
        );
        check_hash(
            &Transaction::new_lease(&pk, &recipient, 10, TESTNET, fee, ts),
            "HHs5qfpDN88WTGszpfjVedhMPHeHynDtWPobm2rkpfH4",
        );
        check_hash(
            &Transaction::new_lease_cancel(&pk, &lease, TESTNET, fee, ts),
            "9BQLzTCHi9H9jqKeC5rvN7x9m8xfHQh1iApqmAPFTFEU",
        );
        let alias = Transaction::new_alias(&pk, "lilias", TESTNET, fee, ts);
        check_hash(&alias, "GPyHWQSCT6znfZmjfZfsS6TXPV3zueVZKFUWG7duku1Z");

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
            "HwWmpBbbYPShKsFAgVA3eH86LkrZgX1xYSoH5YarnwPE",
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
            "HASXvcgoikizpWnCLd2cXNeCN5DxdKojCfcn8f7T3KjK",
        );
        check_hash(
            &Transaction::new_script(&pk, None, TESTNET, fee, ts),
            "1gwS1qkKKShwk5scB7M7N9t6L3eX2Hpkp9hF5RG8HJD",
        );
        check_hash(
            &Transaction::new_sponsor(&pk, &asset, Some(100), fee, ts),
            "9zmHx3fyXz7pW6bRazPP28PGjnM8XjoHuyjzXCMHE2PY",
        );
        check_hash(
            &Transaction::new_set_asset_script(&pk, &asset, None, TESTNET, fee, ts),
            "FiZJwFqVbYiYeRN1oADFuqCmKHQJpDJonD5a9ekqXFuY",
        );
    }

    #[test]
    fn test_sign() {
        let sender = PrivateKeyAccount::from_seed("test");
        let recipient = Address::from_string("3MzGEv9wnaqrYFYujAXSH5RQfHaVKNQvx3D");
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
