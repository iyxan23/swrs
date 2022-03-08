use std::num::ParseIntError;
use std::str::FromStr;
use crate::LinkedHashMap;
use crate::color::Color;
use crate::parser::logic::BlockContainer;
use thiserror::Error;
use crate::api::block::block_content::FieldValue;

type ParserBlock = crate::parser::logic::Block;

/// A struct that basically stores blocks with its starting id
#[derive(Debug, Clone, PartialEq)]
pub struct Blocks {
    pub starting_id: Option<BlockId>,
    pub blocks: LinkedHashMap<BlockId, Block>
}

impl Blocks {
    /// Inserts a block at the end of the blockchain, returns its id
    pub fn push_block(&mut self, mut block: Block) -> BlockId {
        let last_id = self.blocks
            .back()
            .map(|i|i.0.clone())
            .unwrap_or(BlockId(0));

        // get next id
        let new_id = BlockId(last_id.0 + 1);

        // set next_block of the last block to our new id, if it exists
        if let Some(last_block) = self.blocks.get_mut(&last_id) {
            last_block.next_block = Some(new_id);
        }

        // set the new id
        block.id = new_id;

        // then insert our new shiny block
        self.blocks.insert(new_id, block);

        // then return our new shiny block id
        new_id
    }

    /// Removes a block in the block chain, rewires the block before it to the block
    /// after it. Returns false when it failed to do so, can be either the block doesn't exist, or
    /// it couldn't find the block before it.
    pub fn remove_block(&mut self, id: BlockId) -> bool {
        let next_block = if let Some(to_be_removed_block) = self.blocks.get(&id) {
            to_be_removed_block.next_block
        } else {
            return false;
        };

        // check if we're the start of the block chain
        if let Some(starting_id) = self.starting_id {
            if starting_id == id {
                // we need to set the starting id to the next block
                if let Some(next_block) = next_block {
                    self.starting_id = Some(next_block);
                } else {
                    // else then the block chain is empty, there is no block after us and we're the
                    // start of this block chain
                    self.starting_id = None;
                }

                return true;
            }
        }

        // there is a block before us, find the block that has next_block referenced to us
        //
        // we can't just loop back and check if its a block, because it can be that the block
        // before us has a substack and those children are put after that block
        //
        // [ block we wanted ]
        //   [ substack ]
        //   [ substack ]
        // [ us ]

        // loop back until we reach Some() and that block contains next_block that references us
        let mut block_before: &mut Block = self.blocks.get_mut(&id).unwrap();
        loop {
            // check if it's next_block matches our id
            if let Some(next_block) = block_before.next_block {
                if next_block == id {
                    // nice we got it!
                    break;
                }
            }

            // find the block before it
            let mut block_id_before = BlockId(block_before.id.0 - 1);
            while !self.blocks.contains_key(&block_id_before) {
                // check if we've reached the bottom
                if block_id_before.0 == 0 {
                    // meh, there isn't any block before us that references us, that's weird
                    // todo: Result
                    return false;
                }

                block_id_before.0 -= 1;
            }

            block_before = self.blocks.get_mut(&block_id_before).unwrap();
        }

        // we then modify that block to reference the next block (relative to the removed block)
        if let Some(next_block) = next_block {
            block_before.next_block = Some(next_block);
        }

        // we are done!

        true
    }
}

// converts a block container into an API struct Blocks
impl TryFrom<BlockContainer> for Blocks {
    type Error = BlockConversionError;

