use crate::error::{SWRSResult, SWRSError, ParseError};
use chrono::NaiveDate;
use std::fmt::{Formatter, Display, Debug};
use crate::color::Color;
use crate::parser::SWRSParser;
use crate::utils::get_and_check;

/// A struct that stores project metadata (came from the data/project file)
///
/// To parse a project JSON, simply do:
/// ```
/// use swrs::parser::project::Project;
///
/// // ...
/// let parsed_project = Project::parse(decrypted_project).unwrap();
/// // ...
/// ```
///
/// Decrypted project (formatted):
/// ```json
/// {
///     "custom_icon": false,
///     "sc_ver_code": "1",
///     "my_ws_name": "Workspace Name",
///     "color_accent": -1.6740915E7,
///     "my_app_name": "App Name",
///     "sc_ver_name": "1.0",
///     "sc_id": "649",
///     "color_primary": -1.6740915E7,
///     "color_control_highlight": 5.36907213E8,
///     "color_control_normal": -1.1026706E7,
///     "my_sc_reg_dt": "20201220074905",
///     "sketchware_ver": 150,
///     "my_sc_pkg_name": "com.iyxan23.something",
///     "color_primary_dark": -1.674323E7
/// }
/// ```
///
/// These values are mapped into these fields:
///
/// | Field name       | `Project` fields     | Notes                                                               |
/// | ---------------- | -------------------- | ------------------------------------------------------------------- |
/// | `custom_icon`    | `custom_icon`        |                                                                     |
/// | `sc_id`          | `project_id`         |                                                                     |
/// | `my_app_name`    | `app_name`           |                                                                     |
/// | `my_sc_pkg_name` | `package_name`       |                                                                     |
/// | `my_ws_name`     | `workspace_name`     |                                                                     |
/// | `sc_ver_code`    | `version_code`       |                                                                     |
/// | `sc_ver_name`    | `version_name`       |                                                                     |
/// | `my_sc_reg_dt`   | `time_created`       | The creation date is stored in a millis timestamp form (u64)        |
/// | `sketchware_ver` | `sketchware_version` |                                                                     |
/// | All color values | `colors` (Colors)    | All of the color fields are stored in a different [`Colors`] struct |
///
/// [`Colors`]: crate::parser::Colors
///
#[derive(Debug)]
pub struct Project {
    pub custom_icon: bool,
    pub project_id: u16,
    pub app_name: String,
    pub package_name: String,
    pub workspace_name: String,
    pub version_code: u8,
    pub version_name: String,
    pub colors: Colors,
    pub time_created: u64,
    pub sketchware_version: u8,
}

impl SWRSParser<Project> for Project {
    fn parse(project: String) -> SWRSResult<Project> {
        let parsed = json::parse(project.as_ref());

        if parsed.is_err() {
            let err = parsed.unwrap_err();
            return Err(SWRSError::ParseError(
                ParseError {
                    title: "Failed parsing project data".to_string(),
                    description: format!("{}", err),
                }
            ))
        }

        let project_json = parsed.unwrap();

        Ok(Project {
            custom_icon: get_and_check(&project_json, "custom_icon")?.as_bool().unwrap(),
            project_id: get_and_check(&project_json, "sc_id")?.as_str().unwrap().parse::<u16>().unwrap(),
            app_name: get_and_check(&project_json, "my_app_name")?.as_str().unwrap().to_string(),
            package_name: get_and_check(&project_json, "my_sc_pkg_name")?.as_str().unwrap().to_string(),
            workspace_name: get_and_check(&project_json, "my_ws_name")?.as_str().unwrap().to_string(),
            version_code: get_and_check(&project_json, "sc_ver_code")?.as_str().unwrap().parse::<u8>().unwrap(),
            version_name: get_and_check(&project_json, "sc_ver_name")?.as_str().unwrap().to_string(),
            colors: Colors {
                primary: Color::from(get_and_check(&project_json, "color_primary")?.as_f32().unwrap() as i32 as u32),
                primary_dark: Color::from(get_and_check(&project_json, "color_primary_dark")?.as_f32().unwrap() as i32 as u32),
                accent: Color::from(get_and_check(&project_json, "color_accent")?.as_f32().unwrap() as i32 as u32),
                control_normal: Color::from(get_and_check(&project_json, "color_control_normal")?.as_f32().unwrap() as i32 as u32),
                control_highlight: Color::from(get_and_check(&project_json, "color_control_highlight")?.as_f32().unwrap() as i32 as u32)
            },
            time_created: {
                let date = project_json["my_sc_reg_dt"].as_str().unwrap();

                // parse the date
                let year = date[0..4].parse::<i32>().unwrap();
                let month = date[4..6].parse::<u32>().unwrap();
                let day = date[6..8].parse::<u32>().unwrap();
                let hour = date[8..10].parse::<u32>().unwrap();
                let minute = date[10..12].parse::<u32>().unwrap();
                let second = date[12..14].parse::<u32>().unwrap();

                NaiveDate::from_ymd(year, month, day).and_hms(hour, minute, second).timestamp() as u64
            },
            sketchware_version: project_json["sketchware_ver"].as_u8().unwrap()
        })
    }
}

#[derive(Debug)]
pub struct Colors {
    pub primary: Color,
    pub primary_dark: Color,
    pub accent: Color,
    pub control_normal: Color,
    pub control_highlight: Color,
}