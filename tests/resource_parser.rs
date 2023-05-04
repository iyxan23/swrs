use swrs::parser::resource::{Resource, ResourceItem};
use swrs::parser::Parsable;

#[test]
fn parse_resource_item() {
    let input = r#"{"resFullName":"my_img.png","resName":"my_img","resType":1}"#;
    let result = match ResourceItem::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse resource item: {:?}", err),
    };

    let expected = ResourceItem {
        full_name: "my_img.png".to_string(),
        name: "my_img".to_string(),
        r#type: 1,
    };

    assert_eq!(result, expected);
}

#[test]
fn parse_resource() {
    let input = r#"@images
{"resFullName":"ic_developer_mode_white.png","resName":"ic_developer_mode_white","resType":1}
{"resFullName":"ic_info_outline_white.png","resName":"ic_info_outline_white","resType":1}
{"resFullName":"ic_smartphone_white.png","resName":"ic_smartphone_white","resType":1}
@sounds
{"resFullName":"speaker_cleaner.mp3","resName":"speaker_cleaner","resType":1}
@fonts
{"resFullName":"font_regular.ttf","resName":"font_regular","resType":1}
{"resFullName":"font_light.ttf","resName":"font_light","resType":1}"#;

    let result = match Resource::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse resource: {:?}", err),
    };

    let expected = Resource {
        images: vec![
            ResourceItem {
                full_name: "ic_developer_mode_white.png".to_string(),
                name: "ic_developer_mode_white".to_string(),
                r#type: 1,
            },
            ResourceItem {
                full_name: "ic_info_outline_white.png".to_string(),
                name: "ic_info_outline_white".to_string(),
                r#type: 1,
            },
            ResourceItem {
                full_name: "ic_smartphone_white.png".to_string(),
                name: "ic_smartphone_white".to_string(),
                r#type: 1,
            },
        ],
        sounds: vec![ResourceItem {
            full_name: "speaker_cleaner.mp3".to_string(),
            name: "speaker_cleaner".to_string(),
            r#type: 1,
        }],
        fonts: vec![
            ResourceItem {
                full_name: "font_regular.ttf".to_string(),
                name: "font_regular".to_string(),
                r#type: 1,
            },
            ResourceItem {
                full_name: "font_light.ttf".to_string(),
                name: "font_light".to_string(),
                r#type: 1,
            },
        ],
    };

    assert_eq!(result, expected);
}

#[test]
fn reconstruct_resource_item() {
    let input = ResourceItem {
        full_name: "my_img.png".to_string(),
        name: "my_img".to_string(),
        r#type: 1,
    };

    let result = match input.reconstruct() {
        Ok(r) => r,
        Err(err) => panic!("Failed to reconstruct resource item: {:?}", err),
    };

    let expected = r#"{"resFullName":"my_img.png","resName":"my_img","resType":1}"#;

    assert_eq!(result, expected);
}

#[test]
fn reconstruct_resource() {
    let input = Resource {
        images: vec![
            ResourceItem {
                full_name: "ic_developer_mode_white.png".to_string(),
                name: "ic_developer_mode_white".to_string(),
                r#type: 1,
            },
            ResourceItem {
                full_name: "ic_info_outline_white.png".to_string(),
                name: "ic_info_outline_white".to_string(),
                r#type: 1,
            },
            ResourceItem {
                full_name: "ic_smartphone_white.png".to_string(),
                name: "ic_smartphone_white".to_string(),
                r#type: 1,
            },
        ],
        sounds: vec![ResourceItem {
            full_name: "speaker_cleaner.mp3".to_string(),
            name: "speaker_cleaner".to_string(),
            r#type: 1,
        }],
        fonts: vec![
            ResourceItem {
                full_name: "font_regular.ttf".to_string(),
                name: "font_regular".to_string(),
                r#type: 1,
            },
            ResourceItem {
                full_name: "font_light.ttf".to_string(),
                name: "font_light".to_string(),
                r#type: 1,
            },
        ],
    };

    let result = match input.reconstruct() {
        Ok(r) => r,
        Err(err) => panic!("Failed to reconstruct resource: {:?}", err),
    };

    let expected = r#"@images
{"resFullName":"ic_developer_mode_white.png","resName":"ic_developer_mode_white","resType":1}
{"resFullName":"ic_info_outline_white.png","resName":"ic_info_outline_white","resType":1}
{"resFullName":"ic_smartphone_white.png","resName":"ic_smartphone_white","resType":1}
@sounds
{"resFullName":"speaker_cleaner.mp3","resName":"speaker_cleaner","resType":1}
@fonts
{"resFullName":"font_regular.ttf","resName":"font_regular","resType":1}
{"resFullName":"font_light.ttf","resName":"font_light","resType":1}"#;

    assert_eq!(result, expected);
}
