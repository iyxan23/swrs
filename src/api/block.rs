use std::num::ParseIntError;
use std::str::FromStr;
use crate::color::Color;
use crate::parser::logic::BlockContainer;
use thiserror::Error;

type ParserBlock = crate::parser::logic::Block;

/// An abstraction over the blockchain model of sketchware, doesn't store the block ids. It
/// generates them on conversion into BlockContainer.
#[derive(Debug, Clone, PartialEq)]
type Blocks = Vec<Block>;

// converts a block container into an API struct Blocks
impl TryFrom<BlockContainer> for Blocks {
    type Error = BlockConversionError;

    fn try_from(value: BlockContainer) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[derive(Error, Debug)]
pub struct BlockConversionError {

}

impl Into<BlockContainer> for Blocks {
    fn into(self) -> BlockContainer {
        todo!()
    }
}

/// A model that represents a block
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The first substack / nest of this block, gives None if this block doesn't have a substack / nest
    pub sub_stack1: Option<Blocks>,

    /// The second substack / nest of this block, gives None if this block doesn't have a substack / nest
    pub sub_stack2: Option<Blocks>,

    /// The color of this block
    pub color: Color,

    /// The opcode of this block
    pub op_code: String,

    /// The spec of this block
    pub content: block_content::BlockContent,

    /// The return type of this block
    pub ret_type: String,

    /// The type name of this block (the usage is currently unknown)
    pub type_name: String,
}

impl Block {
    /// Retrieves what category this block is from. Will return an error if the block color doesn't
    /// match to any block category
    pub fn category(&self) -> Result<BlockCategory, UnknownColor> {
        BlockCategory::try_from(self.color)
    }
}

