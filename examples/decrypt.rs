extern crate swrs;

use std::process::exit;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} (file to decrypt)", args.get(0).unwrap());
        exit(1);
    }

    let path = args.get(1).unwrap();
    let file = Path::new(path);

    let decrypted = swrs::decrypt_sw_file(file);

    if decrypted.is_err() {
        println!("{:?}", decrypted.unwrap_err());
        exit(1);
    }

    let decrypted = String::from_utf8(decrypted.unwrap()).unwrap();
    println!("{}", decrypted)
}