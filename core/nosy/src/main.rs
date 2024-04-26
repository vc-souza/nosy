use nosy::package::{DependencyGraph, Identifier};
use std::{env, error::Error, fs};

const DEFAULT_SAMPLE_PATH: &str = "samples/tauri/Cargo.lock";

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();

    args.next();

    let filepath = args.next().unwrap_or(String::from(DEFAULT_SAMPLE_PATH));
    let graph: DependencyGraph = fs::read_to_string(filepath)?.parse()?;

    let report = |query| {
        println!("Entry: {:#?}", graph.search(query));
        println!("In: {:#?}", graph.incoming_edges(query));
        println!("Out: {:#?}", graph.outgoing_edges(query));
    };

    let partial_query = Identifier::simple("serde");
    let full_query = Identifier::full("urlpattern", "0.2.0");

    report(&partial_query);
    report(&full_query);

    Ok(())
}
