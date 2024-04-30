//! TODO: doc

use std::str::FromStr;

/// TODO: doc
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Version(String);

impl Version {
    pub fn new(s: String) -> Self {
        Self(s)
    }
}

/// TODO: doc
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub name: String,
    pub version: Option<Version>,
}

impl Identifier {
    /// TODO: doc
    pub fn simple<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            version: None,
        }
    }

    /// TODO: doc
    pub fn full<T, U>(name: T, version: U) -> Self
    where
        T: Into<String>,
        U: Into<String>,
    {
        Self {
            name: name.into(),
            version: Some(Version(version.into())),
        }
    }
}

impl FromStr for Identifier {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(" ");

        Ok(Self {
            name: match i.next() {
                Some(n) => String::from(n),
                None => return Err(String::from("Empty package name")),
            },
            version: match i.next() {
                Some(v) => Some(Version(String::from(v))),
                None => None,
            },
        })
    }
}

/// TODO: doc
#[derive(Debug)]
pub struct Package {
    pub id: Identifier,
    pub source: Option<String>,
    pub checksum: Option<String>,
}

// TODO: test!
