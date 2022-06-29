use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;
use ritelinked::LinkedHashMap;
use crate::color::Color;
use crate::parser::logic::BlockContainer;
use crate::parser::logic::Block as ParserBlock;
use thiserror::Error;

/// An abstraction over the blockchain model of sketchware, doesn't store the block ids. It
/// generates them on conversion into BlockContainer.
#[derive(Debug, Clone, PartialEq)]
pub struct Blocks(pub Vec<Block>);

// converts a block container into an API struct Blocks
impl TryFrom<BlockContainer> for Blocks {
    type Error = BlockConversionError;

    fn try_from(value: BlockContainer) -> Result<Self, Self::Error> {
        let mut blocks = value.0
            .into_iter()
            .try_fold(LinkedHashMap::<u32, ParserBlock>::new(), |mut acc, block| {
                acc.insert(
                    u32::from_str(block.id.as_str())
                        .map_err(|err| BlockConversionError::MalformedBlockId {
                            id: block.id.to_owned(),
                            source: err
                        })?,
                    block
                );

                Ok(acc)
            })?;

        fn parse_block(
            id: u32, blocks: &mut LinkedHashMap<u32, ParserBlock>
        ) -> Result<Block, BlockConversionError> {
            let parser_block = blocks.remove(&id)
                .ok_or_else(|| BlockConversionError::BlockNotFound { id })?;

            Ok(Block {
                sub_stack1: if parser_block.sub_stack1.is_negative() { None } else {
                    Some(parse_blocks(id, blocks)
                        .map_err(|error| BlockConversionError::Substack1ParseError {
                            id,
                            sub_stack1_pointer: parser_block.sub_stack1 as u32,
                            source: Box::new(error)
                        })?)
                },
                sub_stack2: if parser_block.sub_stack2.is_negative() { None } else {
                    Some(parse_blocks(id, blocks)
                        .map_err(|error| BlockConversionError::Substack2ParseError {
                            id,
                            sub_stack2_pointer: parser_block.sub_stack2 as u32,
                            source: Box::new(error)
                        })?)
                },
                color: parser_block.color,
                op_code: parser_block.op_code,
                content: BlockContent::parse(parser_block.spec, parser_block.parameters)
                    .map_err(|err| BlockConversionError::BlockContentParseError {
                        source: err
                    })?
                    .apply_blocks(|id| parse_block(id, blocks).ok())
                    .map_err(|err| BlockConversionError::BlockContentParseError {
                        source: err
                    })?,
                block_type: BlockType::from(&parser_block.r#type, parser_block.type_name)
                    .map_err(|err| BlockConversionError::InvalidType { source: err })?
            })
        }

        fn parse_blocks(
            starting_id: u32, blocks: &mut LinkedHashMap<u32, ParserBlock>
        ) -> Result<Blocks, BlockConversionError> {
            let mut current_id = starting_id;
            let mut result = Vec::new();

            loop {
                let next_block = {
                    blocks.get(&current_id)
                        .ok_or_else(|| BlockConversionError::BlockNotFound { id: current_id })?
                        .next_block
                };

                result.push(parse_block(current_id, blocks)?);

                if next_block.is_negative() { break }
                current_id = next_block as u32;
            }

            Ok(Blocks(result))
        }

        if let Some((id, _)) = blocks.front() {
            parse_blocks(*id, &mut blocks)
        } else {
            Ok(Default::default())
        }
    }
}

#[derive(Error, Debug)]
pub enum BlockConversionError {
    #[error("malformed block id: {id}: {:?}", .source)]
    MalformedBlockId {
        id: String,
        source: ParseIntError
    },

    #[error("block with id {id} not found")]
    BlockNotFound {
        id: u32
    },

    #[error("error while parsing substack1 of block with id `{id}`")]
    Substack1ParseError {
        id: u32,
        sub_stack1_pointer: u32,
        source: Box<BlockConversionError>,
    },

    #[error("error while parsing substack2 of block with id `{id}`")]
    Substack2ParseError {
        id: u32,
        sub_stack2_pointer: u32,
        source: Box<BlockConversionError>,
    },

