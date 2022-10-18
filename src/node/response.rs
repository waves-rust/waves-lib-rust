use serde::Deserialize;

macro_rules! response_generator {
    (struct $name:ident {
        $(
            $( #[$attr:meta] )?
            $field_name:ident: $field_type:ty,
        )*
    }) => {
        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct $name {
            $(
                $( #[$attr] )?
                $field_name: $field_type,
            )*
        }

        impl $name {
            $(
                pub fn $field_name(&self) -> $field_type {
                    self.$field_name.clone()
                }
            )*
        }
    }
}

response_generator! {
    struct ResponseBalance {
        balance: u64,
    }
}

response_generator! {
    struct ResponseBalanceDetails {
        regular: u64,
        generating: u64,
        available: u64,
        effective: u64,
    }
}

response_generator! {
    struct ResponseAddress {
        address: String,
    }
}

response_generator! {
    struct ResponseAsset {
        asset_id: String,
        name: String,
        description: String,
        decimals: u64,
        issuer: String,
        reissuable: bool,
    }
}

response_generator! {
    struct ResponseBlock {
        version: u64,
        timestamp: u64,
        reference: String,
        generator: String,
        signature: String,
        transaction_count: u64,
        id: String,
        height: u64,
        total_fee: u64,
    }
}

response_generator! {
    struct ResponseLease {
        id: String,
        origin_transaction_id: String,
        sender: String,
        recipient: String,
        amount: u64,
        height: u64,
        status: String,
        cancel_height: u64,
        cancel_transaction_id: String,
    }
}

response_generator! {
    struct ResponseNodeVersion {
        version: String,
    }
}

response_generator! {
    struct ResponseTransaction {
        #[serde(alias = "type")]
        type_id: u64,
        version: u64,
        id: String,
        sender: String,
        signature: String,
        timestamp: u64,
        fee: u64,
        fee_asset_id: Option<String>,
    }
}

response_generator! {
    struct ResponseTransactionStatus {
        id: String,
        status: String,
        height: u64,
        confirmations: u64,
    }
}