    fn try_from(value: BlockContainer) -> Result<Self, Self::Error> {
        // first we map them into a LinkedHashMap and associate with each blocks' id
        let mut blocks = value.0
            .into_iter()
            .try_fold(LinkedHashMap::<BlockId, ParserBlock>::new(), |mut acc, block| {
                acc.insert(
                    BlockId::from_str(block.id.as_str())
                        .map_err(|err| BlockConversionError::MalformedBlockId {
                            id: block.id.to_owned(),
                            source: err
                        })?,
                    block
                );

                Ok(acc)
            })?;

        /// a utility function that simply turns an i32 into u32 or if its negative it will return
        /// None
        fn to_u32_else_none(val: i32) -> Option<u32> {
            if val.is_negative() { None } else { Some(val as u32) }
        }

        /// Converts a parser block with the given id into an api Block
        fn parse_a_block(
            id: BlockId, mut blocks: &mut LinkedHashMap<BlockId, ParserBlock>
        ) -> Result<Block, BlockConversionError> {

            // first we get the block from the current id
            let p_block = blocks.remove(&id)
                .ok_or_else(|| BlockConversionError::BlockNotFound { id })?;

            // then we convert it to our own Block struct
            let block = Block {
                id,
                next_block: to_u32_else_none(p_block.next_block).map(|val| BlockId(val)),

                // if the substack1 is negative, then there is no substack1, else we parse the
                // blocks starting from that substack1
                sub_stack1: if p_block.sub_stack1.is_negative() { Ok(None) } else {
                    parse_to_blocks(BlockId(p_block.sub_stack1 as u32), blocks)
                        .map(|it| Some(it))
                }.map_err(|error| BlockConversionError::Substack1ParseError {
                    id,
                    sub_stack1_pointer: BlockId(p_block.sub_stack1 as u32),
                    source: Box::new(error)
                })?,

                // if the substack2 is negative, then there is no substack2, else we parse the
                // blocks starting from that substack2
                sub_stack2: if p_block.sub_stack2.is_negative() { Ok(None) } else {
                    parse_to_blocks(BlockId(p_block.sub_stack2 as u32), blocks)
                        .map(|it| Some(it))
                }.map_err(|error| BlockConversionError::Substack2ParseError {
                    id,
                    sub_stack2_pointer: BlockId(p_block.sub_stack2 as u32),
                    source: Box::new(error)
                })?,

                color: p_block.color,
                op_code: p_block.op_code,
                content: {
                    let content =
                        block_content::BlockContent::parse_from(
                            p_block.spec.as_str(),
                            Some(p_block.parameters),
                            |id| {
                                parse_a_block(id, &mut blocks)
                            }
                        ).map_err(|err| BlockConversionError::MalformedContent {
                            id,
                            source: err
                        })?;

                    content
                },
                ret_type: p_block.r#type,
                type_name: p_block.type_name,
            };

            Ok(block)
        }

        /// A recursive function that parses blocks given from its id and follows the blocks'
        /// next_block until it stops. This function gets its blocks from the [`blocks`]
        /// variable defined above
        fn parse_to_blocks(
            starting_id: BlockId, mut blocks: &mut LinkedHashMap<BlockId, ParserBlock>
        ) -> Result<Blocks, BlockConversionError> {

            let mut result = LinkedHashMap::<BlockId, Block>::new();
            let mut current_id = starting_id;

            loop {
                let block = parse_a_block(current_id, &mut blocks)?;
                let next_block = block.next_block;

                // we've successfully converted the block let's add them to the result
                result.insert(current_id, block);

                // if the next block is none then we stop
                if let Some(id) = next_block {
                    // set this as our next id
                    current_id = id;
                } else {
                    // ok we stop
                    break;
                }
            }

            Ok(Blocks {
                // get the first element and get its id
                starting_id: result.iter().nth(0).map(|(id, _block)| id.to_owned()),
                blocks: result
            })
        }

        // get the first block from the blocks list, then parse the blocks after it or if there
        // isn't any, it will just return default
        if let Some(id) = blocks.iter().nth(0) {
            Ok(parse_to_blocks(id.0.to_owned(), &mut blocks)?)
        } else {
            Ok(Default::default())
        }
    }
}

#[derive(Error, Debug)]
pub enum BlockConversionError {
    #[error("couldn't parse block id as an integer: `{id}`")]
    MalformedBlockId {
        id: String,
        source: ParseIntError
    },

    #[error("couldn't find the block with id `{}`", .id.0)]
    BlockNotFound {
        id: BlockId,
    },

    #[error("malformed block content on the block with id `{}`", .id.0)]
    MalformedContent {
        id: BlockId,
        source: block_content::BlockContentParseError
    },

    #[error("error while parsing substack1 of block with id `{}`", .id.0)]
    Substack1ParseError {
        id: BlockId,
        sub_stack1_pointer: BlockId,
        source: Box<BlockConversionError>,
    },

    #[error("error while parsing substack2 of block with id `{}`", .id.0)]
    Substack2ParseError {
        id: BlockId,
        sub_stack2_pointer: BlockId,
        source: Box<BlockConversionError>,
    },
}

impl Into<BlockContainer> for Blocks {
    fn into(self) -> BlockContainer {
        let mut block_container = BlockContainer(vec![]);

        fn convert_block(block: Block, result: &mut Vec<ParserBlock>) {
            // first we flatten its children
            let mut ss_1_container = BlockContainer(vec![]);
            let mut ss_1_id = -1i32;

            if let Some(blocks) = block.sub_stack1 {
                ss_1_id = blocks.starting_id.map(|i|i.0 as i32).unwrap_or(-1);
                flatten_blocks(blocks, &mut ss_1_container);
            }

            let mut ss_2_container = BlockContainer(vec![]);
            let mut ss_2_id = -1i32;

            if let Some(blocks) = block.sub_stack2 {
                ss_2_id = blocks.starting_id.map(|i|i.0 as i32).unwrap_or(-1);
                flatten_blocks(blocks, &mut ss_2_container);
            }

            // do the actual block conversion
            result.push(ParserBlock {
                color: block.color,
                id: block.id.0.to_string(),
                next_block: block.next_block.map(|n|n.0 as i32).unwrap_or(-1),
                op_code: block.op_code,
                spec: block.content.to_string(),
                parameters: block.content.cloned_args()
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
                sub_stack1: ss_1_id,
                sub_stack2: ss_2_id,
                r#type: block.ret_type,
                type_name: block.type_name
            });

            // after that, we parse the blocks that's in the block's fields
            for field in block.content.cloned_args() {
                if let FieldValue::Block(block) = field {
                    let mut field_blk_blocks = vec![];
                    convert_block(block, &mut field_blk_blocks);

                    // and append em to our result
                    result.append(&mut field_blk_blocks);
                }
            }

            // then finally push its children that's in substack1 or 2
            result.append(&mut ss_1_container.0);
            result.append(&mut ss_2_container.0);

            // remember, the order is quite important (for sketchware)
        }

        fn flatten_blocks(blocks: Blocks, result: &mut BlockContainer) {
            for block in blocks {
                convert_block(block, &mut result.0);
            }
        }

        flatten_blocks(self, &mut block_container);

        block_container
    }
}

