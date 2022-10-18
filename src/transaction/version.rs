use std::fmt;

/// Transaction version
#[derive(Debug, Eq, PartialEq)]
pub enum Version {
    V1 = 1,
    V2 = 2,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Version::V1 => write!(f, "Version 1"),
            Version::V2 => write!(f, "Version 2"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        let version = Version::V1;
        assert_eq!(version.to_string(), "Version 1");
    }
}
