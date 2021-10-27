extern crate swrs;

use swrs::parser::file::{ActivityOptions, File, FileItem, FileType, KeyboardSetting, Orientation, Theme};
use swrs::parser::Parsable;

fn main() {
    let file = r#"
@activity
{"fileName":"main","fileType":0,"keyboardSetting":0,"options":1,"orientation":0,"theme":-1}
@customview"#;

    let expected = File {
        activities: vec![
            FileItem {
                filename: "main".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions::from_num(1),
                orientation: Orientation::Portrait,
                theme: Theme::None
            }
        ],
        custom_views: vec![]
    };

    let parsed_file = File::parse(file).unwrap();

    assert_eq!(expected, parsed_file);
    println!("{:?}", parsed_file);
}
