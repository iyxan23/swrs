extern crate swrs;

use std::fs;
use swrs::parser::Parsable;
use swrs::parser::view::View;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a view to parse");
    let parsed = View::parse(
        &*fs::read_to_string(filename).expect("Invalid path given"))
        .expect("Corrupted view file");

    println!("Screens ==");

    for (name, screen) in parsed.screens {
        println!(" Views of screen {}:", name);
        for (index, view )in screen.0.into_iter().enumerate() {
            println!("  {}. id: {}, type: {}", index, view.id, view.r#type);
        }

        println!();
    }
}
