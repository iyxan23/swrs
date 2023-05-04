extern crate swrs;

use std::fs;
use swrs::parser::file::File;
use swrs::parser::Parsable;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a file to parse");
    let parsed = File::parse(&*fs::read_to_string(filename).expect("Invalid path given"))
        .expect("Corrupted file file");

    println!("Activities:");
    for activity in parsed.activities {
        println!(" - {}: {:?}", activity.filename, activity);
    }

    println!("\nCustom views:");
    for custom_view in parsed.custom_views {
        println!(" - {}: {:?}", custom_view.filename, custom_view);
    }
}
