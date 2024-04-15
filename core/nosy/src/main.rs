use nosy::graph::Packages;
use std::{env, error::Error, fs};
use toml::{Table, Value};

// 1. Record Vertices (Packages)
// 2. Record Edges (Dependencies)
// -- Dependency: (name: String, version: String, source: ENUM? possibly null)

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    args.next();

    let path = args.next().unwrap_or(String::from("./Cargo.lock"));

    let packages = Packages::try_from(fs::read_to_string(path)?)?;

    println!("{:#?}", packages);

    Ok(())
}
