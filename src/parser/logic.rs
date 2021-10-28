use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::color::Color;
use crate::error::{SWRSError, SWRSResult};
use crate::parser::Parsable;

pub struct Logic {
    pub screens: HashMap<String, ScreenLogic>
}

impl Parsable for Logic {
    fn parse(logic: &str) -> SWRSResult<Logic> {
        let mut lines = logic.split("\n");
        let mut screens = HashMap::<String, ScreenLogic>::new();
        let mut line_counter = 0u32;

        loop {
            let line = lines.next();
            if line.is_none() { break; }
            let line = line.unwrap();

            if !line.starts_with("@") {
                // todo: warning: skipping line {} because it doesn't resemble a header
                break;
            }

            // todo: @MainActivity.java_list, @MainActivity.java_events
            if line.ends_with("java_var") {
                // variable pool
                // read the screen name
                let screen_name = &line[1..8]; // 8 -> length of "java_var"

                // collect all the variables (get all things until an empty line)
                let variables_str = {
                    let mut result = String::new();

                    loop {
                        let line = lines.next();
                        if line.is_none() { break; }
                        let line = line.unwrap();

                        if !line.trim().is_empty() {
                            result.push_str(line);
                            result.push_str("\n");
                        }
                    }

                    result = result.trim().to_string();
                    result
                };

                // parse variables
                let variable_pool = variable::VariablePool::parse(variables_str.as_str())
                    .map_err(|e| SWRSError::ParseError(format!(
                        "Error whilst parsing variable pool: {}", e
                    )))?;

                // then put the variable pool to the screen name
                if !screens.contains_key(screen_name) {
                    screens.insert(
                        screen_name.to_string(),
                        ScreenLogic::new_empty(screen_name.to_string())
                    );
                }

                screens
                    .get_mut(screen_name)
                    .unwrap() // this shouldn't be empty, we've initialized it just in case before
                    .variables = variable_pool;

            } else if line.ends_with("java_func") {
                // moreblocks pool
                let screen_name = (&line[1..9]).to_string(); // 9 -> length of "java_func"

                // collect all stuff until an empty line
                let more_blocks_str = {
                    let mut result = String::new();

                    loop {
                        let line = lines.next();
                        if line.is_none() { break; }
                        let line = line.unwrap();

                        if !line.trim().is_empty() {
                            result.push_str(line);
                            result.push_str("\n");
                        }
                    }

                    result = result.trim().to_string();
                    result
                };

                // then parse it
                let more_block_pool = more_block::MoreBlockPool::parse(more_blocks_str.as_str())
                    .map_err(|e|SWRSError::ParseError(
                        format!("Error whilst parsing moreblock pool of {}: {}", screen_name, e)
                    ))?;

                // then put it on the screen i guess
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .more_blocks = more_block_pool;

            } else {
                // some kind of event
                // parse the header
                let header = BlocksContainerHeader::parse(line)
                    .map_err(|e|
                        SWRSError::ParseError(
                            format!(
                                "Error whilst reading a blocks container header at line {}: {}",
                                line_counter, e
                            )
                        )
                    )?;

                // collect all blocks (read everything until empty line)
                let blocks_str = {
                    let mut result = String::new();

                    loop {
                        let line = lines.next();
                        if line.is_none() { break; }
                        let line = line.unwrap();

                        if !line.trim().is_empty() {
                            result.push_str(line);
                            result.push_str("\n");
                        }
                    }

                    result = result.trim().to_string();
                    result
                };

                // parse the blocks
                let blocks = BlocksContainer::parse(blocks_str.as_str())
                    .map_err(|e|SWRSError::ParseError(
                        format!(
                            "Error whilst parsing blocks for the screen {} on event name {}: {}",
                            header.screen_name, header.container_name, e)
                    ))?;

                // first we check if it has the screen in screens, if not then add it
                if !screens.contains_key(header.screen_name.as_str()) {
                    screens.insert(
                        header.screen_name.to_owned(),
                        ScreenLogic::new_empty(header.screen_name.to_owned())
                    );
                }

                // then put the blocks pool to the screen
                screens
                    .get_mut(header.screen_name.as_str())
                    .unwrap()
                    .events
                    .insert(header, blocks);
            }

            line_counter += 1;
        }

        Ok(Logic { screens })
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        todo!()
    }
}

