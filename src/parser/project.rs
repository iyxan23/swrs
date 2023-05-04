use super::Parsable;
use crate::color::Color;
use crate::parser::serde_util::{date_to_timestamp, string_to_u16};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ProjectColorPalette {
    pub color_primary: Color,
    pub color_primary_dark: Color,
    pub color_accent: Color,
    pub color_control_normal: Color,
    pub color_control_highlight: Color,
}

impl Default for ProjectColorPalette {
    fn default() -> Self {
        ProjectColorPalette {
            color_primary: 0xff008dcd.into(),
            color_primary_dark: 0xff0084c2.into(),
            color_accent: 0xff008dcd.into(),
            color_control_normal: 0xff57beee.into(),
            color_control_highlight: 0x20008dcd.into(),
        }
    }
}

impl Parsable for Project {
    type ParseError = serde_json::Error;
    type ReconstructionError = serde_json::Error;

    fn parse(project: &str) -> Result<Project, Self::ParseError> {
        serde_json::from_str(project)
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        serde_json::to_string(self)
    }
}
