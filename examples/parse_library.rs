extern crate swrs;

use std::fs;
use swrs::parser::library::Library;
use swrs::parser::Parsable;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a library to parse");
    let parsed = Library::parse(&*fs::read_to_string(filename).expect("Invalid path given"))
        .expect("Corrupted library file");

    println!("{:?}", parsed);
}
