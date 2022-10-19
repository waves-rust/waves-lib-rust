use regex::Regex;
use std::fmt;

/// Regular expression for the definition of alias
const REGEXP: &str = "^[-.0-9@_a-z]{4,30}$";
/// Regular expression for the definition of alias with prefix
const REGEXP_WITH_PREFIX: &str = "^alias:[A-Z]{1}:[-.0-9@_a-z]{4,30}$";

/// List of errors in processing [`Alias`]
#[derive(Debug, PartialEq, Eq)]
pub enum AliasError {
    InvalidAlias,
    RegexError,
}

/// The [`Alias`] type on working with aliases in the Waves blockchain and presenting it in a format with or without the network prefix
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Alias(String);

impl Alias {
    /// Create an [`Alias`] from the string
    pub fn new(alias: &str) -> Result<Alias, AliasError> {
        if Self::is_valid(REGEXP, alias)? {
            Ok(Alias(format!("{}", alias)))
        } else if Self::is_valid(REGEXP_WITH_PREFIX, alias)? {
            let value = Self::replace_prefix(alias)?;
            Ok(Alias(value))
        } else {
            Err(AliasError::InvalidAlias)
        }
    }

    /// Representing [`Alias`] as a string with a prefix
    pub fn to_string_with_prefix(&self, chain_id: u8) -> String {
        format!("alias:{}:{}", chain_id as char, self.0)
    }

    /// Validation of alias using regular expressions
    fn is_valid(regexp: &str, alias: &str) -> Result<bool, AliasError> {
        match Regex::new(regexp) {
            Ok(r) => Ok(r.is_match(alias)),
            Err(_) => Err(AliasError::RegexError),
        }
    }

    /// Removing the prefix with regular expressions
    fn replace_prefix(alias: &str) -> Result<String, AliasError> {
        match Regex::new("^alias:[A-Z]{1}:") {
            Ok(r) => Ok(r.replace(alias, "").to_string()),
            Err(_) => Err(AliasError::RegexError),
        }
    }
}

impl fmt::Display for Alias {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::account::TESTNET;

    #[test]
    fn test_alias() {
        let result = Alias::new("test");
        assert!(result.is_ok());

        let alias = result.unwrap();

        assert_eq!(alias.to_string(), "test");
        assert_eq!(alias.to_string_with_prefix(TESTNET), "alias:T:test");

        let result = Alias::new("a");
        assert_eq!(result, Err(AliasError::InvalidAlias));

        let result = Alias::new("3MzGEv9wnaqrYFYujAXSH5RQfHaVKNQvx3D");
        assert_eq!(result, Err(AliasError::InvalidAlias));
    }

    #[test]
    fn test_alias_with_prefix() {
        let result = Alias::new("alias:T:test");
        assert!(result.is_ok());

        let alias = result.unwrap();

        assert_eq!(alias.to_string(), "test");
        assert_eq!(alias.to_string_with_prefix(TESTNET), "alias:T:test");
    }
}