pub struct ScreenLogic {
    pub name: String,
    pub events: HashMap<BlocksContainerHeader, BlocksContainer>,
    pub variables: variable::VariablePool,
    pub components: component::ComponentPool,
    pub more_blocks: more_block::MoreBlockPool,
}

impl ScreenLogic {
    pub fn new_empty(name: String) -> ScreenLogic {
        ScreenLogic {
            name,
            events: Default::default(),
            variables: Default::default(),
            components: Default::default(),
            more_blocks: Default::default(),
        }
    }
}

pub mod variable {
    use std::collections::HashMap;
    use std::convert::TryFrom;
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::Parsable;

    #[derive(Debug, Eq, PartialEq)]
    pub struct VariablePool(pub HashMap<String, Variable>);

    impl Parsable for VariablePool {
        /// Parses a variable pool, do not include the header in the input
        fn parse(s: &str) -> SWRSResult<VariablePool> {
            let mut result_map = HashMap::new();

            s.split("\n")
                .map(|s| {
                    Variable::parse(s)
                })
                .collect::<SWRSResult<Vec<Variable>>>()?
                .drain(..)
                .for_each(|variable| {
                    result_map.insert(variable.name.to_owned(), variable);
                });

            Ok(VariablePool(result_map))
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            todo!()
        }
    }

    impl Default for VariablePool {
        fn default() -> Self {
            VariablePool(Default::default())
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Variable {
        pub name: String,
        pub r#type: VariableType,
    }

    impl Parsable for Variable {
        fn parse(s: &str) -> SWRSResult<Variable> {
            let (var_type, var_name) = {
                let mut iter = s.split(":");

                Ok((
                    iter.next()
                        .ok_or(
                            SWRSError::ParseError(
                                "The variable type does not exist".to_string()
                            )
                        )?,
                    iter.next()
                        .ok_or(
                            SWRSError::ParseError(
                                "The variable name does not exist".to_string()
                            )
                        )?
                ))
            }?;

            Ok(Variable {
                name: var_name.to_string(),
                r#type: VariableType::try_from(
                    var_type
                        .parse::<u8>()
                        .map_err(|e| SWRSError::ParseError(e.to_string()))?
                )?
            })
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            todo!()
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    #[repr(u8)]
    pub enum VariableType {
        Boolean,
        Integer,
        String,
        HashMap, // <String, Object>
    }

    impl TryFrom<u8> for VariableType {
        type Error = SWRSError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(VariableType::Boolean),
                1 => Ok(VariableType::Integer),
                2 => Ok(VariableType::String),
                3 => Ok(VariableType::HashMap),
                _ => Err(
                    SWRSError::ParseError(
                        "The value given is not mapped to any variable type".to_string()
                    )
                )
            }
        }
    }
}

pub mod component {
    use serde::{Serialize, Deserialize};
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::Parsable;

    #[derive(Debug, Eq, PartialEq)]
    pub struct ComponentPool(pub Vec<Component>);

    impl Parsable for ComponentPool {
        fn parse(s: &str) -> SWRSResult<ComponentPool> {
            Ok(ComponentPool(
                s.split("\n")
                    .map(Component::parse)
                    .collect::<SWRSResult<Vec<Component>>>()?
            ))
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            todo!()
        }
    }

    impl Default for ComponentPool {
        fn default() -> Self {
            ComponentPool(vec![])
        }
    }

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    pub struct Component {
        #[serde(rename = "componentId")]
        pub id: String,

        pub param1: String,
        pub param2: String,
        pub param3: String,

        pub r#type: u8,
    }

    impl Parsable for Component {
        fn parse(s: &str) -> SWRSResult<Component> {
            serde_json::from_str(s)
                .map_err(|e| SWRSError::ParseError(
                    format!("Failed to parse component: {}", e.to_string())
                ))
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            todo!()
        }
    }
}

pub mod more_block {
    use std::collections::HashMap;
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::Parsable;

