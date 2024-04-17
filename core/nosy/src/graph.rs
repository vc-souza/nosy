//! TODO: doc

use std::{collections::HashMap, convert::TryFrom, rc::Rc};
use toml::{Table, Value};

/// TODO: doc
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Version(String);

/// TODO: doc
#[derive(Debug)]
pub enum Source {
    Local,
    Remote(String),
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct PackageIdentity {
    pub name: String,
    pub version: Option<Version>,
}

/// TODO: doc
#[derive(Debug)]
pub struct Package {
    pub identity: PackageIdentity,
    pub source: Source,
    pub dependencies: Option<Vec<PackageIdentity>>,
}

#[derive(Debug)]
pub struct PackageIndex {
    index: HashMap<PackageIdentity, Rc<Package>>, // TODO: not pub, impl a method!
}

impl PackageIndex {
    /// TODO: doc
    pub fn new(nodes: impl Iterator<Item = Package>) -> Self {
        let mut packages: PackageIndex = Self {
            index: HashMap::new(),
        };

        for package in nodes {
            let original_package = Rc::new(package);
            let ref_package = Rc::clone(&original_package);

            let full_key_name = original_package.identity.name.clone();
            let partial_key_name = original_package.identity.name.clone();

            packages.index.insert(
                PackageIdentity {
                    name: full_key_name,
                    version: match original_package.identity.version.as_ref() {
                        Some(v) => Some(Version(v.0.clone())),
                        None => None,
                    },
                },
                original_package,
            );

            packages.index.insert(
                PackageIdentity {
                    name: partial_key_name,
                    version: None,
                },
                ref_package,
            );
        }

        packages
    }

    /// TODO: doc
    pub fn search(&self, key: &PackageIdentity) -> Option<&Package> {
        match self.index.get(key) {
            Some(p) => Some(p.as_ref()),
            None => None,
        }
    }
}

impl TryFrom<String> for PackageIdentity {
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
            identity: PackageIdentity {
                name: match value.remove("name") {
                    Some(Value::String(name)) => name,
                    _ => return Err(String::from("Unable to find package name.")),
                },
                version: match value.remove("version") {
                    Some(Value::String(version)) => Some(Version(version)),
                    _ => return Err(String::from("Unable to find the package version.")),
                },
            },
            source: match value.remove("source") {
                Some(Value::String(source)) => Source::Remote(source),
                _ => Source::Local,
            },
            dependencies: match value.remove("dependencies") {
                Some(Value::Array(arr)) => Some(
                    arr.into_iter()
                        .filter_map(|v| match v {
                            Value::String(s) => PackageIdentity::try_from(s).ok(),
                            _ => None,
                        })
                        .collect(),
                ),
                _ => None,
            },
        })
    }
}

impl TryFrom<String> for PackageIndex {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut root = value.parse::<Table>().map_err(|e| e.to_string())?;

        match root.remove("package") {
            Some(Value::Array(arr)) => {
                Ok(PackageIndex::new(arr.into_iter().filter_map(|v| match v {
                    Value::Table(t) => Package::try_from(t).ok(),
                    _ => None,
                })))
            }
            _ => Err(String::from("Unable to read package list.")),
        }
    }
}
