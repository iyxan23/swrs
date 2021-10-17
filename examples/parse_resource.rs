extern crate swrs;

use swrs::parser::ProjectData;
use swrs::parser::resource::{Resource, ResourceItem};

fn main() {
    let resource = r#"
@images
{"resFullName":"very_cool_image.png","resName":"my_cool_image","resType":1}
@sounds
{"resFullName":"very_cool_sound.mp3","resName":"my_cool_sound","resType":1}
@fonts
{"resFullName":"very_cool_font.ttf","resName":"my_cool_font","resType":1}"#;

    let expected = Resource {
        images: vec![
            ResourceItem {
                full_name: "very_cool_image.png".to_string(),
                name: "my_cool_image".to_string(),
                r#type: 1
            }
        ],
        sounds: vec![
            ResourceItem {
                full_name: "very_cool_sound.mp3".to_string(),
                name: "my_cool_sound".to_string(),
                r#type: 1
            }
        ],
        fonts: vec![
            ResourceItem {
                full_name: "very_cool_font.ttf".to_string(),
                name: "my_cool_font".to_string(),
                r#type: 1
            }
        ]
    };

    let parsed_file = Resource::parse(resource).unwrap();

    assert_eq!(expected, parsed_file);
    println!("{:?}", parsed_file);
}
