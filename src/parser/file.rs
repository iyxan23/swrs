use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::error::{SWRSError, SWRSResult};
use super::Parsable;

#[derive(Debug, Eq, PartialEq)]
pub struct File {
    pub activities: Vec<FileItem>,
    pub custom_views: Vec<FileItem>,
}

impl Parsable for File {
    fn parse(file: &str) -> SWRSResult<File> {
        let mut iterator = file.split("\n");

        #[derive(Eq, PartialEq)]
        enum FileSection {
            Activity,
            CustomView,
            None,
        }

        let mut cur_section = FileSection::None;
        let mut result = File { activities: vec![], custom_views: vec![] };

        loop {
            let line = iterator.next();
            if line.is_none() { break; }
            let line = line.unwrap();

            if line == "@activity" {
                cur_section = FileSection::Activity;
            } else if line == "@customview" {
                cur_section = FileSection::CustomView;

            } else if cur_section != FileSection::None {
                // parse the file item
                let file_item = FileItem::parse(line)?;

                // push the file item to the appropriate section
                if cur_section == FileSection::Activity {
                    &mut result.activities
                } else if cur_section == FileSection::CustomView {
                    &mut result.custom_views
                } else { break }
                    .push(file_item)
            }
        }

        Ok(result)
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        Ok(format!(
            "@activity\n{}\n@customview\n{}",
            self.activities
                .iter()
                .try_fold(String::new(), |acc, i| {
                    Ok(format!("{}\n{}", acc, i.reconstruct()?))
                })?
                .trim(),

            self.custom_views
                .iter()
                .try_fold(String::new(), |acc, i| {
                    Ok(format!("{}\n{}", acc, i.reconstruct()?))
                })?
                .trim(),
        ))
    }
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
    pub orientation: Orientation,
    pub theme: Theme,
}

impl Parsable for FileItem {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> {
        serde_json::from_str(decrypted_content)
            .map_err(|e|SWRSError::ParseError(e.to_string()))
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        serde_json::to_string(self)
            .map_err(|e|SWRSError::ReconstructionError(e.to_string()))
    }
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum FileType {
    Activity = 0,
    CustomView = 1,
    Drawer = 2,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum KeyboardSetting {
    Unspecified = 0,
    Visible = 1,
    Hidden = 2,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum Orientation {
    Portrait = 0,
    Landscape = 1,
    Both = 2,
}

#[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(i8)]
pub enum Theme {
    None = -1,
    Default = 0,
    Actionbar = 1,
    Fullscreen = 2,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ActivityOptions {
    pub toolbar_enabled: bool,
    pub fullscreen_enabled: bool,
    pub drawer_enabled: bool,
    pub fab_enabled: bool,
}

impl ActivityOptions {
    pub fn from_num(num: u8) -> ActivityOptions {
        ActivityOptions {
            toolbar_enabled: num & ActivityOptionMask::Toolbar as u8 == ActivityOptionMask::Toolbar as u8,
            fullscreen_enabled: num & ActivityOptionMask::Fullscreen as u8 == ActivityOptionMask::Fullscreen as u8,
            drawer_enabled: num & ActivityOptionMask::Drawer as u8 == ActivityOptionMask::Drawer as u8,
            fab_enabled: num & ActivityOptionMask::Fab as u8 == ActivityOptionMask::Fab as u8,
        }
    }

    pub fn as_num(&self) -> u8 {
        let mut result = 0u8;

        if self.toolbar_enabled { result |= ActivityOptionMask::Toolbar as u8; }
        if self.fullscreen_enabled { result |= ActivityOptionMask::Fullscreen as u8; }
        if self.drawer_enabled { result |= ActivityOptionMask::Drawer as u8; }
        if self.fab_enabled { result |= ActivityOptionMask::Fab as u8; }

        result
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ActivityOptionMask {
    Toolbar     = 1 << 0,
    Fullscreen  = 1 << 1,
    Drawer      = 1 << 2,
    Fab         = 1 << 3,
}

mod activity_options_parser {
    use serde::{Deserialize, Deserializer, Serializer};
    use super::ActivityOptions;

    pub fn serialize<S>(options: &ActivityOptions, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_u8(options.as_num())
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<ActivityOptions, D::Error> where D: Deserializer<'de> {
        Ok(ActivityOptions::from_num(u8::deserialize(deserializer)?))
    }
}