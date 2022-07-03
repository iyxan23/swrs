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
                    Some(parse_blocks(parser_block.sub_stack1 as u32, blocks)
                        .map_err(|error| BlockConversionError::Substack1ParseError {
                            id,
                            sub_stack1_pointer: parser_block.sub_stack1 as u32,
                            source: Box::new(error)
                        })?)
                },
                sub_stack2: if parser_block.sub_stack2.is_negative() { None } else {
                    Some(parse_blocks(parser_block.sub_stack2 as u32, blocks)
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
    /// Creates an empty instance of [`Blocks`]
    pub fn new() -> Self {
        Self(vec![])
    }

    fn to_block_container(self, starts_with: u32) -> BlockContainer {
        let mut result = Vec::new();
        let mut id_counter = starts_with - 1;
        let mut blocks = self.0.into_iter().peekable();

        while let Some(block) = blocks.next() {
            convert_block(&mut result, block, &mut id_counter, blocks.peek().is_none());
        }

        /// Converts the given block into a [`ParserBlock`] then adds it into the result mutable
        /// borrow using the id from id_counter
        ///
        /// panics when [`ArgValue::BlockPlaceholder`] is encountered
        ///
        /// returns the id of the block
        fn convert_block(
            result: &mut Vec<ParserBlock>,
            block: Block,
            mut id_counter: &mut u32,
            last_block: bool
        ) -> u32 {
            //////////
            // First we process all of the data of this block

            // --- reserve an id for our block at the start (since the actual block is placed at the start)
            *id_counter += 1;
            let block_id = *id_counter;

            // --- then its arguments
            let (parameters, mut block_args, content)
                = process_content(block.content, &mut id_counter);

            // --- then sub stacks
            let mut sub_stack_blocks = Vec::new();
            let sub_stack1_id = block.sub_stack1
                .map(|ss1| {
                    let mut ss1_blocks = ss1.to_block_container(*id_counter + 1);
                    let ss1_last_id = ss1_blocks.0
                        .last()
                        .map(|b| b.id.parse::<i32>().unwrap())
                        .unwrap_or_else(|| -1);

                    sub_stack_blocks.append(&mut ss1_blocks.0);

                    if ss1_last_id != -1 { *id_counter = ss1_last_id as u32; }
                    ss1_last_id
                }).unwrap_or_else(|| -1);

            let sub_stack2_id = block.sub_stack2
                .map(|ss2| {
                    let mut ss2_blocks = ss2.to_block_container(*id_counter + 1);
                    let ss2_last_id = ss2_blocks.0
                        .last()
                        .map(|b| b.id.parse::<i32>().unwrap())
                        .unwrap_or_else(|| -1);

                    sub_stack_blocks.append(&mut ss2_blocks.0);

                    if ss2_last_id != -1 { *id_counter = ss2_last_id as u32; }
                    ss2_last_id
                }).unwrap_or_else(|| -1);

            /////////
            // Actually append stuff

            // append the block itself
            result.push(ParserBlock {
                color: block.color,
                id: block_id.to_string(),
                next_block: if last_block { -1 } else { (*id_counter + 1) as i32 },
                op_code: block.op_code,
                parameters,
                spec: content.to_string(),
                r#type: block.block_type.to_string(),

                // fixme: which do i choose lol
                type_name: block.block_type.get_typename().unwrap_or_else(|| "".to_string()),

                sub_stack1: sub_stack1_id,
                sub_stack2: sub_stack2_id,
            });

            // then its args
            result.append(&mut block_args);

            // and its sub stacks
            result.append(&mut sub_stack_blocks);

            block_id
        }

        /// takes off arguments and block arguments from the provided block content and return them
        fn process_content(block_content: BlockContent, mut id_counter: &mut u32)
            -> (Vec<String>, Vec<ParserBlock>, BlockContent) {

            let (content, args) = block_content.take_args();

            let mut block_args: Vec<ParserBlock> = Vec::new();
            let parameters = args
                .into_iter()
                .map(|arg| match arg {
                    Argument::String { value, .. } => {
                        match value {
                            ArgValue::Value(val) => val,
                            ArgValue::Block(block) => {
                                convert_block(&mut block_args, block, &mut id_counter, true);
                                format!("@{}", id_counter)
                            }
                            ArgValue::BlockPlaceholder { block_id } =>
                                panic!("tries to convert argument to params but encountered an \
                                        untouched block placeholder with id {}", block_id),
                            ArgValue::Empty => "".to_string()
                        }
                    }
                    Argument::Number { value, .. } => {
                        match value {
                            ArgValue::Value(val) => val.to_string(),
                            ArgValue::Block(block) => {
                                convert_block(&mut block_args, block, &mut id_counter, true);
                                format!("@{}", id_counter)
                            }
                            ArgValue::BlockPlaceholder { block_id } =>
                                panic!("tries to convert argument to params but encountered an \
                                        untouched block placeholder with id {}", block_id),
                            ArgValue::Empty => "".to_string()
                        }
                    }
                    Argument::Boolean { value, .. } => {
                        match value {
                            ArgValue::Value(val) => val.to_string(),
                            ArgValue::Block(block) => {
                                convert_block(&mut block_args, block, &mut id_counter, true);
                                format!("@{}", id_counter)
                            }
                            ArgValue::BlockPlaceholder { block_id } =>
                                panic!("tries to convert argument to params but encountered an \
                                        untouched block placeholder with id {}", block_id),
                            ArgValue::Empty => "".to_string()
                        }
                    }
                    Argument::Menu { value, ..} => {
                        match value {
                            ArgValue::Value(val) => val.to_string(),
                            ArgValue::Block(block) => {
                                convert_block(&mut block_args, block, &mut id_counter, true);
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

            (parameters, block_args, content)
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
    /// Creates a simple block without any substacks
    pub fn new(
        category: BlockCategory,
        op_code: String,
        content: BlockContent,
        block_type: BlockType
    ) -> Block {
        Block {
            sub_stack1: None,
            sub_stack2: None,
            color: category.into(),
            op_code,
            content,
            block_type
        }
    }

    /// Creates a new block that has exactly one substack. The block_type is automatically set to
    /// be [`BlockType::Control(BlockControl::OneNest)`]
    pub fn new_1substack(
        category: BlockCategory,
        op_code: String,
        content: BlockContent,
        sub_stack1: Blocks,
    ) -> Block {
        Block {
            sub_stack1: Some(sub_stack1),
            sub_stack2: None,
            color: category.into(),
            op_code,
            content,
            block_type: BlockType::Control(BlockControl::OneNest)
        }
    }

    /// Creates a new block that has exactly two substack. The block_type is automatically set to
    /// be [`BlockType::Control(BlockControl::TwoNest)`]
    pub fn new_2substack(
        category: BlockCategory,
        op_code: String,
        content: BlockContent,
        sub_stack1: Blocks,
        sub_stack2: Blocks,
    ) -> Block {
        Block {
            sub_stack1: Some(sub_stack1),
            sub_stack2: Some(sub_stack2),
            color: category.into(),
            op_code,
            content,
            block_type: BlockType::Control(BlockControl::TwoNest)
        }
    }
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
            " " => BlockType::Regular,
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
            BlockType::Regular => " ",
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

impl Into<Color> for BlockCategory {
    fn into(self) -> Color {
        let (r, g, b) = match self {
            BlockCategory::Variable         => (0xee, 0x7d, 0x16),
            BlockCategory::List             => (0xcc, 0x5b, 0x22),
            BlockCategory::Control          => (0xe1, 0xa9, 0x2a),
            BlockCategory::Operator         => (0x5c, 0xb7, 0x22),
            BlockCategory::Math             => (0x23, 0xb9, 0xa9),
            BlockCategory::File             => (0xa1, 0x88, 0x7f),
            BlockCategory::ViewFunc         => (0x4a, 0x6c, 0xd4),
            BlockCategory::ComponentFunc    => (0xfc, 0xa5, 0xe2),
            BlockCategory::MoreBlock        => (0x8a, 0x55, 0xd7)
        };

        Color::from_rgb(r, g, b)
    }
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

            // check if this thing has a name
            // example: %s.name
            let has_name = s.chars().nth(2)
                .map(|c| c == '.' && s.len() > 3)
                .unwrap_or(false);
            let name = has_name.then(|| (&s[3..]).to_string());

            let arg = match &s.chars().nth(1).unwrap() {
                's' => Argument::String { name, value: ArgValue::Empty },
                'b' => Argument::Boolean { name, value: ArgValue::Empty },
                'd' => Argument::Number { name, value: ArgValue::Empty },
                'm' => Argument::Menu { name: name.unwrap(), value: ArgValue::Empty },
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

            // check if this thing has a name
            // example: %s.name
            let has_name = s.chars().nth(2)
                .map(|c| c == '.' && s.len() > 3)
                .unwrap_or(false);
            let name = has_name.then(|| (&s[3..]).to_string());

            let arg = match &s.chars().nth(1).unwrap() {
                's' => Argument::String {
                    name,
                    value: {
                        if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                        let value = args.remove(0);

                        if value.starts_with("@") {
                            ArgValue::BlockPlaceholder {
                                block_id: (&value[1..]).parse()
                                    .map_err(|_| BlockContentParseError::InvalidBlockId {
                                        value
                                    })?
                            }
                        } else { ArgValue::Value(value) }
                    }
                },
                'b' => Argument::Boolean {
                    name,
                    value: {
                        if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                        let value = args.remove(0);

                        if value.starts_with("@") {
                            ArgValue::BlockPlaceholder {
                                block_id: (&value[1..]).parse()
                                    .map_err(|_| BlockContentParseError::InvalidBlockId {
                                        value
                                    })?
                            }
                        } else {
                            let value = value.parse()
                                .map_err(|_| BlockContentParseError::InvalidBooleanArgument {
                                    value
                                })?;

                            ArgValue::Value(value)
                        }
                    }
                },
                'd' => Argument::Number {
                    name,
                    value: {
                        if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                        let value = args.remove(0);

                        if value.starts_with("@") {
                            ArgValue::BlockPlaceholder {
                                block_id: (&value[1..]).parse()
                                    .map_err(|_| BlockContentParseError::InvalidBlockId {
                                        value
                                    })?
                            }
                        } else {
                            let value = value.parse()
                                .map_err(|_| BlockContentParseError::InvalidNumberArgument {
                                    value
                                })?;

                            ArgValue::Value(value)
                        }
                    }
                },
                'm' => Argument::Menu {
                    name: name.unwrap(),
                    value: {
                        if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                        ArgValue::Value(
                            args.remove(0)
                        )
                    }
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
                    SpecItem::Parameter(Argument::String { name, value }) => {
                        SpecItem::Parameter(Argument::String {
                            name,
                            value: if let ArgValue::Empty = value {
                                if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                                let val = args.remove(0);

                                if val.starts_with("@") {
                                    ArgValue::BlockPlaceholder {
                                        block_id: (&val[1..]).parse()
                                            .map_err(|_| BlockContentParseError::InvalidBlockId {
                                                value: val
                                            })?
                                    }
                                } else { ArgValue::Value(val) }
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Number { name, value }) => {
                        SpecItem::Parameter(Argument::Number {
                            name,
                            value: if let ArgValue::Empty = value {
                                if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                                let value = args.remove(0);

                                if value.starts_with("@") {
                                    ArgValue::BlockPlaceholder {
                                        block_id: (&value[1..]).parse()
                                            .map_err(|_| BlockContentParseError::InvalidBlockId {
                                                value
                                            })?
                                    }
                                } else {
                                    let value = value.parse()
                                        .map_err(|_| BlockContentParseError::InvalidNumberArgument {
                                            value
                                        })?;

                                    ArgValue::Value(value)
                                }
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Boolean { name, value }) => {
                        SpecItem::Parameter(Argument::Boolean {
                            name,
                            value: if let ArgValue::Empty = value {
                                if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                                let value = args.remove(0);

                                if value.starts_with("@") {
                                    ArgValue::BlockPlaceholder {
                                        block_id: (&value[1..]).parse()
                                            .map_err(|_| BlockContentParseError::InvalidBlockId {
                                                value
                                            })?
                                    }
                                } else {
                                    let value = value.parse()
                                        .map_err(|_| BlockContentParseError::InvalidBooleanArgument {
                                            value
                                        })?;

                                    ArgValue::Value(value)
                                }
                            } else { value }
                        })
                    }
                    SpecItem::Parameter(Argument::Menu { name, value }) => {
                        SpecItem::Parameter(Argument::Menu {
                            name,
                            value: if let ArgValue::Empty = value {
                                if args.is_empty() { Err(BlockContentParseError::RanOutOfArgs)? }
                                let value = args.remove(0);

                                if value.starts_with("@") {
                                    ArgValue::BlockPlaceholder {
                                        block_id: (&value[1..]).parse()
                                            .map_err(|_| BlockContentParseError::InvalidBlockId {
                                                value
                                            })?
                                    }
                                } else { ArgValue::Value(value) }
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
                    SpecItem::Parameter(Argument::String { name, value }) => {
                        SpecItem::Parameter(Argument::String {
                            name,
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
                    SpecItem::Parameter(Argument::Number { name, value }) => {
                        SpecItem::Parameter(Argument::Number {
                            name,
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
                    SpecItem::Parameter(Argument::Boolean { name, value }) => {
                        SpecItem::Parameter(Argument::Boolean {
                            name,
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
                    SpecItem::Parameter(Argument::Menu { name, value }) => {
                        SpecItem::Parameter(Argument::Menu {
                            name,
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
                            //
                            // oh yeah the name should be cloned because it is essential
                            let ret = match &arg {
                                Argument::String { name, .. } =>
                                    Argument::String { name: name.as_ref().cloned(), value: ArgValue::Empty },
                                Argument::Number { name, .. } =>
                                    Argument::Number { name: name.as_ref().cloned(), value: ArgValue::Empty },
                                Argument::Boolean { name, .. } =>
                                    Argument::Boolean { name: name.as_ref().cloned(), value: ArgValue::Empty },
                                Argument::Menu { name, .. } =>
                                    Argument::Menu {
                                        name: name.to_owned(),
                                        value: ArgValue::Empty,
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

    pub fn builder() -> BlockContentBuilder {
        BlockContentBuilder { content: vec![] }
    }
}

#[derive(Error, Debug)]
pub enum BlockContentParseError {
    #[error("block with id {block_id} not found")]
    BlockNotFound {
        block_id: u32
    },

    #[error("invalid block id given on params while referencing a block argument: `{value}`")]
    InvalidBlockId {
        value: String
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
        let mut output = String::new();
        for item in &self.items {
            let mut p_name = None;

            output.push_str(match item {
                SpecItem::Text(text) => text.as_str(),
                SpecItem::Parameter(Argument::String { name, .. }) => {
                    p_name = name.as_ref(); "%s"
                },
                SpecItem::Parameter(Argument::Number { name, .. }) => {
                    p_name = name.as_ref(); "%d"
                },
                SpecItem::Parameter(Argument::Boolean { name, .. }) => {
                    p_name = name.as_ref(); "%b"
                },
                SpecItem::Parameter(Argument::Menu { name, .. }) => {
                    p_name = Some(name); "%m"
                },
            });

            if let Some(name) = p_name {
                output.push('.');
                output.push_str(name);
            }

            output.push(' ');
        }
        output.pop(); // remove the trailing space
        output
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum SpecItem {
    Text(String),
    Parameter(Argument)
}

/// Represents an argument in a spec, for instance:
/// ```txt
/// add source directly %s.inputOnly
/// ```
/// `%s.inputOnly` here is an Argument, specifically:
/// ```txt
/// Argument::String {
///     name: Some("inputOnly"),
///     value: ...,
/// }
/// ```
///
/// Note: names on arguments are usually used to hint the type of the field / special field if they
///    are in a block; for instance, `%s` with `.inputOnly` on ASD blocks means that it could only
///    be an "input only" and variables cannot be "inputted" to them.
///
///    Another example is on most view blocks in which they specify exactly what views that can be
///    inputted as an argument:
///    ```txt
///    %m.textview setText %s
///    ```
///
///    But it could also be used as variable names as in moreblock specs. For instance a moreblock
///    spec:
///    ```txt
///    get_data:get data %s.data_name %d.amount
///    ```
///    the name of the `%s` argument is used as the name of the argument of the moreblock, and so
///    does the `%d` argument.
///
// todo: lists and maps lol how did i forgot about them
#[derive(Debug, Clone, PartialEq)]
pub enum Argument {
    // %s.name
    String {
        name: Option<String>,
        value: ArgValue<String>
    },
    // %d.name
    Number {
        name: Option<String>,
        value: ArgValue<f64>
    },
    // %b.name
    Boolean {
        name: Option<String>,
        value: ArgValue<bool>
    },
    // %m.name
    Menu {
        // names are always present in menus
        name: String,
        value: ArgValue<String>
    }
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

/// A very simple [`BlockContent`] builder
pub struct BlockContentBuilder {
    content: Vec<SpecItem>
}

impl BlockContentBuilder {
    /// Adds a text
    pub fn text<S: ToString>(mut self, s: S) -> Self {
        self.content.push(SpecItem::Text(s.to_string()));
        self
    }

    /// Adds an argument
    pub fn arg(mut self, arg: Argument) -> Self {
        self.content.push(SpecItem::Parameter(arg));
        self
    }

    /// Builds a [`BlockContent`] based off of the text/args given
    pub fn build(self) -> BlockContent {
        BlockContent { items: self.content }
    }
}