extern crate swrs;

use swrs::color::Color;
use swrs::parser::project::{Project, ProjectColorPalette};
use swrs::parser::Parsable;

fn main() {
    let project = r#"
{
    "custom_icon": false,
    "sc_ver_code": "1",
    "my_ws_name": "Workspace Name",
    "color_accent": -1.6740915E7,
    "my_app_name": "App Name",
    "sc_ver_name": "1.0",
    "sc_id": "649",
    "color_primary": -1.6740915E7,
    "color_control_highlight": 5.36907213E8,
    "color_control_normal": -1.1026706E7,
    "my_sc_reg_dt": "20201220074905",
    "sketchware_ver": 150,
    "my_sc_pkg_name": "com.iyxan23.something",
    "color_primary_dark": -1.674323E7
}"#;

    let expected = Project {
        id: 649,
        app_name: "App Name".to_string(),
        workspace_name: "Workspace Name".to_string(),
        package_name: "com.iyxan23.something".to_string(),
        version_code: 1,
        version_name: "1.0".to_string(),
        date_created: 1608450545,
        custom_icon: false,
        color_palette: ProjectColorPalette {
            color_primary: Color::from(0x008dcd),
            color_primary_dark: Color::from(0x0084c2),
            color_accent: Color::from(0x008dcd),
            color_control_normal: Color::from(0x57beee),
            color_control_highlight: Color::from(0x008dcd)
        },
        sketchware_version: 150
    };

    let parsed_project = Project::parse(project).unwrap();

    assert_eq!(expected, parsed_project);
    println!("{:?}", parsed_project);
}