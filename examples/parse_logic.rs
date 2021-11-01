extern crate swrs;

use std::fs;
use swrs::parser::logic::Logic;
use swrs::parser::Parsable;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a logic to parse");
    let do_print: bool = args.next()
                             .map(|v|v.parse::<bool>().expect("Boolean on whether to print or not"))
                             .unwrap_or_else(||false);

    if do_print {
        println!("{:?}", Logic::parse(fs::read_to_string(filename).expect("Valid path").as_str()).unwrap());
    } else {
        Logic::parse(fs::read_to_string(filename).expect("Valid path").as_str())
            .unwrap();
    }
}
