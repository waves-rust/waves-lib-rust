use std::fmt;

/// Transaction type
#[derive(Debug, Eq, PartialEq)]
pub enum Type {
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
