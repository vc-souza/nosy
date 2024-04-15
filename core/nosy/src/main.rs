use nosy::graph::Package;
use std::{env, error::Error, fs};
use toml::{Table, Value};

// 1. Record Vertices (Packages)
// 2. Record Edges (Dependencies)
// -- Dependency: (name: String, version: String, source: ENUM? possibly null)

fn parse_lock_file(path: &str) -> Result<Table, Box<dyn Error>> {
    Ok(fs::read_to_string(path)?.parse()?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    args.next();

    let path = args.next().unwrap_or(String::from("./Cargo.lock"));
    let mut contents = parse_lock_file(&path)?;

    if let Some(Value::Array(arr)) = contents.remove("package") {
        println!("{}", arr.len());

        for raw_package in arr {
            if let Value::Table(package) = raw_package {
                println!("{:#?}", Package::try_from(package));
            }

            // if let Some(Value::String(s)) = package.get("name") {
            //     println!(">> pkg: {}", s);
            // }

            // if let Some(Value::String(s)) = package.get("version") {
            //     println!(">>>> version: {}", s);
            // } else {
            //     println!(">>>> no version");
            // }

            // if let Some(Value::Array(deps)) = package.get("dependencies") {
            //     println!(">>>> dependencies");

            //     for dep in deps {
            //         if let Value::String(s) = dep {
            //             println!("{}", s);
            //         } else {
            //             println!("{:?}", dep);
            //         }
            //     }
            // }
        }
    }

    Ok(())
}
