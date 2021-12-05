use std::collections::HashMap;
use std::ops::Index;
use std::str::FromStr;
use ritelinked::LinkedHashMap;
use crate::color::Color;
use crate::parser::logic::Block as RawBlock;
use crate::SWRSError;

pub type Blocks = LinkedHashMap<BlockId, Block>;

#[derive(Debug, Eq, PartialEq)]
pub struct BlockId(pub u32);

/// A model that represents a block
#[derive(Debug, Eq, PartialEq)]
pub struct Block {
    /// The id of this block
    pub id: BlockId,

    /// The id of the next block
    pub next_block: BlockId,

    /// The first substack / nest of this block, gives None if this block doesn't have a substack / nest
    pub sub_stack1: Option<Vec<Block>>,

    /// The second substack / nest of this block, gives None if this block doesn't have a substack / nest
    pub sub_stack2: Option<Vec<Block>>,

    /// The color of this block
    pub color: Color,

    /// The category of this block, this is known from its block color
    pub category: BlockCategory,

    /// The opcode of this block
    pub op_code: String,

    /// The spec of this block
    pub spec: spec::Spec,

    /// The return type of this block
    pub ret_type: String,

    /// The type name of this block (the usage is currently unknown)
    pub type_name: String,
}

/// Category of a block; known from its block color
#[derive(Debug, Eq, PartialEq)]
pub enum BlockCategory {
    Variable,
    List,
    Control,
    Operator,
    Math,
    File,
    ViewFunc,
    ComponentFunc,
    MoreBlock,
}

pub mod spec {
    use std::str::FromStr;
    use crate::{SWRSError, SWRSResult};

    /// A model that represents the spec of a block
    pub struct Spec {
        pub items: Vec<SpecItem>
    }

    impl Spec {
        /// Retrieves all fields / args of this spec
        pub fn get_all_args(&self) -> Vec<&SpecItem> {
            self.items
                .iter()
                .filter_map(|i| if let SpecItem::Field = i { Some(i) } else { None })
                .collect()
        }

        /// Retrieves a specific index on all of the fields / args of this spec
        pub fn get_arg(&self, index: usize) -> Option<&SpecItem> {
            self.get_all_args().get(index)
        }
    }

    impl FromStr for Spec {
        type Err = SWRSError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Spec {
                items: s.split(" ")
                        .map(SpecFieldType::from_str)
                        .collect::<SWRSResult<Vec<SpecItem>>>()?
            })
        }
    }

    impl ToString for Spec {
        fn to_string(&self) -> String {
            self.items
                .iter()
                .fold(String::new(), |acc, item|
                    format!("{} {}", acc, item.to_string())
                )
                .trim_start()
                .to_string()
        }
    }

    pub enum SpecItem {
        Text(String),

        /// A field, or basically arguments
        ///
        /// Examples:
        ///  - `%s` - type with no name
        ///  - `%s.name` - type with a name
        Field {
            field_type: SpecFieldType,
            name: Option<String>,
        },
    }

    impl FromStr for SpecItem {
        type Err = SWRSError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(if s.starts_with("%") {
                let (stype, name) =
                    s.split_once(".")
                        .map(|(stype, name)| (stype, Some(name.to_string())))
                        .unwrap_or_else(|| (s, None));

                SpecItem::Field {
                    field_type: stype.parse()?,
                    name,
                }
            } else { SpecItem::Text(s.to_string()) })
        }
    }

    impl ToString for SpecItem {
        fn to_string(&self) -> String {
            match self {
                SpecItem::Text(content) => content.clone(),
                SpecItem::Field { field_type, name } =>
                    if let Some(name) = name { format!("%{}.{}", field_type.to_string(), name) }
                    else { format!("%{}", field_type.to_string()) }
            }
        }
    }

    /// Types of a field
    pub enum SpecFieldType {
        String,
        Boolean,
        Number,

        /// A menu is a special type that holds a component, it is displayed as a menu for the user
        /// to pick a component from
        Menu,
    }

    impl FromStr for SpecFieldType {
        type Err = SWRSError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "s" => SpecFieldType::String,
                "b" => SpecFieldType::Boolean,
                "d" => SpecFieldType::Number,
                "m" => SpecFieldType::Menu,
                &_ => Err(SWRSError::ParseError(format!(
                    "Unknown spec field type \"{}\", expected s, b, d, or m", s
                )))?
            })
        }
    }

    impl ToString for SpecFieldType {
        fn to_string(&self) -> String {
            match self {
                SpecFieldType::String => "s",
                SpecFieldType::Boolean => "b",
                SpecFieldType::Number => "d",
                SpecFieldType::Menu => "m",
            }.to_string()
        }
    }
}