    #[derive(Debug, Eq, PartialEq)]
    pub struct MoreBlockPool(pub HashMap<String, MoreBlock>);

    impl Parsable for MoreBlockPool {
        /// Parses a moreblock pool (list of moreblock declarations), make sure to not include its
        /// header into the input
        fn parse(s: &str) -> SWRSResult<MoreBlockPool> {
            let mut more_blocks = s.split("\n")
                .map(MoreBlock::parse)
                .collect::<SWRSResult<Vec<MoreBlock>>>()?;

            let mut result = HashMap::<String, MoreBlock>::new();

            // turn the more_blocks vec into a hashmap
            more_blocks
                .drain(..)
                .for_each(|more_block| {
                    result.insert((&more_block.id).to_owned(), more_block);
                });

            Ok(MoreBlockPool(result))
        }

        /// Reconstructs a moreblock pool to its string form
        fn reconstruct(&self) -> SWRSResult<String> {
            Ok(self.0
                .values()
                .map(MoreBlock::reconstruct)
                .fold(SWRSResult::Ok(String::new()), |ac, i| {
                    Ok(format!("{}{}", ac?, i?))
                })?
            )
        }
    }

    impl Default for MoreBlockPool {
        fn default() -> Self {
            MoreBlockPool(HashMap::new())
        }
    }

    /// Represents an item of @ActivityName.java_func
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MoreBlock {
        pub id: String,
        pub spec: String,
    }

    impl Parsable for MoreBlock {
        /// Parses a moreblock item, example:
        /// ```
        /// execute_shell:execute_shell %s.command
        /// ```
        fn parse(s: &str) -> SWRSResult<MoreBlock> {
            let (id, spec) = s.split_once(':')
                .ok_or_else(||SWRSError::ParseError(
                    format!("Failed to parse a moreblock, couldn't split `:`")
                ))?;

            Ok(
                MoreBlock {
                    id: id.to_string(),
                    spec: spec.to_string(),
                }
            )
        }

        /// Reconstructs a moreblock into its original form
        fn reconstruct(&self) -> SWRSResult<String> {
            Ok(format!("{}:{}", self.id, self.spec))
        }
    }
}

/// Represents the header of a blocks container
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct BlocksContainerHeader {
    pub screen_name: String,
    pub container_name: String,
}

impl Parsable for BlocksContainerHeader {
    /// Parses the header of a blocks container
    fn parse(s: &str) -> SWRSResult<BlocksContainerHeader> {
        if !s.starts_with("@") {
            return Err(SWRSError::ParseError("Header does not start with @".to_string()))
        }

        let mut parts = s.split(".java_");
        let screen_name = parts.next()
            .ok_or_else(||SWRSError::ParseError(
                "Cannot get the screen name of a blocks header".to_string()
            ))?[1..].to_string();

        let container_name = parts.next()
            .ok_or_else(|| SWRSError::ParseError(
                "Cannot get the container name of a blocks header".to_string()
            ))?.to_string();

        Ok(
            BlocksContainerHeader {
                screen_name,
                container_name
            }
        )
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        todo!()
    }
}

/// Basically a list of blocks
#[derive(Debug, Eq, PartialEq)]
pub struct BlocksContainer(Vec<Block>);

impl Parsable for BlocksContainer {
    /// This just parses a list of blocks, do not include the header
    fn parse(s: &str) -> SWRSResult<BlocksContainer> {
        Ok(BlocksContainer(
            s.split("\n")
                .map(Block::parse)
                .collect::<SWRSResult<Vec<Block>>>()?
        ))
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        todo!()
    }
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub color: Color,
    pub id: String,
    pub next_block: i32,
    pub op_code: String,
    pub parameters: Vec<String>,
    pub spec: String,
    pub sub_stack1: i32,
    pub sub_stack2: i32,
    pub r#type: String,
    pub type_name: String,
}

impl Parsable for Block {
    fn parse(s: &str) -> SWRSResult<Block> {
        serde_json::from_str(s)
            .map_err(|e|SWRSError::ParseError(format!("Failed to parse the JSON of a block: {}", e)))
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        todo!()
    }
}