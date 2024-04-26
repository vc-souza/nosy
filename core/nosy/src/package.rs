use std::collections::HashMap;
use std::convert::TryFrom;
use std::error::Error;
use std::rc::{Rc, Weak};
use std::str::FromStr;
use toml::{Table, Value};

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
pub struct DependencyGraph {
    index: HashMap<Identifier, Rc<Package>>,
    incoming: HashMap<Identifier, Vec<Weak<Package>>>,
    outgoing: HashMap<Identifier, Vec<Rc<Package>>>,
}

impl DependencyGraph {
    /// TODO: doc
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
            incoming: HashMap::new(),
            outgoing: HashMap::new(),
        }
    }

    /// TODO: doc
    pub fn add_package(&mut self, package: &Rc<Package>) -> () {
        // specific version
        self.index.insert(
            Identifier {
                name: package.id.name.to_owned(),
                version: package.id.version.to_owned(),
            },
            Rc::clone(package),
        );

        // default version
        self.index.insert(
            Identifier {
                name: package.id.name.to_owned(),
                version: None,
            },
            Rc::clone(package),
        );
    }

    /// TODO: doc
    pub fn register_dependency(
        &mut self,
        package: &Rc<Package>,
        dependency: &Identifier,
    ) -> Result<(), Box<dyn Error>> {
        let resolved_dependency = self.index.get(&dependency).ok_or(String::from(
            "The package should have been added to the index",
        ))?;

        let dependencies = match self.outgoing.get_mut(&package.id) {
            Some(deps) => deps,
            None => {
                self.outgoing.insert(
                    Identifier {
                        name: package.id.name.to_owned(),
                        version: package.id.version.to_owned(),
                    },
                    Vec::with_capacity(1),
                );

                self.outgoing
                    .get_mut(&package.id)
                    .ok_or(String::from("The entry was just inserted"))?
            }
        };

        dependencies.push(Rc::clone(resolved_dependency));

        let back_references = match self.incoming.get_mut(&resolved_dependency.id) {
            Some(refs) => refs,
            None => {
                self.incoming.insert(
                    Identifier {
                        name: resolved_dependency.id.name.to_owned(),
                        version: resolved_dependency.id.version.to_owned(),
                    },
                    Vec::with_capacity(1),
                );

                self.incoming
                    .get_mut(&resolved_dependency.id)
                    .ok_or(String::from("The entry was just inserted"))?
            }
        };

        back_references.push(Rc::downgrade(package));

        Ok(())
    }

    /// TODO: doc
    pub fn search(&self, identifier: &Identifier) -> Option<&Package> {
        Some(self.index.get(identifier)?.as_ref())
    }

    /// TODO: doc
    pub fn incoming_edges(&self, identifier: &Identifier) -> Option<Vec<Option<Rc<Package>>>> {
        Some(
            self.incoming
                .get(&(self.index.get(identifier)?).id)?
                .iter()
                .map(|w| w.upgrade())
                .collect(),
        )
    }

    pub fn outgoing_edges(&self, identifier: &Identifier) -> Option<&Vec<Rc<Package>>> {
        self.outgoing.get(&(self.index.get(identifier)?).id)
    }
}

impl FromStr for DependencyGraph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut root = s.parse::<Table>().map_err(|e| e.to_string())?;

        match root.remove("package") {
            Some(Value::Array(arr)) => {
                let mut graph = DependencyGraph::new();

                let adjacencies: Vec<Adjacency> = arr
                    .into_iter()
                    .filter_map(|v| match v {
                        Value::Table(t) => Some(Adjacency::try_from(t).ok()?),
                        _ => None,
                    })
                    .collect();

                for Adjacency(package, _) in &adjacencies {
                    graph.add_package(package);
                }

                for Adjacency(package, dependencies) in adjacencies {
                    if let Some(dependencies) = dependencies {
                        for dependency in &dependencies {
                            graph
                                .register_dependency(&package, dependency)
                                .map_err(|e| e.to_string())?;
                        }
                    }
                }

                Ok(graph)
            }
            _ => return Err(String::from("Invalid package list")),
        }
    }
}

// TODO: test!
