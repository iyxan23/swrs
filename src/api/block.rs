use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;
use crate::LinkedHashMap;
use crate::color::Color;
use crate::parser::logic::BlockContainer;
use thiserror::Error;
use crate::api::block::block_content::FieldValue;

type ParserBlock = crate::parser::logic::Block;

/// A chain of blocks
#[derive(Debug, Clone, PartialEq)]
pub struct Blocks {
    starting_id: Option<BlockId>,
    blocks: HashMap<BlockId, BlockEntry>,
    ending_id: Option<BlockId> // for faster pushes
}

impl Blocks {
    /// Inserts a block at the end of the blockchain, returns its id
    pub fn push_block(&mut self, mut block: BlockEntry) -> BlockId {
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
        let mut block_before: &mut BlockEntry = self.blocks.get_mut(&id).unwrap();
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

    /// "Rechains" the blocks (and its substacks recursively) to a sequential ID starting with the
    /// `start_id` if Some, else starts at id 10.
    pub fn re_chain_blocks(mut self, start_id: Option<BlockId>) -> Self {
        if self.starting_id.is_none() {
            return Self { starting_id: None, blocks: Default::default(), ending_id: None };
        }

        let mut previous_block: Option<&mut BlockEntry> = None;
        let mut new_blocks = HashMap::new();

        let mut current_id = self.starting_id.unwrap();
        let starting_id = start_id.unwrap_or_else(|| BlockId(10));
        let mut current_new_id = starting_id;

        loop {
            // first we pull the block entry from the blocks map
            let mut block = self.blocks.remove(&current_id)
                .expect(format!(
                    "block with id {current_id} doesn't exist while rechaining blocks"
                ).as_str());

            // set our new shiny id
            block.id = current_new_id;

            // set the previous block's next id to point to this block
            if let Some(prev_block) = previous_block {
                prev_block.next_block = Some(current_new_id);
            }

            // recursively "rechain" the substacks
            block.data.sub_stack1 = block.data.sub_stack1
                .map(|ss1| {
                    let new = ss1.re_chain_blocks(Some(current_new_id));
                    current_new_id.0 += new.blocks.len();
                    new
                });

            block.data.sub_stack2 = block.data.sub_stack2
                .map(|ss2| {
                    let new = ss2.re_chain_blocks(Some(current_new_id));
                    current_new_id.0 += new.blocks.len();
                    new
                });

            let next_block = block.next_block;
            let block_new_id = block.id;

            // then finally insert this block to a new map and set the previous_block to be us
            new_blocks.insert(block_new_id, block);
            previous_block = Some(new_blocks.get_mut(&block_new_id).unwrap());

            if let Some(next_block) = next_block {
                current_id = next_block;
                current_new_id.0 += 1;
            } else {
                break
            }
        }

        Self {
            starting_id: Some(starting_id),
            blocks: new_blocks,
            ending_id: Some(current_id)
        }
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

        /// Converts a parser block with the given id into a block entry
        fn parse_a_block(
            id: BlockId, mut blocks: &mut LinkedHashMap<BlockId, ParserBlock>
        ) -> Result<BlockEntry, BlockConversionError> {

            // first we get the block from the current id
            let p_block = blocks.remove(&id)
                .ok_or_else(|| BlockConversionError::BlockNotFound { id })?;

            // then we convert it to our own Block struct
            let block = BlockEntry {
                id,
                next_block: to_u32_else_none(p_block.next_block).map(|val| BlockId(val)),
                data: BlockData {
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
                    opcode: p_block.op_code,
                    content: {
                        let content =
                            block_content::BlockContent::new_args(
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
                    block_type: p_block.r#type.parse()
                        .map_err(|_| BlockConversionError::InvalidBlockType {
                            raw: p_block.r#type
                        })?,
                    type_name: p_block.type_name,
                }
            };

            Ok(block)
        }

        /// A recursive function that parses blocks given from its id and follows the blocks'
        /// next_block until it stops. This function gets its blocks from the [`blocks`]
        /// variable defined above
        fn parse_to_blocks(
            starting_id: BlockId, mut blocks: &mut LinkedHashMap<BlockId, ParserBlock>
        ) -> Result<Blocks, BlockConversionError> {

            let mut result = HashMap::<BlockId, BlockEntry>::new();
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
                starting_id: Some(starting_id),
                blocks: result,
                ending_id: Some(current_id)
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

    #[error("invalid block type: `{raw}`")]
    InvalidBlockType {
        raw: String
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

        fn convert_block(block_entry: BlockEntry, result: &mut Vec<ParserBlock>) {
            let block = block_entry.data;

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
                id: block_entry.id.0.to_string(),
                next_block: block_entry.next_block.map(|n|n.0 as i32).unwrap_or(-1),
                op_code: block.opcode,
                spec: block.content.to_string(),
                parameters: block.content.cloned_args()
                    .into_iter()
                    .map(|v| v.to_string())
                    .collect(),
                sub_stack1: ss_1_id,
                sub_stack2: ss_2_id,
                r#type: block.block_type.to_string(),
                type_name: block.type_name
            });

            // after that, we parse the blocks that's in the block's fields
            for field in block.content.remove_args().1 {
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

impl FromIterator<BlockData> for Blocks {
    fn from_iter<T: IntoIterator<Item=BlockData>>(iter: T) -> Self {
        // from the logic I've observed, blocks starts with the id of 10
        let mut current_block_id = BlockId(10);

        let mut blocks = iter
            .into_iter()
            .map(|b: BlockData| {
                let res = (
                    current_block_id,
                    BlockEntry { id: current_block_id, cur, data: b }
                );

                current_block_id.0 += 1;

                res
            })
            .collect::<HashMap<BlockId, BlockEntry>>();

        let last_block = if !blocks.is_empty() {
            Some(BlockId(current_block_id.0 - 1))
        } else { None };

        // sets the last block's next_block to be None
        if let Some(last_block) = &last_block {
            if let Some(block) = blocks.get_mut(last_block) {
                block.next_block = None;
            }
        }

        Blocks {
            // if last_block.is_none() is true, means that there is no item here
            starting_id: if last_block.is_none() { None } else { Some(BlockId(10)) },
            blocks,
            ending_id: last_block
        }
    }
}

impl IntoIterator for Blocks {
    type Item = BlockEntry;
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
            blocks: Default::default(),
            ending_id: None
        }
    }
}

/// An iterator that iterates over [`Blocks`] using each blocks' id and next_block
pub struct BlocksIterator {
    blocks: Blocks,
    next_block_id: Option<BlockId>,
}

impl Iterator for BlocksIterator {
    type Item = BlockEntry;

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

/// An entry of [`Blocks`]. Contains the ID and the next block of this block
#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntry {
    /// The id of this block
    pub id: BlockId,

    /// The id of the next block. None if the value is -1 (the end of the block chain)
    pub next_block: Option<BlockId>,

    /// Actual data of this block
    pub data: BlockData
}

/// A model that stores block data, usually chained together in [`Blocks`] wrapped around
/// [`BlockEntry`]
#[derive(Debug, Clone, PartialEq)]
pub struct BlockData {
    /// The first substack / nest of this block. None if this block doesn't have a substack / nest
    pub sub_stack1: Option<Blocks>,

    /// The second substack / nest of this block. None if this block doesn't have a second substack / nest
    pub sub_stack2: Option<Blocks>,

    /// The color of this block
    pub color: Color,

    /// The opcode of this block
    pub opcode: String,

    /// The spec of this block
    pub content: block_content::BlockContent,

    /// The type of this block
    pub block_type: BlockType,

    /// The type name of this block (the usage is currently unknown)
    pub type_name: String,
}

impl BlockData {
    /// Constructs a regular block
    pub fn new_regular(opcode: String, category: BlockCategory, content: block_content::BlockContent) -> Self {
        Self {
            sub_stack1: None, sub_stack2: None,
            color: category.into(),
            opcode,
            content,
            block_type: BlockType::Regular,
            type_name: "".to_string()
        }
    }

    pub fn new_argument(
        opcode: String,
        category: BlockCategory,
        content: block_content::BlockContent,
        return_type: ArgumentBlockReturnType
    ) -> Self {
        Self {
            sub_stack1: None,
            sub_stack2: None,
            color: category.into(),
            opcode,
            content,
            block_type: BlockType::Argument(return_type),
            type_name: "".to_string()
        }
    }

    pub fn new_one_nest(
        opcode: String,
        category: BlockCategory,
        content: block_content::BlockContent,
        nested_blocks: Blocks
    ) -> Self {
        Self {
            sub_stack1: Some(nested_blocks),
            sub_stack2: None,
            color: category.into(),
            opcode,
            content,
            block_type: BlockType::Control(BlockControl::OneNest),
            type_name: "".to_string()
        }
    }

    pub fn new_two_nests(
        opcode: String,
        category: BlockCategory,
        content: block_content::BlockContent,
        first_nest: Blocks,
        second_nest: Blocks,
    ) -> Self {
        Self {
            sub_stack1: Some(first_nest),
            sub_stack2: Some(second_nest),
            color: category.into(),
            opcode,
            content,
            block_type: BlockType::Control(BlockControl::TwoNest),
            type_name: "".to_string()
        }
    }

    pub fn new_ending_block(
        opcode: String,
        category: BlockCategory,
        content: block_content::BlockContent
    ) -> Self {
        Self {
            sub_stack1: None,
            sub_stack2: None,
            color: category.into(),
            opcode,
            content,
            block_type: BlockType::Control(BlockControl::EndingBlock),
            type_name: "".to_string()
        }
    }

    /// Retrieves what category this block is from. Will return an error if the block color doesn't
    /// match to any block category
    pub fn category(&self) -> Result<BlockCategory, UnknownColor> {
        BlockCategory::try_from(self.color)
    }
}

// all of "block types" can be seen here
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
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum BlockControl {
    OneNest, // if block
    TwoNest, // ifElse block
    EndingBlock // finish/break block
}

impl FromStr for BlockType {
    type Err = InvalidBlockType;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
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
            (   _,    _,    _) => Err(UnknownColor { color: value })?
        })
    }
}

impl Into<Color> for BlockCategory {
    fn into(self) -> Color {
        Color::from(match self {
            BlockCategory::Variable => 0xEE7D16,
            BlockCategory::List => 0xCC5B22,
            BlockCategory::Control => 0xE1A92A,
            BlockCategory::Operator => 0x5CB722,
            BlockCategory::Math => 0x23B9A9,
            BlockCategory::File => 0xA1887F,
            BlockCategory::ViewFunc => 0x4A6CD4,
            BlockCategory::ComponentFunc => 0xFCA5E2,
            BlockCategory::MoreBlock => 0x8A55D7
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
        pub fn new_args<F: FnMut(BlockId) -> Result<BlockEntry, BlockConversionError>>(
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
        pub fn parse_from<F: FnMut(BlockId) -> Result<BlockEntry, BlockConversionError>>(
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
        Block(BlockEntry)
    }

    impl FieldValue {
        pub fn parse_from<F: FnMut(BlockId) -> Result<BlockEntry, BlockConversionError>>(
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