    #[error("invalid block type: `{}`", .source.block_type)]
    InvalidType {
        source: InvalidBlockType
    },

    #[error("error while parsing the block content: {source:?}")]
    BlockContentParseError {
        source: BlockContentParseError
    }
}

impl Blocks {
    fn to_block_container(self, starts_with: u32) -> BlockContainer {
        let mut result = Vec::new();
        let mut id_counter = starts_with - 1;
        let mut blocks = self.0.into_iter().peekable();

        while let Some(block) = blocks.next() {
            convert_block(
                &mut result, block, &mut id_counter, None,
                blocks.peek().is_none()
            );
        }

        /// Converts the given block into a [`ParserBlock`] then adds it into the result mutable
        /// borrow using the id from id_counter
        ///
        /// panics when [`ArgValue::BlockPlaceholder`] is encountered
        ///
        /// returns the id of the block
        fn convert_block(
            mut result: &mut Vec<ParserBlock>,
            block: Block,
            mut id_counter: &mut u32,
            type_name: Option<String>,
            last_block: bool
        ) -> u32 {
            //////////
            // First we process all of the data of this block
            let (content, args) = block.content.take_args();

            // takes out block arguments and generate a list of parameters that stores the value
            // of arguments or points to the block arguments we've popped off
            let mut block_args = Vec::new();
            let parameters = args
                .into_iter()
                .map(|arg| match arg {
                    Argument::String { value } => {
                        match value {
                            ArgValue::Value(val) => val,
                            ArgValue::Block(block) => {
                                convert_block(&mut result, block, &mut id_counter, None, false);
                                format!("@{}", id_counter)
                            }
                            ArgValue::BlockPlaceholder { block_id } =>
                                panic!("tries to convert argument to params but encountered an \
                                        untouched block placeholder with id {}", block_id),
                            ArgValue::Empty => "".to_string()
                        }
                    }
                    Argument::Number { value } => {
                        match value {
                            ArgValue::Value(val) => val.to_string(),
                            ArgValue::Block(block) => {
                                convert_block(&mut result, block, &mut id_counter, None, false);
                                format!("@{}", id_counter)
                            }
                            ArgValue::BlockPlaceholder { block_id } =>
                                panic!("tries to convert argument to params but encountered an \
                                        untouched block placeholder with id {}", block_id),
                            ArgValue::Empty => "".to_string()
                        }
                    }
                    Argument::Boolean { value } => {
                        match value {
                            ArgValue::Value(val) => val.to_string(),
                            ArgValue::Block(block) => {
                                convert_block(&mut result, block, &mut id_counter, None, false);
                                format!("@{}", id_counter)
                            }
                            ArgValue::BlockPlaceholder { block_id } =>
                                panic!("tries to convert argument to params but encountered an \
                                        untouched block placeholder with id {}", block_id),
                            ArgValue::Empty => "".to_string()
                        }
                    }
                    Argument::Menu { value, type_name } => {
                        match value {
                            ArgValue::Value(val) => val.to_string(),
                            ArgValue::Block(block) => {
                                convert_block(&mut result, block, &mut id_counter, Some(type_name), false);
                                format!("@{}", id_counter)
                            }
                            ArgValue::BlockPlaceholder { block_id } =>
                                panic!("tries to convert argument to params but encountered an \
                                        untouched block placeholder with id {}", block_id),
                            ArgValue::Empty => "".to_string()
                        }
                    }
                })
                .collect();

            // since argument block starts before the actual block, we need to get our block id
            // and leave the next ids for the substacks (because they come after the actual block)
            *id_counter += 1;
            let block_id = *id_counter;

            // parse sub stack blocks and add them to a list
            let mut sub_stack_blocks = Vec::new();
            let sub_stack1_id = block.sub_stack1
                .map(|ss1| {
                    let mut ss1_blocks = ss1.to_block_container(*id_counter);
                    let ss1_last_id = ss1_blocks.0
                        .last()
                        .map(|b| b.id.parse::<i32>().unwrap())
                        .unwrap_or_else(|| -1);

                    sub_stack_blocks.append(&mut ss1_blocks.0);

                    *id_counter = ss1_last_id as u32;
                    ss1_last_id
                }).unwrap_or_else(|| -1);

            let sub_stack2_id = block.sub_stack2
                .map(|ss2| {
                    let mut ss2_blocks = ss2.to_block_container(*id_counter);
                    let ss2_last_id = ss2_blocks.0
                        .last()
                        .map(|b| b.id.parse::<i32>().unwrap())
                        .unwrap_or_else(|| -1);

                    sub_stack_blocks.append(&mut ss2_blocks.0);

                    *id_counter = ss2_last_id as u32;
                    ss2_last_id
                }).unwrap_or_else(|| -1);

            /////////
            // Actually append stuff

            // append the block arguments of this block, it comes before the actual block
            result.append(&mut block_args);

            // then the actual block itself
            result.push(ParserBlock {
                color: block.color,
                id: block_id.to_string(),
                next_block: if last_block { -1 } else { (*id_counter + 1) as i32 },
                op_code: block.op_code,
                parameters,
                spec: content.to_string(),
                r#type: block.block_type.to_string(),

                // fixme: which do i choose lol
                type_name: type_name.or(block.block_type.get_typename())
                    .unwrap_or_else(|| "".to_string()),

                sub_stack1: sub_stack1_id,
                sub_stack2: sub_stack2_id,
            });

            // after the actual block, its substacks gets added
            result.append(&mut sub_stack_blocks);

            block_id
        }

        BlockContainer(result)
    }
}

impl Into<BlockContainer> for Blocks {
    fn into(self) -> BlockContainer {
        self.to_block_container(10)
    }
}

impl Default for Blocks {
    fn default() -> Self {
        Self(vec![])
    }
}

impl IntoIterator for Blocks {
    type Item = Block;
    type IntoIter = std::vec::IntoIter<Block>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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

