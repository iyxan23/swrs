use std::collections::btree_map::Range;
use std::collections::{Bound, BTreeMap};
use std::num::ParseIntError;
use std::ops::RangeBounds;
use std::str::FromStr;
use thiserror::Error;
use crate::color::Color;
use crate::parser::logic::BlockContainer;

/// The default start id of a [`Blocks`]
const DEFAULT_BLOCK_ID_START: BlockId = BlockId(10);

#[derive(Debug, Clone, PartialEq)]
pub struct Blocks {
    blocks: BTreeMap<BlockId, BlockEntry>,
    starting_id: BlockId,
}

impl Blocks {
    /// Creates a new [`Blocks`] instance
    pub fn new() -> Self {
        Self {
            blocks: BTreeMap::new(),
            starting_id: DEFAULT_BLOCK_ID_START
        }
    }

    /// Creates a new [`Blocks`] instance that will start with the specified starting id
    pub fn new_w_start(starting_id: BlockId) -> Self {
        Self { blocks: BTreeMap::new(), starting_id }
    }

    /// Compacts blocks after the specified id.
    ///
    /// ### Examples
    /// ```txt
    /// a = [10, 11, 14, 16, 30, 31, 32, 35]
    /// a.compact(after: None)
    /// a -> [10, 11, 12, 13, 14, 15, 16, 17]
    /// ```
    /// ```txt
    /// a = [10, 11, 14, 16, 30, 31, 32, 35]
    /// a.compact(after: Some(16))
    ///
    /// a -> [10, 11, 14, 16, 30, 31, 32, 35]
    ///                       vv  vv  vv  vv
    /// a -> [10, 11, 14, 16, 17, 18, 19, 20]
    /// ```
    pub fn compact_blocks(&mut self, after: Option<BlockId>) {
        todo!()
    }

    /// Inserts another [`Blocks`] chain after the specified block id. The remained blocks after the
    /// inserted [`Blocks`]'s IDs will get rewritten and compacted using [`Blocks::compact_blocks`],
    /// and so does the newly inserted blocks.
    ///
    /// Inserting blocks inside a block's substack will change the block's substack position as well
    ///
    /// ### Examples
    /// ```txt
    /// Insertion to the back
    ///
    /// a = [10, 11, 13]
    /// b = [10, 12, 15]
    ///
    /// a.insert(b, after: 13)
    ///
    /// b ->             [10, 12, 15]
    /// a -> [10, 11, 13] vv  vv  vv
    /// a -> [10, 11, 13, 14, 15, 16]
    /// ```
    /// ```txt
    /// Insertion in the middle of blocks
    ///
    /// a = [10, 12, 13, 15, 16]
    /// b = [10, 12, 15]
    ///
    /// a.insert(b, after: 12)
    ///
    /// b ->         [10, 12, 15]
    ///              v
    /// a -> [10, 12, 13, 15, 16]
    /// a -> [10, 12,             13, 15, 16]
    /// b ->         [10, 12, 15] vv  vv  vv
    ///               vv  vv  vv  vv  vv  vv
    /// a -> [10, 12, 13, 14, 15, 16, 17, 18]
    /// ```
    pub fn insert_blocks(
        &mut self,
        blocks: Blocks,
        after: BlockId
    ) -> Option<BlockId> {
        todo!()
    }

    /// Shifts blocks after the specified block into a specified number amount of time (exclusive).
    /// Does not compact the blocks that are shifted.
    ///
    /// Returns the first shifted block.
    ///
    /// ### Panic
    /// Panics when there is no space for a backwards/negative shift.
    ///
    /// Example:
    /// ```txt
    /// a -> [10, 11, 13, 14]
    /// a.shift_blocks(after: 11, -2)
    ///
    /// a -> [10, 11, 13, 14]
    ///               -2  -2
    /// a -> [10, 11, 11, 12]
    ///           ^^  ^^
    ///           [!] collision
    /// ```
    ///
    /// ### Examples
    /// ```txt
    /// a -> [10, 11, 12, 15, 17]
    /// a.shift_blocks(after: 11, shift: 5)
    ///
    /// a -> [10, 11, 12, 15, 17]
    ///               +5  +5  +5
    /// a -> [10, 11, 17, 20, 22]
    ///
    /// returns 17 ---^^
    /// ```
    /// ```txt
    /// a -> [10, 11, 12, 20, 26]
    /// a.shift_blocks(after: 12, shift: -6)
    ///
    /// a -> [10, 11, 12, 20, 26]
    ///                   -6  -6
    /// a -> [10, 11, 17, 14, 20]
    ///
    /// returns 14 -------^^
    /// ```
    pub fn shift_blocks(&mut self, after: BlockId, shift: i32) -> Option<BlockId> {
        // todo: check if we'll be surpassing the starting_id
        if shift.is_negative() {
            // check if there is a space to the back
            let block_before = self.blocks.range(..=after).rev().next();
            if let Some((block_before_id, _)) = block_before {
                if *block_before_id >= BlockId(after.0 + shift) {
                    panic!("cannot shift {} to the back because there is a block in the way (id {block_before_id})", shift * 1)
                }
            }
        }

        // take the blocks after the given block id

        let blocks = self.blocks.range(after..)
            .map(|(id, _)| id).collect::<Vec<BlockId>>().into_iter()
            .map(|id| self.blocks.remove(&id).unwrap())
            .map(|mut block| {
                block.id.0 += shift;
                block
            })
            .collect::<Vec<BlockEntry>>();

        // re-add them with their new shifted ids

        let mut first_id = None;
        for block in blocks {
            if let None = first_id { first_id = Some(block.id); }

            self.blocks.insert(block.id, block);
        }

        first_id
    }

