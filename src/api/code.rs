use std::collections::HashMap;
use std::ops::Index;
use ritelinked::LinkedHashMap;
use crate::color::Color;
use crate::parser::logic::Block as RawBlock;

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
    pub spec: String,

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