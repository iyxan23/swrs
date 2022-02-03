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
        if let Some(to_be_removed_block) = self.blocks.get(&id) {
            // check if we're the start of the block chain
            if self.starting_id == id {
                // we need to set the starting id to the next block
                if let Some(next_block) = to_be_removed_block.next_block {
                    self.starting_id = Some(next_block);
                } else {
                    // else then the block chain is empty, there is no block after us and we're the
                    // start of this block chain
                    self.starting_id = None;
                }

                return true;
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
                if block.next_block == id {
                    // nice we got it!
                    break;
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
            if let Some(next_block) = to_be_removed_block.next_block {
                block_before.next_block = Some(next_block);
            }

            // we are done!

            true
        } else {
            false
        }
    }
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
                        )))?.set_args(p_block.parameters.to_owned())?,
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

impl TryInto<BlockContainer> for Blocks {
    type Error = SWRSError;

    fn try_into(self) -> Result<BlockContainer, Self::Error> {
        let mut block_container = BlockContainer(vec![]);

        fn flatten(blocks: Blocks, result: &mut BlockContainer) {
            for block in blocks {
                // flatten its children first
                let mut ss_1_container = BlockContainer(vec![]);
                let mut ss_1_id = -1i32;

                if let Some(blocks) = block.sub_stack1 {
                    ss_1_id = blocks.starting_id.map(|i|i.0 as i32).unwrap_or(-1);
                    flatten(blocks, &mut ss_1_container);
                }

                let mut ss_2_container = BlockContainer(vec![]);
                let mut ss_2_id = -1i32;

                if let Some(blocks) = block.sub_stack2 {
                    ss_2_id = blocks.starting_id.map(|i|i.0 as i32).unwrap_or(-1);
                    flatten(blocks, &mut ss_2_container);
                }

                // do the actual block conversion
                result.0.push(ParserBlock {
                    color: block.color,
                    id: block.id.0.to_string(),
                    next_block: block.next_block.map(|n|n.0 as i32).unwrap_or(-1),
                    op_code: block.op_code,
                    spec: block.spec.to_string(),
                    parameters: block.spec.items.into_iter().map(|i|i.to_string()).collect(),
                    sub_stack1: ss_1_id,
                    sub_stack2: ss_2_id,
                    r#type: block.ret_type,
                    type_name: block.type_name
                });

                // then push its children
                result.0.append(&mut ss_1_container.0);
                result.0.append(&mut ss_2_container.0);
            }
        }

        flatten(self, &mut block_container);

        Ok(block_container)
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
    use ritelinked::LinkedHashMap;
    use crate::{SWRSError, SWRSResult};

    /// A model that represents the spec of a block
    #[derive(Debug, Clone, PartialEq)]
    pub struct Spec {
        pub items: Vec<SpecItem>,
        args: Option<LinkedHashMap<SpecItem, String>>,
    }

    impl Spec {
        /// Retrieves all the fields of this spec
        pub fn get_all_fields(&self) -> Vec<&SpecItem> {
            self.items
                .iter()
                .filter_map(|i| if let SpecItem::Field { .. } = i { Some(i) } else { None })
                .collect()
        }

        /// Retrieves a specific index on all of the fields of this spec
        pub fn get_field(&self, index: usize) -> Option<&SpecItem> {
            self.get_all_fields()
                .get(index)
                .map(|i| *i)
        }

        /// Sets the arguments for this [`Spec`]
        pub fn set_args(mut self, args: Vec<String>) -> SWRSResult<Self> {
            let params = self.get_all_fields();

            // do a check if it has the same length as our spec total arguments
            if params.len() != args.len() {
                Err(SWRSError::ParseError(format!(
                    "The provided list of arguments does not have the same length ({}) as the parameters in the spec ({})",
                    args.len(), params.len()
                )))?
            }

            // cool let's zip them together
            self.args = Some(params.into_iter().cloned().zip(args).collect());

            Ok(self)
        }

        /// Retrieves the arguments of the block attached to this spec
        pub fn get_args(&self) -> &Option<LinkedHashMap<SpecItem, String>> { &self.args }
    }

    impl FromStr for Spec {
        type Err = SWRSError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(Spec {
                items: s.split(" ")
                        .map(SpecItem::from_str)
                        .collect::<SWRSResult<Vec<SpecItem>>>()?,
                args: None,
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

    #[derive(Debug, Clone, Hash, Eq, PartialEq)]
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

    #[cfg(test)]
    mod test {
        use super::*;
        use super::SpecItem::*;
        use super::SpecFieldType::*;

        #[test]
        fn spec_parse() {
            let spec = Spec::from_str("custom_block %s.name number %d true %b %m.thing")
                .unwrap();

            let expected = Spec {
                items: vec![
                    Text("custom_block".to_string()),
                    Field { field_type: String, name: Some("name".to_string()) },
                    Text("number".to_string()),
                    Field { field_type: Number, name: None },
                    Text("true".to_string()),
                    Field { field_type: Boolean, name: None },
                    Field { field_type: Menu, name: Some("thing".to_string()) }
                ],
                args: None
            };

            assert_eq!(spec, expected);
        }

        #[test]
        fn spec_reconstruction() {
            let raw_spec = "hello world %s.something %d.num %m.oke %b yeet".to_string();

            assert_eq!(
                Spec::from_str(raw_spec.as_str()).unwrap().to_string(),
                raw_spec
            );
        }
    }
}