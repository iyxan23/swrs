use serde::{Deserialize, Serialize};
use crate::error::{SWRSError, SWRSResult};
use super::Parsable;

#[derive(Debug, Eq, PartialEq)]
pub struct Resource {
    pub images: Vec<ResourceItem>,
    pub sounds: Vec<ResourceItem>,
    pub fonts: Vec<ResourceItem>,
}

impl Parsable for Resource {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> {
        let mut iterator = decrypted_content.split("\n");

        #[derive(Eq, PartialEq)]
        enum ResourceSection {
            Images,
            Sounds,
            Fonts,
            None,
        }

        let mut cur_section = ResourceSection::None;
        let mut result = Resource { images: vec![], sounds: vec![], fonts: vec![] };

        loop {
            let line = iterator.next();
            if line.is_none() { break; }
            let line = line.unwrap();

            if line == "@images" {
                cur_section = ResourceSection::Images;
            } else if line == "@sounds" {
                cur_section = ResourceSection::Sounds;
            } else if line == "@fonts" {
                cur_section = ResourceSection::Fonts

            } else if cur_section != ResourceSection::None {
                // parse the resource item
                let resource_item = ResourceItem::parse(line)?;

                // push the resource item to the appropriate section
                if cur_section == ResourceSection::Images {
                    &mut result.images
                } else if cur_section == ResourceSection::Sounds {
                    &mut result.sounds
                } else if cur_section == ResourceSection::Fonts {
                    &mut result.fonts
                } else { break }
                    .push(resource_item)
            }
        }

        Ok(result)
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        Ok(format!(
            "@images\n{}@sounds\n{}@fonts\n{}",
            self.images
                .iter()
                .try_fold(String::new(), |acc, i|
                    Ok(format!("{}{}\n", acc, i.reconstruct()?))
                )?,
            self.sounds
                .iter()
                .try_fold(String::new(), |acc, i|
                    Ok(format!("{}{}\n", acc, i.reconstruct()?))
                )?,
            self.fonts
                .iter()
                .try_fold(String::new(), |acc, i|
                    Ok(format!("{}{}\n", acc, i.reconstruct()?))
                )?
                .trim(), // i don't want the extra \n at the end of the file
        ))
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ResourceItem {
    #[serde(rename = "resFullName")]
    pub full_name: String,

    #[serde(rename = "resName")]
    pub name: String,

    #[serde(rename = "resType")]
    pub r#type: u8,
}

impl Parsable for ResourceItem {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> {
        serde_json::from_str(decrypted_content)
            .map_err(|e|SWRSError::ParseError(e.to_string()))
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        serde_json::to_string(self)
            .map_err(|e|SWRSError::ParseError(e.to_string()))
    }
}