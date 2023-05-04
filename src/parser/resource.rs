use super::Parsable;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Resource {
    pub images: Vec<ResourceItem>,
    pub sounds: Vec<ResourceItem>,
    pub fonts: Vec<ResourceItem>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResourceSection {
    Images,
    Sounds,
    Fonts,
    None,
}

impl Parsable for Resource {
    type ParseError = ResourceParseError;
    type ReconstructionError = ResourceReconstructionError;

    fn parse(decrypted_content: &str) -> Result<Self, Self::ParseError> {
        let mut iterator = decrypted_content.split("\n");

        let mut cur_section = ResourceSection::None;
        let mut result = Resource {
            images: vec![],
            sounds: vec![],
            fonts: vec![],
        };
        let mut line_count = 0u32;

        loop {
            let line = iterator.next();
            if line.is_none() {
                break;
            }
            let line = line.unwrap();

            if line == "@images" {
                cur_section = ResourceSection::Images;
            } else if line == "@sounds" {
                cur_section = ResourceSection::Sounds;
            } else if line == "@fonts" {
                cur_section = ResourceSection::Fonts
            } else if cur_section != ResourceSection::None {
                // parse the resource item if the line isn't empty
                if line.is_empty() {
                    break;
                }

                let resource_item = ResourceItem::parse(line).map_err(|err| {
                    ResourceParseError::ResourceItemParseError {
                        source: err,
                        section: cur_section,
                        line: line_count,
                    }
                })?;

                // push the resource item to the appropriate section
                if cur_section == ResourceSection::Images {
                    &mut result.images
                } else if cur_section == ResourceSection::Sounds {
                    &mut result.sounds
                } else if cur_section == ResourceSection::Fonts {
                    &mut result.fonts
                } else {
                    break;
                }
                .push(resource_item)
            }

            line_count += 1;
        }

        Ok(result)
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        Ok(format!(
            "@images\n{}@sounds\n{}@fonts\n{}",
            // a macro might be handy
            self.images
                .iter()
                .try_fold(String::new(), |acc, i| Ok(format!(
                    "{}{}\n",
                    acc,
                    i.reconstruct().map_err(|err| {
                        ResourceReconstructionError::ResourceItemReconstructionError {
                            source: err,
                            section: ResourceSection::Images,
                            item: i.to_owned(),
                        }
                    })?
                )))?,
            self.sounds
                .iter()
                .try_fold(String::new(), |acc, i| Ok(format!(
                    "{}{}\n",
                    acc,
                    i.reconstruct().map_err(|err| {
                        ResourceReconstructionError::ResourceItemReconstructionError {
                            source: err,
                            section: ResourceSection::Sounds,
                            item: i.to_owned(),
                        }
                    })?
                )))?,
            self.fonts
                .iter()
                .try_fold(String::new(), |acc, i| Ok(format!(
                    "{}{}\n",
                    acc,
                    i.reconstruct().map_err(|err| {
                        ResourceReconstructionError::ResourceItemReconstructionError {
                            source: err,
                            section: ResourceSection::Fonts,
                            item: i.to_owned(),
                        }
                    })?
                )))?
                .trim(), // i don't want the extra \n at the end of the file
        ))
    }
}

#[derive(Error, Debug)]
pub enum ResourceParseError {
    #[error("error while parsing a resource item of {section:?} at line {line}")]
    ResourceItemParseError {
        #[source]
        source: serde_json::Error,
        section: ResourceSection,
        line: u32,
    },
}

#[derive(Error, Debug)]
pub enum ResourceReconstructionError {
    #[error("error while reconstruction resource item `{item:?}` of section {section:?}")]
    ResourceItemReconstructionError {
        #[source]
        source: serde_json::Error,
        section: ResourceSection,
        item: ResourceItem,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
pub struct ResourceItem {
    /// The filename of the resource
    #[serde(rename = "resFullName")]
    pub full_name: String,

    /// The resource name of this resource
    #[serde(rename = "resName")]
    pub name: String,

    // Unknown usage, this value is always 1
    #[serde(rename = "resType")]
    pub r#type: u8,
}

impl Parsable for ResourceItem {
    type ParseError = serde_json::Error;
    type ReconstructionError = serde_json::Error;

    fn parse(decrypted_content: &str) -> Result<Self, Self::ParseError> {
        serde_json::from_str(decrypted_content)
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        serde_json::to_string(self)
    }
}
