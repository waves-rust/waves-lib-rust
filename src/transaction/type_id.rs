use std::fmt;

/// Transaction type
///
/// There are various types of transactions implemented on the Waves blockchain.
///
/// Transaction is an action on the blockchain on behalf of an account.
///
/// The Waves blockchain provides various types of transactions. For example:
///
/// * Data transaction writes data to the account data storage,
/// * Transfer transaction sends a certain amount of token to another account.
///
/// Content of transaction depends on its type.
#[derive(Debug, Eq, PartialEq)]
pub enum Type {
    /// Issue Transaction
    Issue = 3,
    /// Transfer Transaction
    Transfer = 4,
    /// Reissue Transaction
    Reissue = 5,
    /// Burn Transaction
    Burn = 6,
    /// Lease Transaction
    Lease = 8,
    /// Lease Cancel Transaction
    LeaseCancel = 9,
    /// Create Alias Transaction
    Alias = 10,
    /// Mass Transfer Transaction
    MassTransfer = 11,
    /// Data Transaction
    Data = 12,
    /// Set Script Transaction
    SetScript = 13,
    /// Sponsor Fee Transaction
    Sponsor = 14,
    /// Set Asset Script Transaction
    SetAssetScript = 15,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Issue => write!(f, "Issue Transaction"),
            Type::Transfer => write!(f, "Transfer Transaction"),
            Type::Reissue => write!(f, "Reissue Transaction"),
            Type::Burn => write!(f, "Burn Transaction"),
            Type::Lease => write!(f, "Lease Transaction"),
            Type::LeaseCancel => write!(f, "Lease Cancel Transaction"),
            Type::Alias => write!(f, "Create Alias Transaction"),
            Type::MassTransfer => write!(f, "Mass Transfer Transaction"),
            Type::Data => write!(f, "Data Transaction"),
            Type::SetScript => write!(f, "Set Script Transaction"),
            Type::Sponsor => write!(f, "Sponsor Fee Transaction"),
            Type::SetAssetScript => write!(f, "Set Asset Script Transaction"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type() {
        let type_id = Type::Issue;
        assert_eq!(type_id.to_string(), "Issue Transaction");
    }
}
