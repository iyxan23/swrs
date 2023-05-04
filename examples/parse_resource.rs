extern crate swrs;

use std::fs;
use swrs::parser::resource::Resource;
use swrs::parser::Parsable;

fn main() {
    let mut args = std::env::args();
    args.next();

    let filename = args.next().expect("File path of a resource to parse");
    let parsed = Resource::parse(&*fs::read_to_string(filename).expect("Invalid path given"))
        .expect("Corrupted resource file");

    println!("\nFonts:");
    for font in parsed.fonts {
        println!(" - {} - {}", font.name, font.full_name);
    }

    println!("\nImages:");
    for image in parsed.images {
        println!(" - {} - {}", image.name, image.full_name);
    }

    println!("\nSounds:");
    for sound in parsed.sounds {
        println!(" - {} - {}", sound.name, sound.full_name);
    }
}