// all of block types can be seen here
// https://github.com/Iyxan23/sketchware-data/blob/main/data/block-opcodes.md
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlockType {
    Regular,
    Argument(ArgumentBlockReturnType),
    Control(BlockControl),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ArgumentBlockReturnType {
    Boolean,
    String,
    Number,
    View { type_name: String },
    Component { type_name: String }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlockControl {
    OneNest, // if block
    TwoNest, // ifElse block
    EndingBlock // finish/break block
}

impl BlockType {
    pub fn from(s: &str, type_name: String) -> Result<Self, InvalidBlockType> {
        Ok(match s {
            "v" => BlockType::Argument(ArgumentBlockReturnType::View { type_name }),
            "p" => BlockType::Argument(ArgumentBlockReturnType::Component { type_name }),
            "b" => BlockType::Argument(ArgumentBlockReturnType::Boolean),
            "s" => BlockType::Argument(ArgumentBlockReturnType::String),
            "d" => BlockType::Argument(ArgumentBlockReturnType::Number),
            "c" => BlockType::Control(BlockControl::OneNest),
            "e" => BlockType::Control(BlockControl::TwoNest),
            "f" => BlockType::Control(BlockControl::EndingBlock),
            "" => BlockType::Regular,
            _ => Err(InvalidBlockType)?
        })
    }
}

pub struct InvalidBlockType;

impl ToString for BlockType {
    fn to_string(&self) -> String {
        match self {
            BlockType::Regular => "",
            BlockType::Argument(ArgumentBlockReturnType::Boolean) => "b",
            BlockType::Argument(ArgumentBlockReturnType::String) => "s",
            BlockType::Argument(ArgumentBlockReturnType::Number) => "d",
            BlockType::Argument(ArgumentBlockReturnType::View { .. }) => "v",
            BlockType::Argument(ArgumentBlockReturnType::Component { .. }) => "p",
            BlockType::Control(BlockControl::OneNest) => "c",
            BlockType::Control(BlockControl::TwoNest) => "e",
            BlockType::Control(BlockControl::EndingBlock) => "f",
        }.to_string()
    }
}

/// Category of a block; known from its block color
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

impl TryFrom<Color> for BlockCategory {
    type Error = UnknownColor;

    fn try_from(value: Color) -> Result<Self, Self::Error> {
        Ok(match value.rgb() {
            (0xee, 0x7d, 0x16) => BlockCategory::Variable,
            (0xcc, 0x5b, 0x22) => BlockCategory::List,
            (0xe1, 0xa9, 0x2a) => BlockCategory::Control,
            (0x5c, 0xb7, 0x22) => BlockCategory::Operator,
            (0x23, 0xb9, 0xa9) => BlockCategory::Math,
            (0xa1, 0x88, 0x7f) => BlockCategory::File,
            (0x4a, 0x6c, 0xd4) => BlockCategory::ViewFunc,
            (0xfc, 0xa5, 0xe2) => BlockCategory::ComponentFunc,
            (0x8a, 0x55, 0xd7) => BlockCategory::MoreBlock,
            (    _,    _,   _) => Err(UnknownColor { color: value })?
        })
    }
}

#[derive(Error, Debug)]
#[error("color {color} does not correlate to any block category")]
pub struct UnknownColor {
    pub color: Color
}

pub mod block_content {
    use std::cell::RefCell;
    use std::num::ParseIntError;
    use std::str::FromStr;
    use thiserror::Error;
    use crate::LinkedHashMap;
    use crate::api::block::{BlockEntry, BlockConversionError, BlockId};
    use super::Block;

    #[derive(Debug, Clone, PartialEq)]
    pub struct BlockContent {
        items: Vec<SpecItem>,
    }

    impl BlockContent {
        /// Creates a new [`BlockContent`] with no arguments
        pub fn new(spec: &str) -> Result<Self, BlockContentParseError> {
            BlockContent::new_args(spec, None, |_| unreachable!())
        }

        /// Creates a new [`BlockContent`] with the specified spec and arguments.
        /// Note: the argument `get_block` is used to retrieve block arguments through their id.
        pub fn new_args<F: FnMut(u32) -> Result<Block, BlockConversionError>>(
            spec: &str,
            arguments: Option<Vec<String>>,
            mut get_block: F
        ) -> Result<Self, BlockContentParseError> {
            let mut result = Vec::new();
            let arguments = RefCell::new(arguments);

            for (idx, value) in spec.split(" ").enumerate() {
                result.push(
                    SpecItem::parse_from(value, &mut arguments.borrow_mut(), &mut get_block)
                        .map_err(|err| BlockContentParseError::SpecItemParseError {
                            index: idx as u32,
                            source: err
                        })?
                );
            }

            Ok(BlockContent { items: result })
        }

        pub fn remove_args(mut self) -> (Self, Vec<FieldValue>) {
            let mut args = Vec::new();

            self.items = self.items
                .into_iter()
                .map(|item| {
                    if let SpecItem::Field {
                        field_type, name, value
                    } = item {
                        if let Some(value) = value { args.push(value); }

                        SpecItem::Field {
                            field_type, name, value: None
                        }
                    } else { item }
                })
                .collect();

            (self, args)
        }

        pub fn cloned_args(&self) -> Vec<FieldValue> {
            self.get_args_vec().into_iter().cloned().collect()
        }

        pub fn get_args(&self) -> LinkedHashMap<String, &FieldValue> {
            self.items.iter()
                .filter_map(|i| {
                    if let SpecItem::Field { name, value, .. } = i {
                        if name.is_some() && value.is_some() {
                            Some((name.as_ref().unwrap().clone(), value.as_ref().unwrap()))
                        } else { None }
                    } else { None }
                })
                .collect()
        }

        pub fn get_arg_names(&self) -> Vec<&str> {
            self.items.iter()
                .filter_map(|i| {
                    if let SpecItem::Field { name, .. } = i {
                        if let Some(name) = name {
                            Some(name.as_str())
                        } else { None }
                    } else { None }
                })
                .collect()
        }

        pub fn get_args_vec(&self) -> Vec<&FieldValue> {
            self.items.iter()
                .filter_map(|i|
                    // weird code, too lazy to make it prettier lol
                    if let SpecItem::Field { value, .. } = i {
                        if let Some(v) = value {
                            Some(v)
                        } else { None }
                    } else { None })
                .collect()
        }
    }

    #[derive(Error, Debug)]
    pub enum BlockContentParseError {
        #[error("error whilst parsing a spec item index `{index}`")]
        SpecItemParseError {
            index: u32,
            source: SpecItemParseError
        }
    }

    impl ToString for BlockContent {
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

    #[derive(Debug, Clone, PartialEq)]
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

            /// None if it doesn't support values (eg spec on MoreBlock)
            value: Option<FieldValue>
        },
    }

    impl SpecItem {
        // small note: sketchware named their arguments field as "parameters" for some reason, so
        // don't get confused by it. this "arguments" parameter is taking that ^
        pub fn parse_from<F: FnMut(u32) -> Result<Block, BlockConversionError>>(
            s: &str,
            arguments: &mut Option<Vec<String>>,
            get_block: F
        ) -> Result<Self, SpecItemParseError> {
            Ok(if s.starts_with("%") {
                let (stype, name) =
                    s.split_once(".")
                        .map(|(stype, name)| (stype, Some(name.to_string())))
                        .unwrap_or_else(|| (s, None));

                SpecItem::Field {
                    field_type: stype.parse().map_err(SpecItemParseError::UnknownSpecFieldType)?,
                    name,
                    // .map wont work since i need to propagate the Error
                    value: if let Some(args) = arguments {
                        if args.is_empty() { Err(SpecItemParseError::NotEnoughArgs)? }

                        Some(FieldValue::parse_from(
                            args.remove(0).as_str(),
                            get_block)?)
                    } else { None }
                }
            } else { SpecItem::Text(s.to_string()) })
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum FieldValue {
        Text(String),
        Block {
            block: Block,
            id: u32
        }
    }

    impl FieldValue {
        pub fn parse_from<F: FnMut(u32) -> Result<Block, BlockConversionError>>(
            s: &str,
            mut get_block: F
        ) -> Result<Self, SpecItemParseError> {
            Ok(if s.starts_with("@") {
                let block_id = s[1..].parse::<u32>()
                    .map_err(|err| SpecItemParseError::MalformedParameterBlockId {
                        content: s.to_string(),
                        source: err
                    })?;

                FieldValue::Block {
                    block: get_block(block_id)
                        .map_err(|err| SpecItemParseError::BlockArgConversionError {
                            block_id,
                            source: Box::new(err)
                        })?,
                    id: block_id
                }
            } else { FieldValue::Text(s.to_string()) })
        }
    }

    impl ToString for FieldValue {
        fn to_string(&self) -> String {
            match self {
                FieldValue::Text(text) => text.to_owned(),
                FieldValue::Block { id, ..} => format!("@{}", id)
            }
        }
    }

    #[derive(Error, Debug)]
    pub enum SpecItemParseError {
        #[error("couldn't turn a block parameter's id into an int: `{content}`")]
        MalformedParameterBlockId {
            content: String,
            source: ParseIntError
        },
        #[error("error whilst parsing a block parameter with id `{}`", .block_id.0)]
        BlockArgConversionError {
            block_id: BlockId,
            source: Box<BlockConversionError>
        },
        #[error("not enough arguments is supplied")]
        NotEnoughArgs,
        #[error("")]
        UnknownSpecFieldType(#[from] UnknownSpecFieldType)
    }

    impl ToString for SpecItem {
        fn to_string(&self) -> String {
            match self {
                SpecItem::Text(content) => content.to_owned(),
                SpecItem::Field { field_type, name, .. } =>
                    if let Some(name) = name {
                        format!("%{}.{}", field_type.to_string(), name)
                    } else {
                        format!("%{}", field_type.to_string())
                    }
            }
        }
    }

    /// Types of a field
    #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
    pub enum SpecFieldType {
        String,
        Boolean,
        Number,

        /// A menu is a special type that holds a component, it is displayed as a menu for the user
        /// to pick a component from
        Menu,
    }

    impl FromStr for SpecFieldType {
        type Err = UnknownSpecFieldType;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(match s {
                "%s" => SpecFieldType::String,
                "%b" => SpecFieldType::Boolean,
                "%d" => SpecFieldType::Number,
                "%m" => SpecFieldType::Menu,
                &_ => Err(UnknownSpecFieldType {
                    value: s.to_string()
                })?
            })
        }
    }

    #[derive(Error, Debug)]
    #[error("unknown spec field type {value}, expected %s, %b, %d, or %m")]
    pub struct UnknownSpecFieldType {
        pub value: String
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