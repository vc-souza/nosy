//! TODO: doc

use std::convert::TryFrom;
use toml::{Table, Value};

/// TODO: doc
#[derive(Debug)]
pub struct Version(String);

/// TODO: doc
#[derive(Debug)]
pub enum Source {
    Local,
    Remote(String),
}

/// TODO: doc
#[derive(Debug)]
pub struct Dependency {
    name: String,
    version: Option<Version>,
}

/// TODO: doc
#[derive(Debug)]
pub struct Package {
    name: String,
    version: Version,
    source: Source,
    dependencies: Option<Vec<Dependency>>,
}

impl TryFrom<String> for Dependency {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut iter = value.split(" ");

        Ok(Self {
            name: match iter.next() {
                Some(s) => String::from(s),
                None => return Err(String::from("Empty dependency")),
            },
            version: match iter.next() {
                Some(s) => Some(Version(String::from(s))),
                None => None,
            },
        })
    }
}

impl TryFrom<Table> for Package {
    type Error = String;

    fn try_from(mut value: Table) -> Result<Self, Self::Error> {
        Ok(Self {
            name: match value.remove("name") {
                Some(Value::String(name)) => name,
                _ => return Err(String::from("Unable to find package name.")),
            },
            version: match value.remove("version") {
                Some(Value::String(version)) => Version(version),
                _ => return Err(String::from("Unable to find the package version.")),
            },
            source: match value.remove("source") {
                Some(Value::String(source)) => Source::Remote(source),
                _ => Source::Local,
            },
            dependencies: match value.remove("dependencies") {
                Some(Value::Array(arr)) => Some(
                    arr.into_iter()
                        .filter_map(|v| match v {
                            Value::String(s) => Dependency::try_from(s).ok(),
                            _ => None,
                        })
                        .collect(),
                ),
                _ => None,
            },
        })
    }
}
