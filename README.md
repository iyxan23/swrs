<h1 align="center"><pre>swrs</pre></h1>
<p align="center">A rust library that parses and reconstructs sketchware projects</p>

swrs is (my first) rust library that can parse and reconstruct Sketchware projects easily. You can use the provided apis ([`crate::api`](/src/api)) to modify parsed Sketchware projects and convert them back into a raw sketchware project without much hassle!

This library is in **alpha**, and only supports regular Sketchware projects; this library have not been tested with modded projects - although I do have a plan to support it maybe sometime in the future. This library has been tested with multiple projects off of [Sketchub](https://sketchub.in) and parsing and reconstruction worked flawlessly.

I'm going to state that this library is pretty useless; nobody uses Sketchware anymore and it's getting irrelevant as time goes on. So, don't expect me to do a lot of maintenance on this library. I mainly made this so that I could build my next project.

### Installation
I don't have the courage to publish this library on crates.io, it's pretty useless anyway.

### Usage
```rs
use swrs::parser::RawSketchwareProject;
use swrs::parser::SketchwareProject as ParserSketchwareProject;
use swrs::api::SketchwareProject;

use swrs::api::block::Blocks;
use swrs::api::block::BlockId;
use swrs::api::block::BlockContent;

use swrs::parser::logic::variable::Variable;
use swrs::parser::logic::variable::VariableType;

// First you'll need to make a RawSketchwareProject struct instance
// which contains unencrypted raw data of a Sketchware project
let raw = RawSketchwareProject::from_encrypted(
    std::fs::read(Path::new("path/to/project")),
    std::fs::read(Path::new("path/to/file")),
    ...
);

// Then you'll have to parse it
let parsed = ParserSketchwareProject::parse_from(raw).except("Corrupted sketchware project");

// note: ParserSketchwareProject is [`swrs::parser::SketchwareProject`]
// it's type aliased because it has the same name as in `swrs::api::SketchwareProject`

// And convert it to an API object so that you can modify it easily
let project = SketchwareProject::try_from(parsed).except("Corrupted sketchware project");

// There you have it!
println!("Screens:");

for screen in &mut project.screens {
    println!(" - {} has {} total events", screen.layout_name, screen.events.len());

    // Add a boolean variable
    screen.variables.insert("hello_world".to_string(), Variable {
        name: "hello_world".to_string(),
        r#type: VariableType::Boolean
    });

    // Add a block on onCreate
    if let Some(on_create) = screen.events.iter_mut().find(|e| e.name == "onCreate") {
        // I might need to make a better api for this lmao
        on_create.code.push_block(Block {
            id: BlockId(0), // id and next_block are set by the function
            next_block: None,
            sub_stack1: None,
            sub_stack2: None,
            color: Color::from_rgb(0xa0, 0x6f, 0x3d), // block color
            op_code: "addSourceDirectly".to_string(),
            content: BlockContent::parse_from(
                "add source directly %s.inputOnly",
                Some(vec!["// Hello asd from swrs!".to_string()]),
                |_| unreachable!()
            ).unwrap(),
            ret_type: Default::default(),
            type_name: Default::default()
        });
    }

    // And a lot more!
}
```