    /// The content of this block (names and arguments)
    pub content: BlockContent,

    /// The type of this block
    pub block_type: BlockType,
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
#[derive(Debug, Clone, PartialEq)]
pub enum BlockType {
    Regular,
    Argument(ArgumentBlockReturnType),
    Control(BlockControl),
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArgumentBlockReturnType {
    Boolean,
    String,
    Number,
    View { type_name: String },
    Component { type_name: String }
}

#[derive(Debug, Clone, PartialEq)]
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
            _ => Err(InvalidBlockType { block_type: s.to_string() })?
        })
    }

    /// Retrieves the typename of this block (if any)
    pub fn get_typename(&self) -> Option<String> {
        match self {
            BlockType::Argument(ArgumentBlockReturnType::View { type_name }) =>
                Some(type_name.to_string()),
            BlockType::Argument(ArgumentBlockReturnType::Component { type_name }) =>
                Some(type_name.to_string()),
            _ => None
        }
    }
}

#[derive(Debug, Error)]
#[error("invalid block type: {block_type}")]
pub struct InvalidBlockType {
    pub block_type: String
}

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

/// A model that stores the appearance of a block and its arguments
#[derive(Debug, Clone, PartialEq)]
pub struct BlockContent {
    pub items: Vec<SpecItem>
}

impl BlockContent {
    /// Parses a spec without any arguments, leaves the argument values as Empty
    pub fn parse_wo_params(spec: &str) -> Result<Self, BlockContentParseError> {
        let mut items = Vec::new();
        for s in spec.split(" ") {
            if !s.starts_with("%") {
                items.push(SpecItem::Text(s.to_string()));
                continue
            }

            let arg = match &s.chars().nth(1).unwrap() {
                's' => Argument::String { value: ArgValue::Empty },
                'b' => Argument::Boolean { value: ArgValue::Empty },
                'd' => Argument::Number { value: ArgValue::Empty },
                'm' => Argument::Menu { type_name: s[3..].to_string(), value: ArgValue::Empty },
                _ => Err(BlockContentParseError::UnknownSpecParam {
                    name: s.chars().nth(1).unwrap().to_string(),
                    full: s.to_string()
                })?
            };

            items.push(SpecItem::Parameter(arg));
        }

        Ok(Self { items })
    }

