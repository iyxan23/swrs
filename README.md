<h1 align="center"><pre>swrs</pre></h1>
<p align="center">A rust library that parses and reconstructs sketchware projects</p>

swrs is (my first) rust library that can parse and reconstruct Sketchware projects easily. You can use the provided apis ([`crate::api`](/src/api)) to modify parsed Sketchware projects and convert them back into a raw sketchware project without much hassle!

This library is in **alpha**, and only supports regular Sketchware projects; this library has not been tested with modded projects (although I do have a plan to support it sometime in the future). This library has been tested with multiple projects off of [Sketchub](https://sketchub.in) and parsing and reconstruction worked flawlessly.

### Structure

It's important to take note that this `swrs` library is composed of two parts: [`parser`](src/parser) and [`api`](src/api).

 - **`parser`** is the part where raw sketchware projects are then being parsed into a structure that still retains their raw form but with rust types. It's output is a [`swrs::parser::SketchwareProject`](https://github.com/Iyxan23/swrs/blob/dev/src/parser/mod.rs#L138)
 - **`api`** is the part where the `parser` models are then read and interpreted in a way in order to provide high level apis of collecting data and modifying the project. It's output is a [`swrs::api::SketchwareProject`](https://github.com/Iyxan23/swrs/blob/dev/src/api/mod.rs#L238)

Sketchware projects starts from being parsed by the `parser` ([here](https://github.com/Iyxan23/swrs/blob/0f570c0f8907504a36ec3169385dcbe17edec401/src/parser/mod.rs#L153)) into rust structs and then piped onto `api` ([here](https://github.com/Iyxan23/swrs/blob/dev/src/api/mod.rs#L252)) to provide high-level APIs like iterating through blocks, substacks, view tree, etc.

### Installation
I don't have the courage to publish this library on crates.io, it's pretty useless lol.

Just clone this repo and include it in your `Cargo.toml` through `path` or using cargo workspaces.

### Usage
```rs
// First you'll need to make a RawSketchwareProject struct instance
// which contains unencrypted raw data of a Sketchware project
let raw = RawSketchwareProject::from_encrypted(
    std::fs::read(Path::new("path/to/project")),
    std::fs::read(Path::new("path/to/file")),
    ...
);

// Then you'll have to parse it
let parsed = ParserSketchwareProject::parse_from(raw).except("Corrupted sketchware project");

// !! NOTE: ParserSketchwareProject is [`swrs::parser::SketchwareProject`] !!
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