    /// Inserts a block after the block with the specified id.
    ///
    /// ### Behavior
    /// If there is a block with the id of the given `after` parameter, the new ID will be after + 1
    ///
    /// Otherwise it will find if there is any block before the specified after parameter, and use
    /// that block's id plus 1. If there is no block, it will use the Blocks' starting id.
    ///
    /// After all that, it will check if the determined ID is already occupied by another block,
    /// if it's occupied it will shift the blocks by one to make for one space for the inserted
    /// block
    ///
    /// ### Examples
    /// ```text
    /// Regular insertion
    /// blocks -> [10, 11, 14, 17]
    /// blocks.insert_block(block, after: 14)
    ///                       v
    /// blocks -> [10, 11, 14, 17]
    /// blocks -> [10, 11, 14,   , 17]
    /// blocks -> [10, 11, 14, 15, 17]
    /// ```
    /// ```text
    /// Insertion with shifting
    /// blocks = [10, 11, 12, 14]
    /// blocks.insert_block(block, after: 11)
    ///                    vv
    /// blocks -> [10, 11, 12, 14]
    /// blocks -> [10, 11,   , 13, 14] <- shifts
    /// blocks -> [10, 11, 12, 13, 14]
    /// ```
    /// ```text
    /// Insertion after a non-existent block
    /// blocks -> [10, 11, 13, 18]
    /// blocks.insert_block(block, after: 15)
    ///                       v
    /// blocks -> [10, 11, 13, 18]
    /// blocks -> [10, 11, 13,   , 18]
    /// blocks -> [10, 11, 13, 16, 18] <- 16 because 15 + 1
    /// ```
    /// ```text
    /// Insertion on an empty blocks
    /// blocks -> []
    /// blocks.insert_block(block, after: 15)
    ///
    /// blocks -> [10] <- no matter what `after` is, its id will always be the starting id
    /// ```
    pub fn insert_block(
        &mut self,
        block: Block,
        after: BlockId
    ) -> BlockId {
        let id = if let Some(block_before) = self.blocks.get(&after) {
            block_before.id.increment()
        } else {
            // this block doesn't exist
            // if there is a block behind somewhere, we could do this operation and just set this
            //   block's id to be after.increment()
            // if not, we set the id to be the starting id
            if let Some(_) = self.blocks.range(..after).rev().next() {
                after.increment()
            } else {
                self.starting_id
            }
        };

        // check if that id is already occupied
        if let Some(_) = self.blocks.get(&id) {
            // shift the blocks after it by one
            self.shift_blocks(after, 1);
        }

        self.blocks.insert(id, BlockEntry {
            id, block, sub_stack1: None, sub_stack2: None
        });

        id
    }

    /// Pushes a block on the back of the chain. The block id is retrieved through the addition of
    /// one on the last block.
    pub fn push_back(
        &mut self,
        block: Block
    ) -> BlockId {
        self.push_back_w_sub_stack(block, (None, None))
    }

    /// Pushes a block on the back of the chain with its substacks. The block id is retrieved
    /// through the addition of one on the last block.
    ///
    /// ### Panics
    /// Panics when sub_stacks is given a tuple of (None, Some(_))
    pub fn push_back_w_sub_stack(
        &mut self,
        block: Block,
        sub_stacks: (Option<Blocks>, Option<Blocks>)
    ) -> BlockId {
        // todo: utilise BTreeMap::last_key_value() once the feature `map_first_last` (62924) got
        //       stabilised

        // to kind of prevent undefined behavior
        if sub_stacks.0.is_none() && sub_stacks.1.is_some() {
            panic!("the second substack cannot be some if the first substack is none");
        }

        let id = if let Some((last_block_id, _)) = self.blocks.iter().max() {
            last_block_id.increment()
        } else {
            // oh this blocks is empty
            self.starting_id
        };

        let entry = BlockEntry { id, block, sub_stack1: None, sub_stack2: None };
        self.blocks.insert(id, entry);

        // temporarily get a mutable reference to change the substack ids once we've inserted them
        // into the chain
        let entry = self.blocks.get_mut(&id).unwrap();

        // add our substacks
        let ss1_id = sub_stacks.0.map(|sub_stack1| {
            self.insert_blocks(sub_stack1, id).unwrap()
        });

        let ss2_id = sub_stacks.1.map(|sub_stack2| {
            self.insert_blocks(sub_stack2, ss1_id.unwrap()).unwrap()
        });

        // then change the ss1/ss2 ids on the block entry we've just inserted
        entry.sub_stack1 = ss1_id;
        entry.sub_stack2 = ss2_id;

        id
    }