    /// Parses a spec and leaves the block args with [`ArgValue::BlockPlaceholder`]
    pub fn parse(spec: String, mut args: Vec<String>) -> Result<Self, BlockContentParseError> {
        let mut items = Vec::new();
        for s in spec.split(" ") {
            if !s.starts_with("%") {
                items.push(SpecItem::Text(s.to_string()));
                continue
            }

            let arg = match &s.chars().nth(1).unwrap() {
                's' => Argument::String {
                    value: ArgValue::Value(
                        args.pop()
                            .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?
                    )
                },
                'b' => {
                    let value =
                        args.pop()
                            .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?;

                    let value = value.parse()
                        .map_err(|_| BlockContentParseError::InvalidBooleanArgument { value })?;

                    Argument::Boolean { value: ArgValue::Value(value) }
                },
                'd' => {
                    let value =
                        args.pop()
                            .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?;

                    let value = value.parse()
                        .map_err(|_| BlockContentParseError::InvalidBooleanArgument { value })?;

                    Argument::Number { value: ArgValue::Value(value) }
                },
                'm' => Argument::Menu {
                    type_name: s[3..].to_string(),
                    value: ArgValue::Value(
                        args.pop()
                            .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?
                    )
                },
                _ => Err(BlockContentParseError::UnknownSpecParam {
                    name: s.chars().nth(1).unwrap().to_string(),
                    full: s.to_string()
                })?
            };

            items.push(SpecItem::Parameter(arg));
        }

        Ok(Self { items })
    }

