extern crate swrs;

use std::fs;
use swrs::parser::view::View;
use swrs::parser::Parsable;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a view to parse");
    let parsed = View::parse(&*fs::read_to_string(filename).expect("Invalid path given"))
        .expect("Corrupted view file");

    println!("Screens ==");

    for (name, layout) in parsed.layouts {
        println!(" Views of layout {}:", name);
        for (index, view) in layout.0.into_iter().enumerate() {
            println!("  {}. id: {}, type: {}", index, view.id, view.r#type);
        }

        println!();
    }
}
