use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::error::{SWRSError, SWRSResult};

#[derive(Debug, Eq, PartialEq)]
pub struct File {
    pub activities: Vec<FileItem>,
    pub custom_views: Vec<FileItem>,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct FileItem {
    #[serde(rename = "fileName")]
    pub filename: String,

    #[serde(rename = "fileType")]
    pub file_type: FileType,

    #[serde(rename = "keyboardSetting")]
    pub keyboard_setting: KeyboardSetting,

    #[serde(with = "activity_options_parser")]
    pub options: ActivityOptions,
    pub orientation: u8,
    pub theme: i8,
}

impl FileItem {
    pub fn parse(file_item: &str) -> SWRSResult<FileItem> {
        serde_json::from_str(file_item).map_err(|e|SWRSError::ParseError(e.to_string()))
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum FileType {
    Activity = 0,
    CustomView = 1,
    Drawer = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum KeyboardSetting {
    Unspecified = 0,
    Visible = 1,
    Hidden = 2,
}

#[derive(Debug, Eq, PartialEq)]
pub struct ActivityOptions {
    pub toolbar_enabled: bool,
    pub fullscreen_enabled: bool,
    pub drawer_enabled: bool,
    pub fab_enabled: bool,
}

impl ActivityOptions {
    pub fn from_num(num: u8) -> ActivityOptions {
        ActivityOptions {
            toolbar_enabled: num & ActivityOptionMask::Toolbar == ActivityOptionMask::Toolbar,
            fullscreen_enabled: num & ActivityOptionMask::Fullscreen == ActivityOptionMask::Fullscreen,
            drawer_enabled: num & ActivityOptionMask::Drawer == ActivityOptionMask::Drawer,
            fab_enabled: num & ActivityOptionMask::Fab == ActivityOptionMask::Fab,
        }
    }

    pub fn into_num(self) -> u8 {
        let mut result = 0u8;

        if self.toolbar_enabled { result |= ActivityOptionMask::Toolbar; }
        if self.fullscreen_enabled { result |= ActivityOptionMask::Fullscreen; }
        if self.drawer_enabled { result |= ActivityOptionMask::Drawer; }
        if self.fab_enabled { result |= ActivityOptionMask::Fab; }

        result
    }
}

#[derive(Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum ActivityOptionMask {
    Toolbar     = 1 << 0,
    Fullscreen  = 1 << 1,
    Drawer      = 1 << 2,
    Fab         = 1 << 3,
}

mod activity_options_parser {
    use serde::{Deserialize, Deserializer, Serializer};
    use serde::de::Error;
    use super::ActivityOptions;

    pub fn serialize<S>(options: ActivityOptions, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_u8(options.into_num())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ActivityOptions, D::Error> where D: Deserializer<'de> {
        Ok(ActivityOptions::from_num(u8::deserialize(deserializer)?))
    }
}