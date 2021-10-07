extern crate swrs;

use swrs::parser::project::Project;

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

    println!("{:?}", Project::parse(project).unwrap());
}