    /// Retrieves a block with the given id
    pub fn get(&self, id: BlockId) -> Option<&BlockEntry> {
        self.blocks.get(&id)
    }

    /// Retrieves a mutable borrow of a block with the given id
    pub fn get_mut(&mut self, id: BlockId) -> Option<&mut BlockEntry> {
        self.blocks.get_mut(&id)
    }

    /// Removes the block with the given id and its substacks.
    pub fn remove(&mut self, id: BlockId) -> Option<(Block, (Option<Blocks>, Option<Blocks>))> {
        let block = self.blocks.remove(&id)?;

        Some((block.block, (
            block.sub_stack1.map(|sub_stack1| self.remove_ranged(id..sub_stack1)).flatten(),
            block.sub_stack1.map(|sub_stack1| self.remove_ranged(id..sub_stack1)).flatten(),
        )))
    }

    /// Removes a range of blocks
    pub fn remove_ranged<R>(&mut self, range: R) -> Blocks
    where R: RangeBounds<BlockId> {
        // used to skip substacks
        let mut skip_until = None;
        let mut removed_blocks = Blocks::new();

        for (id, entry) in self.blocks.range(range) {
            if let Some(skip_until_) = skip_until {
                if id != skip_until_ { continue; }
                skip_until = None;
                continue;
            }

            if let Some(sub_stack1) = entry.sub_stack1 { skip_until = Some(sub_stack1); }
            if let Some(sub_stack2) = entry.sub_stack2 { skip_until = Some(sub_stack2); }

            let (removed_block, substacks) = self.remove(*id).unwrap();
            removed_blocks.push_back_w_sub_stack(removed_block, substacks);
        }

        removed_blocks
    }

    /// Removes an amount of blocks after a block with the given id
    pub fn remove_amount(&mut self, from: BlockId, amount: u32) -> Option<Blocks> {
        todo!()
    }

    /// Retrieves the substack1 of a block with the given id
    pub fn get_sub_stack1(&self, id: BlockId) -> Option<Range<BlockId, BlockEntry>> {
        let block = self.blocks.get(&id)?;
        Some(self.blocks.range(block.id..=block.sub_stack1?))
    }

    /// Retrieves the substack2 of a block with the given id
    pub fn get_sub_stack2(&self, id: BlockId) -> Option<Range<BlockId, BlockEntry>> {
        let block = self.blocks.get(&id)?;
        Some(self.blocks.range(block.sub_stack1..=block.sub_stack2?))
    }

    /// Removes the substack of the specified block id and mented by Rust's built-in range types, produced by range syntax like .., a.., ..b, ..=c, d..e, or f..=g.replaces it with the provided [`Blocks`].
    /// Will compact the blocks after the inserted substack with [`Blocks::compact_blocks`]
    pub fn replace_sub_stack1(&mut self, id: BlockId, blocks: Blocks) -> Option<BlockId> {
        todo!()
    }

    /// Removes the sub stack of the specified block id and replaces it with the provided [`Blocks`]
    /// Will compact the blocks after the inserted substack with [`Blocks::compact_blocks`]
    pub fn replace_sub_stack2(&mut self, id: BlockId, blocks: Blocks) -> Option<BlockId> {
        todo!()
    }

    /// Returns how much blocks are there that are stored in this [`Blocks`]
    pub fn length(&self) -> usize {
        self.blocks.len()
    }
}

/// An entry of [`Blocks`]
#[derive(Debug, Clone, PartialEq)]
pub struct BlockEntry {
    id: BlockId,
    pub block: Block,
    sub_stack1: Option<BlockId>,
    sub_stack2: Option<BlockId>,
}

impl BlockEntry {
    pub fn sub_stack1(&self, origin: &Blocks) -> Option<Range<BlockId, BlockEntry>> {
        origin.get_sub_stack1(self.id)
    }

    pub fn sub_stack2(&self, origin: &Blocks) -> Option<Range<BlockId, BlockEntry>> {
        origin.get_sub_stack2(self.id)
    }
}

/// A model that stores information about a block
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub content: block_content::BlockContent,
    pub color: Color,
    pub opcode: String,
    pub block_type: BlockType,
}

impl TryFrom<Range<BlockId, BlockEntry>> for Blocks {
    type Error = ();

    fn try_from(value: Range<BlockId, BlockEntry>) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<BlockContainer> for Blocks {
    type Error = ();

    fn try_from(value: BlockContainer) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryInto<BlockContainer> for Blocks {
    type Error = ();

    fn try_into(self) -> Result<BlockContainer, Self::Error> {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct BlockId(pub u32);

impl BlockId {
    /// Increments [`BlockId`] by one
    pub fn increment(&self) -> Self {
        BlockId(self.0 + 1)
    }

    /// Increments [`BlockId`] by the given amount of number
    pub fn increment_by(&self, num: u32) -> Self {
        BlockId(self.0 + num)
    }
}

impl FromStr for BlockId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BlockId(s.parse::<u32>()?))
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