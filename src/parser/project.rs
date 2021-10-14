use core::fmt;
use chrono::NaiveDate;
use serde::de;
use serde::{Serialize, Deserialize};
use serde_aux::field_attributes::deserialize_number_from_string;
use crate::color::Color;
use crate::error::{SWRSError, SWRSResult};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "sc_id")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    id: u16,

    #[serde(rename = "my_app_name")]
    app_name: String,

    #[serde(rename = "my_ws_name")]
    workspace_name: String,

    #[serde(rename = "my_sc_pkg_name")]
    package_name: String,

    #[serde(rename = "sc_ver_code")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    version_code: u16,

    #[serde(rename = "sc_ver_name")]
    version_name: String,

    #[serde(rename = "my_sc_reg_dt")]
    #[serde(deserialize_with = "deserialize_date_to_timestamp")]
    date_created: u64,
    custom_icon: bool,

    color_primary: Color,
    color_primary_dark: Color,
    color_accent: Color,
    color_control_normal: Color,
    color_control_highlight: Color,

    #[serde(rename = "sketchware_ver")]
    #[serde(deserialize_with = "deserialize_number_from_string")]
    sketchware_version: u8,
}

impl Project {
    pub fn parse(project: &str) -> SWRSResult<Project> {
        serde_json::from_str(project)
            .map_err(|e| SWRSError::ParseError(e.to_string()))
    }
}

fn deserialize_date_to_timestamp<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: de::Deserializer<'de>,
{
    struct CompactDateVisitor;

    impl<'de> de::Visitor<'de> for CompactDateVisitor {
        type Value = u64;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a compact date format")
        }

        fn visit_str<E>(self, v: &str) -> Result<u64, E>
            where
                E: de::Error,
        {
            // parse the date
            let year = v[0..4].parse::<i32>().map_err(E::custom)?;
            let month = v[4..6].parse::<u32>().map_err(E::custom)?;
            let day = v[6..8].parse::<u32>().map_err(E::custom)?;
            let hour = v[8..10].parse::<u32>().map_err(E::custom)?;
            let minute = v[10..12].parse::<u32>().map_err(E::custom)?;
            let second = v[12..14].parse::<u32>().map_err(E::custom)?;

            Ok(
                NaiveDate::from_ymd(year, month, day)
                    .and_hms(hour, minute, second)
                    .timestamp() as u64
            )
        }
    }

    deserializer.deserialize_any(CompactDateVisitor)
}