    /// Applies the args to the arguments with the value [`ArgValue::Empty`]
    pub fn apply_args(self, mut args: Vec<String>) -> Result<Self, BlockContentParseError> {
        Ok(Self {
            items: self.items.into_iter()
                .map(|item| Ok(match item {
                    SpecItem::Text(val) => SpecItem::Text(val),
                    SpecItem::Parameter(Argument::String { value }) => {
                        SpecItem::Parameter(Argument::String {
                            value: if let ArgValue::Empty = value {
                                ArgValue::Value(
                                    args.pop()
                                        .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?
                                )
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Number { value }) => {
                        SpecItem::Parameter(Argument::Number {
                            value: if let ArgValue::Empty = value {
                                let value =
                                    args.pop()
                                        .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?;

                                let value = value.parse()
                                    .map_err(|_| BlockContentParseError::InvalidBooleanArgument {
                                        value
                                    })?;

                                ArgValue::Value(value)
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Boolean { value }) => {
                        SpecItem::Parameter(Argument::Boolean {
                            value: if let ArgValue::Empty = value {
                                let value =
                                    args.pop()
                                        .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?;

                                let value = value.parse()
                                    .map_err(|_| BlockContentParseError::InvalidBooleanArgument {
                                        value
                                    })?;

                                ArgValue::Value(value)
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Menu { type_name, value }) => {
                        SpecItem::Parameter(Argument::Menu {
                            type_name,
                            value: if let ArgValue::Empty = value {
                                ArgValue::Value(
                                    args.pop()
                                        .ok_or_else(|| BlockContentParseError::RanOutOfArgs)?
                                )
                            } else { value }
                        })
                    }
                }))
                .collect::<Result<_, _>>()?
        })
    }

    /// Applies blocks of block placeholders arguments of this block
    pub fn apply_blocks<F>(self, mut get_block: F) -> Result<Self, BlockContentParseError>
    where F: FnMut(u32) -> Option<Block> {
        Ok(Self {
            items: self.items.into_iter()
                .map(|item| Ok(match item {
                    SpecItem::Text(val) => SpecItem::Text(val),
                    SpecItem::Parameter(Argument::String { value }) => {
                        SpecItem::Parameter(Argument::String {
                            value: if let ArgValue::BlockPlaceholder { block_id } = value {
                                ArgValue::Block(
                                    get_block(block_id)
                                        .ok_or_else(|| BlockContentParseError::BlockNotFound {
                                            block_id
                                        })?
                                )
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Number { value }) => {
                        SpecItem::Parameter(Argument::Number {
                            value: if let ArgValue::BlockPlaceholder { block_id } = value {
                                ArgValue::Block(
                                    get_block(block_id)
                                        .ok_or_else(|| BlockContentParseError::BlockNotFound {
                                            block_id
                                        })?
                                )
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Boolean { value }) => {
                        SpecItem::Parameter(Argument::Boolean {
                            value: if let ArgValue::BlockPlaceholder { block_id } = value {
                                ArgValue::Block(
                                    get_block(block_id)
                                        .ok_or_else(|| BlockContentParseError::BlockNotFound {
                                            block_id
                                        })?
                                )
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Menu { type_name, value }) => {
                        SpecItem::Parameter(Argument::Menu {
                            type_name,
                            value: if let ArgValue::BlockPlaceholder { block_id } = value {
                                ArgValue::Block(
                                    get_block(block_id)
                                        .ok_or_else(|| BlockContentParseError::BlockNotFound {
                                            block_id
                                        })?
                                )
                            } else { value }
                        })
                    }
                })).collect::<Result<_, _>>()?
        })
    }

    /// Retrieves all the arguments of this block
    pub fn get_args(&self) -> Vec<&Argument> {
        self.items
            .iter()
            .filter_map(|item| {
                match item {
                    SpecItem::Text(_) => None,
                    SpecItem::Parameter(arg) => Some(arg)
                }
            })
            .collect()
    }

    /// Retrieves all the arguments as a mutable reference
    pub fn get_args_mut(&mut self) -> Vec<&mut Argument> {
        self.items
            .iter_mut()
            .filter_map(|item| {
                match item {
                    SpecItem::Text(_) => None,
                    SpecItem::Parameter(arg) => Some(arg)
                }
            })
            .collect()
    }

    /// Takes the arguments from the block content, returns them as a [`Vec<Argument>`] and
    /// replaces them with [`ArgValue::Empty`]
    pub fn take_args(self) -> (Self, Vec<Argument>) {
        let mut arguments = Vec::new();
        let reconstructed = Self {
            items: self.items
                .into_iter()
                .map(|item| {
                    if let SpecItem::Parameter(arg) = item {
                        SpecItem::Parameter({
                            // create a new instance of Argument depending on the type of the
                            // argument before, then set them to be empty
                            let ret = match &arg {
                                Argument::String { .. } =>
                                    Argument::String { value: ArgValue::Empty },
                                Argument::Number { .. } =>
                                    Argument::Number { value: ArgValue::Empty },
                                Argument::Boolean { .. } =>
                                    Argument::Boolean { value: ArgValue::Empty },
                                Argument::Menu { type_name, .. } =>
                                    Argument::Menu {
                                        value: ArgValue::Empty,
                                        type_name: type_name.to_string()
                                    },
                            };

                            // push the owned argument
                            arguments.push(arg);

                            ret
                        })
                    } else { item }
                })
                .collect()
        };

        (reconstructed, arguments)
    }
}

#[derive(Error, Debug)]
pub enum BlockContentParseError {
    #[error("block with id {block_id} not found")]
    BlockNotFound {
        block_id: u32
    },

    #[error("unknown spec parameter: {name}")]
    UnknownSpecParam {
        name: String,
        full: String
    },

    #[error("not enough arguments is given")]
    RanOutOfArgs,

    #[error("invalid argument given on a number-typed parameter: {value}")]
    InvalidNumberArgument {
        value: String,
    },

    #[error("invalid argument given on a boolean-typed parameter: {value}")]
    InvalidBooleanArgument {
        value: String
    },
}

impl ToString for BlockContent {
    fn to_string(&self) -> String {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecItem {
    Text(String),
    Parameter(Argument)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    String { value: ArgValue<String> },
    Number { value: ArgValue<i32> },
    Boolean { value: ArgValue<bool> },
    Menu { type_name: String, value: ArgValue<String> }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ArgValue<T: Debug + Clone + PartialEq> {
    Value(T),
    Block(Block),

    // used when we have a block param but we only have the ID
    BlockPlaceholder { block_id: u32 },

    // when there is no value passed to this parameter
    Empty
}