impl FromIterator<Block> for Blocks {
    fn from_iter<T: IntoIterator<Item=Block>>(iter: T) -> Self {
        let blocks = iter
            .into_iter()
            .map(|b: Block| (b.id, b))
            .collect::<LinkedHashMap<BlockId, Block>>();

        Blocks {
            starting_id: blocks.front().map(|i| i.0.clone()),
            blocks
        }
    }
}

impl IntoIterator for Blocks {
    type Item = Block;
    type IntoIter = BlocksIterator;

    fn into_iter(self) -> Self::IntoIter {
        BlocksIterator {
            next_block_id: self.starting_id,
            blocks: self,
        }
    }
}

impl Default for Blocks {
    fn default() -> Self {
        Blocks {
            starting_id: None,
            blocks: Default::default()
        }
    }
}

/// An iterator that iterates over [`Blocks`] using each blocks' id and next_block
pub struct BlocksIterator {
    blocks: Blocks,
    next_block_id: Option<BlockId>,
}

impl Iterator for BlocksIterator {
    type Item = Block;

    fn next(&mut self) -> Option<Self::Item> {
        match self.next_block_id {
            None => None,
            Some(next_block_id) => {
                // get the next block
                if let Some(block) = self.blocks.blocks.remove(&next_block_id) {
                    // checks if this block has a next block
                    if let Some(next_block) = block.next_block {
                        // yes, update the next block id
                        self.next_block_id = Some(next_block);
                    } else {
                        // apparently not, it seems like we're at the end of this iteration
                        self.next_block_id = None;
                    }

                    Some(block)
                } else { None }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
pub struct BlockId(pub u32);

impl FromStr for BlockId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BlockId(s.parse::<u32>()?))
    }
}

/// A model that represents a block
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    /// The id of this block
    pub id: BlockId,

    /// The id of the next block. None if the value is -1 (the end of the block chain)
    pub next_block: Option<BlockId>,

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
            (_, _, _) => Err(UnknownColor { color: value })?
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
    use crate::api::block::{Block, BlockConversionError, BlockId};

    #[derive(Debug, Clone, PartialEq)]
    pub struct BlockContent {
        items: Vec<SpecItem>,
    }

    impl BlockContent {
        pub fn parse_from<F: FnMut(BlockId) -> Result<Block, BlockConversionError>>(
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

        /// Parses from a spec string without arguments
        pub fn parse_from_wo_args(spec: &str) -> Result<Self, BlockContentParseError> {
            BlockContent::parse_from(spec, None, |_| panic!("shouldn't be called"))
        }

        // make remove_args(&mut self) -> Vec<FieldValue>
        // this thing is hard to implement

        pub fn cloned_args(&self) -> Vec<FieldValue> {
            self.get_args().into_iter().cloned().collect()
        }

        pub fn get_args(&self) -> Vec<&FieldValue> {
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
        pub fn parse_from<F: FnMut(BlockId) -> Result<Block, BlockConversionError>>(
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
                        Some(FieldValue::parse_from(
                            args.pop()
                                .ok_or_else(|| SpecItemParseError::NotEnoughArgs)?
                                .as_str(),
                            get_block)?)
                    } else { None }
                }
            } else { SpecItem::Text(s.to_string()) })
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum FieldValue {
        Text(String),
        Block(Block)
    }

    impl FieldValue {
        pub fn parse_from<F: FnMut(BlockId) -> Result<Block, BlockConversionError>>(
            s: &str,
            mut get_block: F
        ) -> Result<Self, SpecItemParseError> {
            Ok(if s.starts_with("@") {
                let block_id = BlockId((&s[1..]).parse::<u32>()
                    .map_err(|err| SpecItemParseError::MalformedParameterBlockId {
                        content: s.to_string(),
                        source: err
                    })?);

                FieldValue::Block(
                    get_block(block_id.to_owned())
                        .map_err(|err| SpecItemParseError::BlockArgConversionError {
                            block_id,
                            source: Box::new(err)
                        })?
                )
            } else { FieldValue::Text(s.to_string()) })
        }
    }

    impl ToString for FieldValue {
        fn to_string(&self) -> String {
            match self {
                FieldValue::Text(text) => text.to_owned(),
                FieldValue::Block(block) => format!("@{}", block.id.0)
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