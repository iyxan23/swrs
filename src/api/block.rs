use std::str::FromStr;
use crate::{LinkedHashMap, SWRSError, SWRSResult};
use crate::color::Color;
use crate::parser::logic::BlockContainer;

type ParserBlock = crate::parser::logic::Block;

/// A struct that basically stores blocks with its starting id
#[derive(Debug, Clone, PartialEq)]
pub struct Blocks {
    pub starting_id: Option<BlockId>,
    pub blocks: LinkedHashMap<BlockId, Block>
}

// converts a block container into an API struct Blocks
impl TryFrom<BlockContainer> for Blocks {
    type Error = SWRSError;

    fn try_from(value: BlockContainer) -> Result<Self, Self::Error> {
        // first we map them into a LinkedHashMap and associate with each blocks' id
        let blocks = value.0
            .into_iter()
            .map(|block| Ok((BlockId::from_str(block.id.as_str())?, block)))
            .collect::<SWRSResult<LinkedHashMap<BlockId, ParserBlock>>>()?;

        /// a utility function that simply turns an i32 into u32 or if its negative it will return
        /// None
        fn to_u32_else_none(val: i32) -> Option<u32> {
            if val.is_negative() { None } else { Some(val as u32) }
        }

        /// A recursive function that parses blocks given from its id and follows the blocks'
        /// next_block until it stops. This function gets its blocks from the [`blocks`]
        /// variable defined above
        fn parse_to_blocks(starting_id: BlockId, blocks: &LinkedHashMap<BlockId, ParserBlock>) -> SWRSResult<Blocks> {
            let mut result = LinkedHashMap::<BlockId, Block>::new();
            let mut current_id = starting_id;

            loop {
                // first we get the block from the current id
                let p_block = blocks.get(&current_id)
                    .ok_or_else(||SWRSError::ParseError(format!(
                        "Unable to find a block with id {}", starting_id.0
                    )))?;

                // then we convert it to our own Block struct
                let block = Block {
                    id: current_id,
                    next_block: to_u32_else_none(p_block.next_block).map(|val| BlockId(val)),

                    // if the substack1 is negative, then there is no substack1, else we parse the
                    // blocks starting from that substack1
                    sub_stack1: if p_block.sub_stack1.is_negative() { Ok(None) } else {
                        parse_to_blocks(BlockId(p_block.sub_stack1 as u32), blocks)
                            .map(|it| Some(it))
                    }.map_err(|err| SWRSError::ParseError(format!(
                        "Err while parsing substack1 of block id {}: \n{}", current_id.0, err
                    )))?,

                    // if the substack2 is negative, then there is no substack2, else we parse the
                    // blocks starting from that substack2
                    sub_stack2: if p_block.sub_stack2.is_negative() { Ok(None) } else {
                        parse_to_blocks(BlockId(p_block.sub_stack2 as u32), blocks)
                            .map(|it| Some(it))
                    }.map_err(|err| SWRSError::ParseError(format!(
                        "Err while parsing substack2 of block id {}: \n{}", current_id.0, err
                    )))?,

                    color: p_block.color,
                    op_code: p_block.op_code.to_owned(),
                    spec: spec::Spec::from_str(p_block.spec.as_str())
                        .map_err(|err| SWRSError::ParseError(format!(
                            "Unable to parse spec of block with id {} due to: \n{}",
                            current_id.0, err
                        )))?,
                    ret_type: p_block.r#type.to_owned(),
                    type_name: p_block.type_name.to_owned(),
                };

                let next_block = block.next_block.to_owned();

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
            Ok(parse_to_blocks(id.0.to_owned(), &blocks)?)
        } else {
            Ok(Default::default())
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
    type Err = SWRSError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BlockId(
            s.parse::<u32>()
                .map_err(|err|SWRSError::ParseError(format!(
                    "Unable to convert block id {} as an u32, perhaps it's negative? err: {}", s, err
                )))?
        ))
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
    pub spec: spec::Spec,

    /// The return type of this block
    pub ret_type: String,

    /// The type name of this block (the usage is currently unknown)
    pub type_name: String,
}

impl Block {
    /// Retrieves what category this block is from. Will return an error if the block color doesn't
    /// match to any block category
    pub fn category(&self) -> SWRSResult<BlockCategory> {
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
    type Error = SWRSError;

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
            (_, _, _) => Err(SWRSError::ParseError(format!(
                "Color {} does not correlate to any block category", value
            )))?
        })
    }
}

pub mod spec {
    use std::str::FromStr;
    use crate::{SWRSError, SWRSResult};

    /// A model that represents the spec of a block
    #[derive(Debug, Clone, PartialEq)]
    pub struct Spec {
        pub items: Vec<SpecItem>
    }

    impl Spec {
        /// Retrieves all fields / args of this spec
        pub fn get_all_args(&self) -> Vec<&SpecItem> {
            self.items
                .iter()
                .filter_map(|i| if let SpecItem::Field { .. } = i { Some(i) } else { None })
                .collect()
        }

        /// Retrieves a specific index on all of the fields / args of this spec
        pub fn get_arg(&self, index: usize) -> Option<&SpecItem> {
            self.get_all_args()
                .get(index)
                .map(|i| *i)
        }
    }

    impl FromStr for Spec {
        type Err = SWRSError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Spec {
                items: s.split(" ")
                        .map(SpecItem::from_str)
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
    #[derive(Debug, Clone, Copy, Eq, PartialEq)]
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
                "%s" => SpecFieldType::String,
                "%b" => SpecFieldType::Boolean,
                "%d" => SpecFieldType::Number,
                "%m" => SpecFieldType::Menu,
                &_ => Err(SWRSError::ParseError(format!(
                    "Unknown spec field type \"{}\", expected %s, %b, %d, or %m", s
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