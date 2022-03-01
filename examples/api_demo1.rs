extern crate swrs;

use std::fs::read;
use swrs::api::block::Blocks;
use swrs::api::SketchwareProject;
use swrs::parser::RawSketchwareProject;
use swrs::parser::SketchwareProject as ParsedSketchwareProject;

fn main() {
    let mut args = std::env::args();
    args.next();

    let project = read(args.next().expect("File path of a project data file to parse")).expect("Invalid project path given");
    let file = read(args.next().expect("File path of a file data file to parse")).expect("Invalid file path given");
    let library = read(args.next().expect("File path of a library data file to parse")).expect("Invalid library path given");
    let resource = read(args.next().expect("File path of a resource data file to parse")).expect("Invalid resource path given");
    let view = read(args.next().expect("File path of a view data file to parse")).expect("Invalid view path given");
    let logic = read(args.next().expect("File path of a logic data file to parse")).expect("Invalid logic path given");

    // decrypt them

    let project = swrs::decrypt_sw_encrypted(&project).expect("Unable to decrypt project");
    let file = swrs::decrypt_sw_encrypted(&file).expect("Unable to decrypt file");
    let library = swrs::decrypt_sw_encrypted(&library).expect("Unable to decrypt library");
    let resource = swrs::decrypt_sw_encrypted(&resource).expect("Unable to decrypt resource");
    let view = swrs::decrypt_sw_encrypted(&view).expect("Unable to decrypt view");
    let logic = swrs::decrypt_sw_encrypted(&logic).expect("Unable to decrypt logic");

    // turn them into strings

    let project = String::from_utf8(project).expect("Invalid charset for project");
    let file = String::from_utf8(file).expect("Invalid charset for file");
    let library = String::from_utf8(library).expect("Invalid charset for library");
    let resource = String::from_utf8(resource).expect("Invalid charset for resource");
    let view = String::from_utf8(view).expect("Invalid charset for view");
    let logic = String::from_utf8(logic).expect("Invalid charset for logic");

    // and parse them

    let sketchware_project = SketchwareProject::try_from(
        ParsedSketchwareProject::parse_from(
            RawSketchwareProject::new(project, file, library, resource, view, logic, vec![])
        ).expect("Corrupted sketchware project")
    ).expect("Corrupted sketchware project");

    println!("Screens:");

    for screen in sketchware_project.screens {
        println!("  Screen {} or {}:", screen.layout_name, screen.java_name);
        println!("    Events:");

        for event in screen.events {
            println!("      - Event {}, type: {:?}, blocks:", event.name, event.event_type);

            fn print_blocks(indentation: u32, blocks: Blocks) {
                for block in blocks {
                    println!("{}Block #{} opcode {}: {}", " ".repeat(indentation as usize), block.id.0, block.op_code, block.spec.to_string());

                    if let Some(blocks_ss1) = block.sub_stack1 {
                        println!("{}substack1: ", " ".repeat(indentation as usize));
                        print_blocks(indentation + 2, blocks_ss1);
                    }

                    if let Some(blocks_ss2) = block.sub_stack2 {
                        println!("{}substack2: ", " ".repeat(indentation as usize));
                        print_blocks(indentation + 2, blocks_ss2);
                    }
                }

                println!();
            }

            print_blocks(8, event.code);
        }
    }
}