extern crate swrs;

use std::fs;
use swrs::color::Color;
use swrs::parser::Parsable;
use swrs::parser::project::Project;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a project to parse");
    let parsed = Project::parse(
        &*fs::read_to_string(filename).expect("Invalid path given"))
        .expect("Corrupted project file");

    println!("{:?}", parsed);
}