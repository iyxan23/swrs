use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::color::Color;
use crate::error::{SWRSError, SWRSResult};
use crate::parser::Parsable;

#[derive(Debug, Eq, PartialEq)]
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

            if line.ends_with("java_var") {
                // variable pool
                // read the screen name
                let screen_name = (&line[1..line.len() - 9]).to_string(); // 8 (length of "java_var") + 1 (the dot)

                // parse variables
                let variable_pool = variable::VariablePool::parse_iter(&mut lines)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst parsing variable pool of {}: {}", screen_name, e
                    )))?;

                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .variables = variable_pool;

            } else if line.ends_with("java_list") {
                // list variable pool
                // get the screen name
                let screen_name = (&line[1..9]).to_string(); // 9 -> length of "java_list"

                // then parse it
                let list_variable_pool = list_variable::ListVariablePool::parse_iter(&mut lines)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst parsing list variable pool of {}: {}",
                        screen_name, e
                    )))?;

                // then put it on the screens list with the screen_name above
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name))
                    .list_variables = list_variable_pool;

            } else if line.ends_with("java_components") {
                // component pool
                // get screen name
                let screen_name = (&line[1..line.len() - 16]).to_string(); // 15 (length of "java_components") + 1 (the dot)

                // then parse it
                let component_pool = component::ComponentPool::parse_iter(&mut lines)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst parsing component pool of {}: {}",
                        screen_name, e
                    )))?;

                // then put it on the screens list with the screen_name above
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name))
                    .components = component_pool;

            } else if line.ends_with("java_events") {
                // event pool
                // get the screen name from the header
                let screen_name = (&line[1..line.len() - 12]).to_string(); // 11 (length of "java_events") + 1 (the dot)

                // then parse it
                let event_pool = event::EventPool::parse_iter(&mut lines)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst parsing event pool of {}: {}",
                        screen_name, e
                    )))?;

                // then put it on the screen it belongs to
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name))
                    .events = event_pool;

            } else if line.ends_with("java_func") {
                // moreblocks pool
                let screen_name = (&line[1..line.len() - 10]).to_string(); // 9 (length of "java_func") + 1 (the dot)

                // then parse it
                let more_block_pool = more_block::MoreBlockPool::parse_iter(&mut lines)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst parsing moreblock pool of {}: {}",
                        screen_name, e
                    )))?;

                // then put it on the screen i guess
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name))
                    .more_blocks = more_block_pool;

            } else {
                // some kind of event that will contain blocks
                // parse the header
                let header = BlockContainerHeader::parse(line)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst reading a blocks container header at line {}: {}",
                        line_counter, e
                    )))?;

                // parse the blocks
                let blocks = BlockContainer::parse_iter(&mut lines)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst parsing blocks for the screen {} on event name {}: {}",
                        header.screen_name, header.container_name, e
                    )))?;

                // then add this to the block container pool
                screens
                    .entry(header.screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(header.screen_name.to_owned()))
                    .block_containers
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

#[derive(Debug, Eq, PartialEq)]
pub struct ScreenLogic {
    pub name: String,
    pub block_containers: HashMap<BlockContainerHeader, BlockContainer>,
    pub variables: variable::VariablePool,
    pub list_variables: list_variable::ListVariablePool,
    pub components: component::ComponentPool,
    pub events: event::EventPool,
    pub more_blocks: more_block::MoreBlockPool,
}

