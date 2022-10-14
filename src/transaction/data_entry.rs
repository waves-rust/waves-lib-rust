use std::fmt;

/// Structure that sets key and value of account data storage entry.
#[derive(Debug, Eq, PartialEq)]
pub enum DataEntry<'a> {
    Integer(&'a str, u64),
    Boolean(&'a str, bool),
    Binary(&'a str, &'a Vec<u8>),
    String(&'a str, &'a str),
}

impl<'a> fmt::Display for DataEntry<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DataEntry::Integer(key, value) => write!(f, "Data<Int>({}: {})", key, value),
            DataEntry::Boolean(key, value) => write!(f, "Data<Bool>({}: {})", key, value),
            DataEntry::Binary(key, value) => write!(f, "Data<Binary>({}: {:?})", key, value),
            DataEntry::String(key, value) => write!(f, "Data<String>({}: {})", key, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_entry() {
        let data_entry = DataEntry::Integer("key1", 42);
        assert_eq!(data_entry.to_string(), "Data<Int>(key1: 42)");

        let data_entry = DataEntry::Boolean("key2", true);
        assert_eq!(data_entry.to_string(), "Data<Bool>(key2: true)");

        let binary = vec![0u8, 1u8, 2u8];
        let data_entry = DataEntry::Binary("key3", &binary);
        assert_eq!(data_entry.to_string(), "Data<Binary>(key3: [0, 1, 2])");

        let data_entry = DataEntry::String("key4", "test");
        assert_eq!(data_entry.to_string(), "Data<String>(key4: test)");
    }
}
