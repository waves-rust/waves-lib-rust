use crate::account::Address;
use crate::transaction::{Asset, DataEntry, TransactionId};

/// Data specific to a particular transaction type
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
    },
}
