use nosy::package::{DependencyGraph, Identifier, Version};
use std::{env, error::Error, fs};

const DEFAULT_SAMPLE_PATH: &str = "samples/tauri/Cargo.lock";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    args.next();

    let graph: DependencyGraph =
        fs::read_to_string(args.next().unwrap_or(String::from(DEFAULT_SAMPLE_PATH)))?.parse()?;

    let query = Identifier {
        name: String::from("serde"),
        version: None,
        // version: Some(Version::new(String::from("0.21.0"))),
    };

    println!("Entry: {:#?}", graph.search(&query));
    println!("In: {:#?}", graph.incoming_edges(&query));
    println!("Out: {:#?}", graph.outgoing_edges(&query));

    Ok(())
}
