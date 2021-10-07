use json::JsonValue;
use crate::error::{SWRSResult, SWRSError, ParseError};

#[derive(Debug, PartialEq, Eq)]
pub struct File {
    pub activities: Vec<FileItem>,
    pub custom_views: Vec<FileItem>,
}

impl File {
    pub fn parse<S: AsRef<str>>(file: S) -> SWRSResult<File> {
        let file = file.as_ref();

        let mut activities: Vec<FileItem> = vec![];
        let mut custom_views: Vec<FileItem> = vec![];

        #[derive(PartialEq, Eq)]
        enum FileSection {
            Activity,
            CustomView,
            None
        }

        let mut cur_section = FileSection::None;

        for line in file.split("\n") {
            if line == "@activity" {
                cur_section = FileSection::Activity;
                continue;
            } else if line == "@customview" {
                cur_section = FileSection::CustomView;
                continue;
            }

            let parse_result = json::parse(line);
            if parse_result.is_err() {
                // this should not be an error
                return SWRSResult::Err(
                    SWRSError::ParseError(
                        ParseError {
                            title: "Failed to parse json on a line in file".to_string(),
                            description: format!("Line: \n\t{}\nErr: {}", line, parse_result.unwrap_err())
                        }
                    )
                );
            }

            let parsed_item = parse_result.unwrap();

            // then add the parsed item based off of our section
            if cur_section == FileSection::Activity { &mut activities }
            else if cur_section == FileSection::CustomView { &mut custom_views }
            else { /* this is neither on activity or customview, weird */ continue }
                .push(
                    FileItem::from(parsed_item)
                )
        }

        Ok(File { activities, custom_views })
    }
}

/// ```json
/// {"fileName": "main", "fileType": 0, "keyboardSetting": 0, "options": 1, "orientation": 0, "theme": -1}
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct FileItem {
    pub file_name: String,
    pub file_type: u8,
    pub keyboard_setting: u8,
    pub options: u8,
    pub orientation: u8,
    pub theme: i8,
}

impl From<JsonValue> for FileItem {
    fn from(json: JsonValue) -> Self {
        FileItem {
            file_name: json["fileName"].as_str().unwrap().to_string(),
            file_type: json["fileType"].as_u8().unwrap(),
            keyboard_setting: json["keyboardSetting"].as_u8().unwrap(),
            options: json["options"].as_u8().unwrap(),
            orientation: json["orientation"].as_u8().unwrap(),
            theme: json["theme"].as_i8().unwrap()
        }
    }
}