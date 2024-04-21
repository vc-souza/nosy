use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use toml::{Table, Value};

/// TODO: doc
#[derive(Debug)]
pub struct Version(String);

/// TODO: doc
#[derive(Debug)]
pub struct Identifier {
    pub name: String,
    pub version: Option<Version>,
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

/// TODO: doc
struct Adjacency(Rc<Package>, Option<Vec<Identifier>>);

impl TryFrom<Table> for Adjacency {
    type Error = String;

    fn try_from(mut value: Table) -> Result<Self, Self::Error> {
        Ok(Self(
            Rc::new(Package {
                id: Identifier {
                    name: match value.remove("name") {
                        Some(Value::String(s)) => s,
                        _ => return Err(String::from("Invalid package name")),
                    },
                    version: match value.remove("version") {
                        Some(Value::String(s)) => Some(Version(s)),
                        _ => return Err(String::from("Invalid package version")),
                    },
                },
                source: match value.remove("source") {
                    Some(Value::String(s)) => Some(s),
                    _ => None,
                },
                checksum: match value.remove("checksum") {
                    Some(Value::String(s)) => Some(s),
                    _ => None,
                },
            }),
            match value.remove("dependencies") {
                Some(Value::Array(arr)) => Some(
                    arr.into_iter()
                        .filter_map(|v| match v {
                            Value::String(s) => s.parse::<Identifier>().ok(),
                            _ => None,
                        })
                        .collect(),
                ),
                _ => None,
            },
        ))
    }
}

/// TODO: doc
type Index = HashMap<Identifier, Rc<Package>>;

/// TODO: doc
type Outgoing = HashMap<Identifier, Option<Vec<Rc<Package>>>>;

/// TODO: doc
type Incoming = HashMap<Identifier, Option<Vec<Weak<Package>>>>;
