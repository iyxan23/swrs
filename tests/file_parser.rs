use swrs::parser::file::{
    ActivityOptions, File, FileItem, FileType, KeyboardSetting, Orientation, Theme,
};
use swrs::parser::Parsable;

#[test]
fn parse_file_item() {
    let input = r#"{"fileName":"main","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}"#;
    let result = match FileItem::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse FileItem: {:?}", err),
    };

    let expected = FileItem {
        filename: "main".to_string(),
        file_type: FileType::Activity,
        keyboard_setting: KeyboardSetting::Unspecified,
        options: ActivityOptions {
            toolbar_enabled: false,
            fullscreen_enabled: false,
            drawer_enabled: false,
            fab_enabled: false,
        },
        orientation: Orientation::Both,
        theme: Theme::None,
    };

    assert_eq!(result, expected);
}

#[test]
fn parse_file() {
    let input = r#"@activity
{"fileName":"main","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"deviceinfo","fileType":0,"keyboardSetting":0,"options":5,"orientation":2,"theme":-1}
{"fileName":"about","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"speaker_cleaner","fileType":0,"keyboardSetting":0,"options":0,"orientation":0,"theme":-1}
{"fileName":"tools","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"display_test","fileType":0,"keyboardSetting":2,"options":2,"orientation":2,"theme":-1}
@customview
{"fileName":"_drawer_deviceinfo","fileType":2,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"design_3","fileType":1,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}"#;

    let result = match File::parse(input) {
        Ok(r) => r,
        Err(err) => panic!("Failed to parse file: {:?}", err),
    };

    let expected = File {
        activities: vec![
            FileItem {
                filename: "main".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "deviceinfo".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: true,
                    fullscreen_enabled: false,
                    drawer_enabled: true,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "about".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "speaker_cleaner".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Portrait,
                theme: Theme::None,
            },
            FileItem {
                filename: "tools".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "display_test".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Hidden,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: true,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
        ],
        custom_views: vec![
            FileItem {
                filename: "_drawer_deviceinfo".to_string(),
                file_type: FileType::Drawer,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "design_3".to_string(),
                file_type: FileType::CustomView,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
        ],
    };

    assert_eq!(result, expected);
}

#[test]
fn reconstruct_file_item() {
    let input = FileItem {
        filename: "main".to_string(),
        file_type: FileType::Activity,
        keyboard_setting: KeyboardSetting::Unspecified,
        options: ActivityOptions {
            toolbar_enabled: false,
            fullscreen_enabled: false,
            drawer_enabled: false,
            fab_enabled: false,
        },
        orientation: Orientation::Both,
        theme: Theme::None,
    };

    let result = match input.reconstruct() {
        Ok(r) => r,
        Err(err) => panic!("Failed reconstructing fileitem: {:?}", err),
    };

    let expected = r#"{"fileName":"main","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}"#;

    assert_eq!(result, expected);
}

#[test]
fn reconstruct_file() {
    let input = File {
        activities: vec![
            FileItem {
                filename: "main".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "deviceinfo".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: true,
                    fullscreen_enabled: false,
                    drawer_enabled: true,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "about".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "speaker_cleaner".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Portrait,
                theme: Theme::None,
            },
            FileItem {
                filename: "tools".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "display_test".to_string(),
                file_type: FileType::Activity,
                keyboard_setting: KeyboardSetting::Hidden,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: true,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
        ],
        custom_views: vec![
            FileItem {
                filename: "_drawer_deviceinfo".to_string(),
                file_type: FileType::Drawer,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
            FileItem {
                filename: "design_3".to_string(),
                file_type: FileType::CustomView,
                keyboard_setting: KeyboardSetting::Unspecified,
                options: ActivityOptions {
                    toolbar_enabled: false,
                    fullscreen_enabled: false,
                    drawer_enabled: false,
                    fab_enabled: false,
                },
                orientation: Orientation::Both,
                theme: Theme::None,
            },
        ],
    };

    let result = match input.reconstruct() {
        Ok(r) => r,
        Err(err) => panic!("Failed to reconstruct file: {:?}", err),
    };

    let expected = r#"@activity
{"fileName":"main","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"deviceinfo","fileType":0,"keyboardSetting":0,"options":5,"orientation":2,"theme":-1}
{"fileName":"about","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"speaker_cleaner","fileType":0,"keyboardSetting":0,"options":0,"orientation":0,"theme":-1}
{"fileName":"tools","fileType":0,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"display_test","fileType":0,"keyboardSetting":2,"options":2,"orientation":2,"theme":-1}
@customview
{"fileName":"_drawer_deviceinfo","fileType":2,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}
{"fileName":"design_3","fileType":1,"keyboardSetting":0,"options":0,"orientation":2,"theme":-1}"#;

    assert_eq!(result, expected);
}
