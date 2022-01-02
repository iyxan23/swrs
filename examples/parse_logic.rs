extern crate swrs;

use std::fs;
use swrs::parser::logic::Logic;
use swrs::parser::Parsable;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a logic to parse");
    let parsed = Logic::parse(
        &*fs::read_to_string(filename).expect("Invalid path given"))
        .expect("Corrupted project file");

    println!("Screens:");
    for (name, screen) in parsed.screens {
        println!(
            " - {}: has {} variables, {} list variables, {} components, {} block containers, {} events, {} moreblocks",
            name,
            screen.variables.unwrap_or_default().0.len(),
            screen.list_variables.unwrap_or_default().0.len(),
            screen.components.unwrap_or_default().0.len(),
            screen.block_containers.len(),
            screen.events.unwrap_or_default().0.len(),
            screen.more_blocks.unwrap_or_default().0.len(),
        );
    }
}
