use json::JsonValue;
use crate::error::{SWRSResult, SWRSError, ParseError};

#[derive(Debug, PartialEq, Eq)]
pub struct Resource {
    pub images: Vec<ResourceItem>,
    pub sounds: Vec<ResourceItem>,
    pub fonts: Vec<ResourceItem>,
}

impl Resource {
    pub fn parse<S: AsRef<str>>(file: S) -> SWRSResult<Resource> {
        let file = file.as_ref();

        let mut images: Vec<ResourceItem> = vec![];
        let mut sounds: Vec<ResourceItem> = vec![];
        let mut fonts: Vec<ResourceItem> = vec![];

        #[derive(PartialEq, Eq)]
        enum ResourceSection {
            Images,
            Sounds,
            Fonts,
            None
        }

        let mut cur_section = ResourceSection::None;

        for (ln, line) in file.split("\n").enumerate() {
            // checks for section headers like images, sounds, and fonts
            if line == "@images" {
                cur_section = ResourceSection::Images;
            } else if line == "@sounds" {
                cur_section = ResourceSection::Sounds;
            } else if line == "@fonts" {
                cur_section = ResourceSection::Fonts;
            } else {
                // this must be an item
                let parse_result = json::parse(line);
                if parse_result.is_err() {
                    return SWRSResult::Err(
                        SWRSError::ParseError(
                            ParseError {
                                title: format!("Failed to parse json at line {} in file", ln),
                                description: format!("Line {}: \n\t{}\nErr: {}", ln, line, parse_result.unwrap_err())
                            }
                        )
                    );
                }

                let parsed_item = parse_result.unwrap();

                // then add the parsed item based off of our section
                if cur_section == ResourceSection::Images { &mut images }
                else if cur_section == ResourceSection::Sounds { &mut sounds }
                else if cur_section == ResourceSection::Fonts { &mut fonts }
                else { /* weird */ continue }
                    .push(
                        ResourceItem::from(parsed_item)
                    )
            }
        }

        Ok(Resource { images, sounds, fonts })
    }
}

/// ```json
/// {"resFullName":"hello_world.png","resName":"hello_world","resType":1}
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct ResourceItem {
    full_name: String,
    name: String,
    r#type: u8,
}

impl From<JsonValue> for ResourceItem {
    fn from(json: JsonValue) -> Self {
        ResourceItem {
            full_name: json["resFullName"].as_str().unwrap().to_string(),
            name: json["resName"].as_str().unwrap().to_string(),
            r#type: json["resType"].as_u8().unwrap()
        }
    }
}