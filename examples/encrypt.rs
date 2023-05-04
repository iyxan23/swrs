extern crate swrs;

use std::env;
use std::path::Path;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 && args.len() != 2 {
        eprintln!(
            r#"Usage: {} (FILE TO ENCRYPT) [OUTPUT]

OUTPUT will be "(FILE TO ENCRYPT)_enc" if not specified"#,
            args.get(0).unwrap()
        );
        exit(1);
    }

    let input = args.get(1).unwrap();
    let default_output = format!("{}_enc", input);
    let output = args.get(2).unwrap_or(&default_output);

    let input_path = Path::new(input);
    let output_path = Path::new(output);

    println!("Saving as {}", output);

    let result = swrs::encrypt_sw_file_to_file(input_path, Option::Some(output_path));

    if let Err(e) = result {
        eprintln!("Failed to encrypt: {:?}", e);
        exit(1);
    } else {
        println!("Successfully encrypted {}", input);
    }
}
