use serde::{Serialize, Deserialize};
use crate::parser::serde_util::{string_to_u16, date_to_timestamp};
use crate::color::Color;
use crate::error::{SWRSError, SWRSResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "sc_id")]
    #[serde(with = "string_to_u16")]
    id: u16,

    #[serde(rename = "my_app_name")]
    app_name: String,

    #[serde(rename = "my_ws_name")]
    workspace_name: String,

    #[serde(rename = "my_sc_pkg_name")]
    package_name: String,

    #[serde(rename = "sc_ver_code")]
    #[serde(with = "string_to_u16")]
    version_code: u16,

    #[serde(rename = "sc_ver_name")]
    version_name: String,

    #[serde(rename = "my_sc_reg_dt")]
    #[serde(with = "date_to_timestamp")]
    date_created: u64,
    custom_icon: bool,

    color_primary: Color,
    color_primary_dark: Color,
    color_accent: Color,
    color_control_normal: Color,
    color_control_highlight: Color,

    #[serde(rename = "sketchware_ver")]
    sketchware_version: u8,
}

impl Project {
    pub fn parse(project: &str) -> SWRSResult<Project> {
        serde_json::from_str(project)
            .map_err(|e| SWRSError::ParseError(e.to_string()))
    }
}