impl ScreenLogic {
    pub fn new_empty(name: String) -> ScreenLogic {
        ScreenLogic {
            name,
            block_containers: Default::default(),
            variables: Default::default(),
            list_variables: Default::default(),
            components: Default::default(),
            events: Default::default(),
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

    impl VariablePool {
        /// Parses a variable pool from an iterator of newline string
        pub fn parse_iter<'a>(newline_iter: &mut impl Iterator<Item=&'a str>) -> SWRSResult<Self> {
            let mut result_map = HashMap::new();

            newline_iter
                .by_ref()
                .take_while(|i|*i != "")
                .map(Variable::parse)
                .collect::<SWRSResult<Vec<Variable>>>()?
                .drain(..)
                .for_each(|variable| {
                    result_map.insert(variable.name.to_owned(), variable);
                });

            Ok(VariablePool(result_map))
        }
    }

    impl Parsable for VariablePool {
        /// Parses a variable pool, do not include the header in the input
        fn parse(s: &str) -> SWRSResult<VariablePool> {
            VariablePool::parse_iter(&mut s.split("\n"))
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
            let (var_type, var_name) =
                s.split_once(":")
                    .ok_or_else(||SWRSError::ParseError(format!(
                        "Couldn't get the variable type / name"
                    )))?;

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

    #[derive(Debug, Copy, Clone, Eq, PartialEq)]
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

pub mod list_variable {
    use std::collections::HashMap;
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::Parsable;

    /// Represents a list variable pool
    ///
    /// `0: HashMap<String, ListVariable>` is a map of variable name -> [`ListVariable`]
    #[derive(Debug, Eq, PartialEq)]
    pub struct ListVariablePool(pub HashMap<String, ListVariable>);

    impl ListVariablePool {
        /// Parses an iterator of newlines (should be taken from `.split("\n")`) into a [`ListVariablePool`]
        pub fn parse_iter<'a>(newline_iter: &mut impl Iterator<Item=&'a str>) -> SWRSResult<Self> {
            newline_iter
                .by_ref()
                .take_while(|i|*i != "")
                .map(ListVariable::parse)
                .try_fold(ListVariablePool(HashMap::new()), |mut acc, i| {
                    let i = i
                        .map_err(|e|SWRSError::ParseError(format!(
                            "Failed to parse a list variable item at line {}: {}", acc.0.len(), e
                        )))?;

                    acc.0.insert(i.name.to_owned(), i);

                    Ok(acc)
                })
        }
    }

    impl Parsable for ListVariablePool {
        fn parse(s: &str) -> SWRSResult<Self> {
            ListVariablePool::parse_iter(&mut s.split("\n"))
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            todo!()
        }
    }

    impl Default for ListVariablePool {
        fn default() -> Self {
            ListVariablePool(HashMap::new())
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct ListVariable {
        pub name: String,
        pub r#type: super::variable::VariableType
    }

    impl Parsable for ListVariable {
        fn parse(s: &str) -> SWRSResult<Self> {
            let (lvar_type, name) = {
                Ok(s.split_once(":")
                    .ok_or_else(||SWRSError::ParseError("Failed to split list variable by \":\"".to_string()))?)
            }?;

            Ok(ListVariable {
                name: name.to_string(),
                r#type: <super::variable::VariableType as TryFrom<u8>>
                    ::try_from(
                    lvar_type
                        .parse()
                        .map_err(|_|SWRSError::ParseError(
                            "Couldn't turn the list's variable type into integer".to_string()
                        ))?
                    )
                    .map_err(|e|
                        SWRSError::ParseError(e.to_string())
                    )?,
            })
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            Ok(format!("{}:{}", self.r#type as u8, self.name))
        }
    }
}

pub mod component {
    use serde::{Serialize, Deserialize};
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::Parsable;

    #[derive(Debug, Eq, PartialEq)]
    pub struct ComponentPool(pub Vec<Component>);

    impl ComponentPool {
        pub fn parse_iter<'a>(newlines_iter: impl Iterator<Item=&'a str>) -> SWRSResult<Self> {
            Ok(ComponentPool(
                newlines_iter
                    .take_while(|i|*i != "")
                    .map(Component::parse)
                    .collect::<SWRSResult<Vec<Component>>>()?
            ))
        }
    }

    impl Parsable for ComponentPool {
        fn parse(s: &str) -> SWRSResult<ComponentPool> {
            ComponentPool::parse_iter(s.split("\n"))
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

    impl MoreBlockPool {
        /// Parses a moreblock pool using an iterator of newlines
        pub fn parse_iter<'a>(newline_iter: &mut impl Iterator<Item=&'a str>) -> SWRSResult<Self> {
            let mut more_blocks = newline_iter
                .by_ref()
                .take_while(|i|*i != "")
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
    }

    impl Parsable for MoreBlockPool {
        /// Parses a moreblock pool (list of moreblock declarations), make sure to not include its
        /// header into the input
        fn parse(s: &str) -> SWRSResult<MoreBlockPool> {
            MoreBlockPool::parse_iter(&mut s.split("\n"))
        }

        /// Reconstructs a moreblock pool to its string form
        fn reconstruct(&self) -> SWRSResult<String> {
            Ok(self.0
                .values()
                .map(MoreBlock::reconstruct)
                .fold(SWRSResult::Ok(String::new()), |ac, i| {
                    Ok(format!("{}\n{}", ac?, i?))
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

pub mod event {
    use serde::{Serialize, Deserialize};
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::Parsable;

    #[derive(Debug, Eq, PartialEq)]
    pub struct EventPool(pub Vec<Event>);

    impl EventPool {
        /// Parses an event pool from a newline iterator
        pub fn parse_iter<'a>(newline_iter: &mut impl Iterator<Item=&'a str>) -> SWRSResult<Self> {
            Ok(EventPool(
                newline_iter
                    .by_ref()
                    .take_while(|i|*i != "")
                    .map(Event::parse)
                    .collect::<SWRSResult<Vec<Event>>>()?
            ))
        }
    }

    impl Parsable for EventPool {
        fn parse(decrypted_content: &str) -> SWRSResult<Self> {
            EventPool::parse_iter(&mut decrypted_content.split("\n"))
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            self.0
                .iter()
                .try_fold(String::new(), |acc, event| {
                    Ok(format!("{}\n{}", acc, event.reconstruct()?))
                })
        }
    }

    impl Default for EventPool {
        fn default() -> Self {
            EventPool(vec![])
        }
    }

    #[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Event {
        pub event_name: String,
        pub event_type: u8,
        pub target_id: String,
        pub target_type: u8,
    }

    impl Parsable for Event {
        fn parse(decrypted_content: &str) -> SWRSResult<Self> {
            serde_json::from_str(decrypted_content)
                .map_err(|e|SWRSError::ParseError(format!(
                    "Failed to parse event: {}", e
                )))
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            serde_json::to_string(self)
                .map_err(|e|SWRSError::ParseError(format!(
                    "Failed to reconstruct event: {}", e
                )))
        }
    }
}

// List variable
/// Represents the header of a blocks container
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct BlockContainerHeader {
    pub screen_name: String,
    pub container_name: String,
}

impl Parsable for BlockContainerHeader {
    /// Parses the header of a block container
    fn parse(s: &str) -> SWRSResult<BlockContainerHeader> {
        if !s.starts_with("@") {
            return Err(SWRSError::ParseError("Header does not start with @".to_string()))
        }

        let mut parts = s.split(".java_");
        let screen_name = parts.next()
            .ok_or_else(||SWRSError::ParseError(
                "Cannot get the screen name of a block container header".to_string()
            ))?[1..].to_string();

        let container_name = parts.next()
            .ok_or_else(|| SWRSError::ParseError(
                "Cannot get the container name of a block container header".to_string()
            ))?.to_string();

        Ok(
            BlockContainerHeader {
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
pub struct BlockContainer(Vec<Block>);

impl BlockContainer {
    /// Parses a block container from an iterator of newlines
    fn parse_iter<'a>(newline_iter: &mut impl Iterator<Item=&'a str>) -> SWRSResult<Self> {
        Ok(BlockContainer(newline_iter
            .by_ref()
            .take_while(|i|*i != "")
            .map(Block::parse)
            .collect::<SWRSResult<Vec<Block>>>()?
        ))
    }
}

impl Parsable for BlockContainer {
    /// This just parses a list of blocks, do not include the header
    fn parse(s: &str) -> SWRSResult<BlockContainer> {
        BlockContainer::parse_iter(&mut s.split("\n"))
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