use serde::{Serialize, Deserialize};
use crate::parser::serde_util::{string_to_u16, date_to_timestamp};
use crate::color::Color;
use crate::error::{SWRSError, SWRSResult};
use super::ProjectData;

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct Project {
    #[serde(rename = "sc_id")]
    #[serde(with = "string_to_u16")]
    pub id: u16,

    #[serde(rename = "my_app_name")]
    pub app_name: String,

    #[serde(rename = "my_ws_name")]
    pub workspace_name: String,

    #[serde(rename = "my_sc_pkg_name")]
    pub package_name: String,

    #[serde(rename = "sc_ver_code")]
    #[serde(with = "string_to_u16")]
    pub version_code: u16,

    #[serde(rename = "sc_ver_name")]
    pub version_name: String,

    #[serde(rename = "my_sc_reg_dt")]
    #[serde(with = "date_to_timestamp")]
    pub date_created: u64,
    pub custom_icon: bool,

    #[serde(flatten)]
    pub color_palette: ProjectColorPalette,

    #[serde(rename = "sketchware_ver")]
    pub sketchware_version: u8,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ProjectColorPalette {
    pub color_primary: Color,
    pub color_primary_dark: Color,
    pub color_accent: Color,
    pub color_control_normal: Color,
    pub color_control_highlight: Color,
}

impl ProjectData for Project {
    fn parse(project: &str) -> SWRSResult<Project> {
        serde_json::from_str(project)
            .map_err(|e| SWRSError::ParseError(e.to_string()))
    }

    fn reconstruct(&self) -> SWRSResult<&str> {
        todo!()
    }
}