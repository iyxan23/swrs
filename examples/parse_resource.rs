extern crate swrs;

use swrs::parser::resource::Resource;

fn main() {
    let resource = r#"@images
{"resFullName":"hello_world.png","resName":"hello_world","resType":1}
@sounds
@fonts"#;

    println!("{:?}", Resource::parse(resource).unwrap());
}