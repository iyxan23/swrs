use ritelinked::LinkedHashMap;
use serde::{Serialize, Deserialize};
use crate::color::Color;
use crate::parser::Parsable;
use thiserror::Error;
use crate::parser::logic::component::{ComponentPoolParseError, ComponentPoolReconstructionError};
use crate::parser::logic::event::EventPoolParseError;
use crate::parser::logic::list_variable::ListVariablePoolParseError;
use crate::parser::logic::more_block::MoreBlockPoolParseError;
use crate::parser::logic::variable::VariablePoolParseError;

#[derive(Debug, Eq, PartialEq)]
pub struct Logic {
    /// All the logic of each screens
    ///
    /// The key is the java / logic name
    pub screens: LinkedHashMap<String, ScreenLogic>
}

impl Parsable for Logic {
    type ParseError = LogicParseError;
    type ReconstructionError = LogicReconstructionError;

    fn parse(logic: &str) -> Result<Logic, Self::ParseError> {
        let mut lines = logic.split("\n");
        let mut screens = LinkedHashMap::<String, ScreenLogic>::new();
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
                    .map_err(|err| LogicParseError::VariablePoolParseError {
                        screen_name: screen_name.to_owned(),
                        line: line_counter,
                        source: err
                    })?;

                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .variables = Some(variable_pool);

            } else if line.ends_with("java_list") {
                // list variable pool
                // get the screen name
                let screen_name = (&line[1..line.len() - 10]).to_string(); // 9 -> length of "java_list" + 1 (the dot)

                // then parse it
                let list_variable_pool = list_variable::ListVariablePool::parse_iter(&mut lines)
                    .map_err(|err| LogicParseError::ListVariablePoolParseError {
                        screen_name: screen_name.to_owned(),
                        line: line_counter,
                        source: err,
                    })?;

                // then put it on the screens list with the screen_name above
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .list_variables = Some(list_variable_pool);

            } else if line.ends_with("java_components") {
                // component pool
                // get screen name
                let screen_name = (&line[1..line.len() - 16]).to_string(); // 15 (length of "java_components") + 1 (the dot)

                // then parse it
                let component_pool = component::ComponentPool::parse_iter(&mut lines)
                    .map_err(|err| LogicParseError::ComponentPoolParseError {
                        screen_name: screen_name.to_owned(),
                        line: line_counter,
                        source: err
                    })?;

                // then put it on the screens list with the screen_name above
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .components = Some(component_pool);

            } else if line.ends_with("java_events") {
                // event pool
                // get the screen name from the header
                let screen_name = (&line[1..line.len() - 12]).to_string(); // 11 (length of "java_events") + 1 (the dot)

                // then parse it
                let event_pool = event::EventPool::parse_iter(&mut lines)
                    .map_err(|err| LogicParseError::EventPoolParseError {
                        screen_name: screen_name.to_owned(),
                        line: line_counter,
                        source: err
                    })?;

                // then put it on the screen it belongs to
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .events = Some(event_pool);

            } else if line.ends_with("java_func") {
                // moreblocks pool
                let screen_name = (&line[1..line.len() - 10]).to_string(); // 9 (length of "java_func") + 1 (the dot)

                // then parse it
                let more_block_pool = more_block::MoreBlockPool::parse_iter(&mut lines)
                    .map_err(|err| LogicParseError::MoreBlockPoolParseError {
                        screen_name: screen_name.to_owned(),
                        line: line_counter,
                        source: err,
                    })?;

                // then put it on the screen i guess
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .more_blocks = Some(more_block_pool);

            } else {
                // some kind of event that will contain blocks
                // parse the header
                let BlockContainerHeader { screen_name, container_name } =
                    BlockContainerHeader::parse(line)
                        .map_err(|err| LogicParseError::BlockContainerHeaderParseError {
                            line: line_counter,
                            source: err
                        })?;

                // parse the blocks
                let blocks = BlockContainer::parse_iter(&mut lines)
                    .map_err(|err| LogicParseError::BlockContainerParseError {
                        screen_name: screen_name.to_owned(),
                        container_name: container_name.to_owned(),
                        line: line_counter,
                        source: err
                    })?;

                // then add this to the block container pool
                screens
                    .entry(screen_name.to_owned())
                    .or_insert_with(||ScreenLogic::new_empty(screen_name.to_owned()))
                    .block_containers
                    .insert(container_name.to_owned(), blocks);
            }

            line_counter += 1;
        }

        Ok(Logic { screens })
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        // first we are going to append each screens' containers in different variables
        let mut variable_containers = Vec::<String>::new();
        let mut list_variable_containers = Vec::<String>::new();
        let mut more_block_containers = Vec::<String>::new();
        let mut component_containers = Vec::<String>::new();
        let mut event_containers = Vec::<String>::new();
        let mut block_containers = Vec::<String>::new();

        for screen in self.screens.values() {
            if let Some(variables) = &screen.variables {
                variable_containers
                    .push(format!(
                        "@{}.java_var\n{}", screen.name, variables.reconstruct().unwrap() /* should never fail */
                    ));
            }

            if let Some(list_variable) = &screen.list_variables {
                list_variable_containers
                    .push(format!(
                        "@{}.java_list\n{}", screen.name, list_variable.reconstruct().unwrap() /* should never fail */
                    ))
            }

            if let Some(more_block) = &screen.more_blocks {
                more_block_containers
                    .push(format!(
                        "@{}.java_func\n{}", screen.name, more_block.reconstruct().unwrap() /* should never fail */
                    ));
            }

            if let Some(component) = &screen.components {
                component_containers
                    .push(format!(
                        "@{}.java_components\n{}",
                        screen.name,
                        component.reconstruct()
                            .map_err(|err| LogicReconstructionError::ComponentPoolReconstructionError {
                                screen_name: screen.name.to_owned(),
                                source: err
                            })?
                    ));
            }

            if let Some(event) = &screen.events {
                event_containers
                    .push(format!(
                        "@{}.java_events\n{}", screen.name, event.reconstruct().unwrap() /* should never fail */
                    ));
            }

            let mut result = String::new();

            for (container_id, blocks) in screen.block_containers.iter() {
                result.push('@');
                result.push_str(screen.name.as_str());
                result.push_str(".java_");
                result.push_str(container_id.as_str());
                result.push('\n');
                result.push_str(
                    blocks.reconstruct()
                        .map_err(|err| LogicReconstructionError::BlockContainerReconstructionError {
                            screen_name: screen.name.to_owned(),
                            container_name: container_id.to_owned(),
                            source: err
                        })?
                        .as_str()
                );
                result.push('\n');
            }

            block_containers.push(result);
        }

        // stitch them together and boom!
        Ok(variable_containers
            .into_iter()
            .chain(list_variable_containers)
            .chain(component_containers)
            .chain(event_containers)
            .chain(more_block_containers)
            .chain(block_containers)
            .fold(String::new(), |acc, i| format!("{}\n\n{}", acc, i.trim()))
            .trim()
            .to_string()
        )
    }
}

#[derive(Error, Debug)]
pub enum LogicParseError {
    #[error("error while parsing variable pool of screen {screen_name} at line {line}")]
    VariablePoolParseError {
        screen_name: String,
        line: u32,
        source: VariablePoolParseError
    },

    #[error("error while parsing list variable pool of screen {screen_name} at line {line}")]
    ListVariablePoolParseError {
        screen_name: String,
        line: u32,
        source: ListVariablePoolParseError
    },

    #[error("error while parsing component pool of screen {screen_name} at line {line}")]
    ComponentPoolParseError {
        screen_name: String,
        line: u32,
        source: ComponentPoolParseError
    },

    #[error("error while parsing event pool of screen {screen_name} at line {line}")]
    EventPoolParseError {
        screen_name: String,
        line: u32,
        source: EventPoolParseError
    },

    #[error("error while parsing more block pool of screen {screen_name} at line {line}")]
    MoreBlockPoolParseError {
        screen_name: String,
        line: u32,
        source: MoreBlockPoolParseError
    },

    #[error("error while parsing a block container header at line {line}")]
    BlockContainerHeaderParseError {
        line: u32,
        source: BlockContainerHeaderParseError
    },

    #[error("error while parsing a block container of screen {screen_name} container {container_name} at line {line}")]
    BlockContainerParseError {
        screen_name: String,
        container_name: String,
        line: u32,
        source: BlockContainerParseError
    }
}

#[derive(Error, Debug)]
pub enum LogicReconstructionError {
    #[error("error while reconstructing the component pool of screen {screen_name}")]
    ComponentPoolReconstructionError {
        screen_name: String,
        source: ComponentPoolReconstructionError
    },

    #[error("error while reconstructing the block container of {container_name} of screen {screen_name}")]
    BlockContainerReconstructionError {
        screen_name: String,
        container_name: String,
        source: BlockContainerReconstructionError
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ScreenLogic {
    pub name: String,
    pub block_containers: LinkedHashMap<String, BlockContainer>,
    pub variables: Option<variable::VariablePool>,
    pub list_variables: Option<list_variable::ListVariablePool>,
    pub components: Option<component::ComponentPool>,
    pub events: Option<event::EventPool>,
    pub more_blocks: Option<more_block::MoreBlockPool>,
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
    use std::convert::TryFrom;
    use std::num::ParseIntError;
    use ritelinked::LinkedHashMap;
    use crate::parser::Parsable;
    use thiserror::Error;

    #[derive(Debug, Eq, PartialEq)]
    pub struct VariablePool(pub LinkedHashMap<String, Variable>);

    impl VariablePool {
        /// Parses a variable pool from an iterator of newline string
        pub fn parse_iter<'a>(
            newline_iter: &mut impl Iterator<Item=&'a str>
        ) -> Result<Self, VariablePoolParseError> {

            let mut result_map = LinkedHashMap::new();

            for (count, line) in newline_iter.by_ref().take_while(|i| *i != "").enumerate() {
                let variable = Variable::parse(line)
                    .map_err(|err| VariablePoolParseError {
                        count: count as u32,
                        content: line.to_string(),
                        source: err
                    })?;

                result_map.insert(variable.name.to_owned(), variable);
            }

            Ok(VariablePool(result_map))
        }
    }

    impl Parsable for VariablePool {
        type ParseError = VariablePoolParseError;
        type ReconstructionError = ();

        /// Parses a variable pool, do not include the header in the input
        fn parse(s: &str) -> Result<VariablePool, Self::ParseError> {
            VariablePool::parse_iter(&mut s.split("\n"))
        }

        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            Ok(self.0
                .values()
                .fold(String::new(), |acc, i|
                    format!("{}\n{}", acc, i.reconstruct().unwrap() /* this never fails */ )
                )
                .trim()
                .to_string())
        }
    }

    #[derive(Error, Debug)]
    #[error("error while parsing a variable at count {count}")]
    pub struct VariablePoolParseError {
        pub count: u32,
        pub content: String,

        #[source]
        pub source: VariableParseError
    }

    impl Default for VariablePool {
        fn default() -> Self {
            VariablePool(Default::default())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct Variable {
        pub name: String,
        pub r#type: VariableType,
    }

    impl Parsable for Variable {
        type ParseError = VariableParseError;
        type ReconstructionError = ();

        fn parse(s: &str) -> Result<Variable, Self::ParseError> {
            let (var_type, var_name) =
                s.split_once(":")
                    .ok_or_else(||VariableParseError::MalformedVariable {
                        content: s.to_string()
                    })?;

            Ok(Variable {
                name: var_name.to_string(),
                r#type: VariableType::try_from(
                    var_type
                        .parse::<u8>()
                        .map_err(|err| VariableParseError::MalformedVariableType {
                            content: var_type.to_string(),
                            source: err
                        })?
                ).map_err(VariableParseError::InvalidVariableType)?
            })
        }

        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            Ok(format!("{}:{}", self.r#type as u8, self.name))
        }
    }

    #[derive(Error, Debug)]
    pub enum VariableParseError {
        #[error("malformed variable, couldn't find `:` to separate")]
        MalformedVariable {
            content: String
        },
        #[error("variable type `{content}` is not an int")]
        MalformedVariableType{
            content: String,
            source: ParseIntError
        },
        #[error("variable type {} is not mapped to any value", .0.value)]
        InvalidVariableType(#[source] InvalidVariableTypeError)
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
        type Error = InvalidVariableTypeError;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                0 => Ok(VariableType::Boolean),
                1 => Ok(VariableType::Integer),
                2 => Ok(VariableType::String),
                3 => Ok(VariableType::HashMap),
                _ => Err(InvalidVariableTypeError { value })
            }
        }
    }

    #[derive(Error, Debug)]
    #[error("the value given ({value}) is not mapped to any variable type")]
    pub struct InvalidVariableTypeError {
        pub value: u8
    }
}

pub mod list_variable {
    use std::num::ParseIntError;
    use ritelinked::LinkedHashMap;
    use crate::parser::Parsable;
    use crate::parser::logic::variable::InvalidVariableTypeError;
    use thiserror::Error;

    /// Represents a list variable pool
    ///
    /// `0: HashMap<String, ListVariable>` is a map of variable name -> [`ListVariable`]
    #[derive(Debug, Eq, PartialEq)]
    pub struct ListVariablePool(pub LinkedHashMap<String, ListVariable>);

    impl ListVariablePool {
        /// Parses an iterator of newlines (should be taken from `.split("\n")`) into a [`ListVariablePool`]
        pub fn parse_iter<'a>(
            newline_iter: &mut impl Iterator<Item=&'a str>
        ) -> Result<Self, ListVariablePoolParseError> {

            let mut result = LinkedHashMap::new();

            for (count, line) in
                newline_iter
                    .by_ref()
                    .take_while(|i| *i != "")
                    .enumerate() {

                let list_variable =
                    ListVariable::parse(line)
                        .map_err(|err| ListVariablePoolParseError {
                            count: count as u32,
                            content: line.to_string(),
                            source: err
                        })?;

                result.insert(list_variable.name.to_owned(), list_variable);
            }

            Ok(ListVariablePool(result))
        }
    }

    impl Parsable for ListVariablePool {
        type ParseError = ListVariablePoolParseError;
        type ReconstructionError = ();

        fn parse(s: &str) -> Result<Self, Self::ParseError> {
            ListVariablePool::parse_iter(&mut s.split("\n"))
        }

        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            Ok(self.0
                .values()
                .fold(String::new(), |acc, i|
                    format!("{}\n{}", acc, i.reconstruct().unwrap() /* this never fails */ )
                )
                .trim()
                .to_string())
        }
    }

    #[derive(Error, Debug)]
    #[error("error while parsing list variable at count {count}")]
    pub struct ListVariablePoolParseError {
        pub count: u32,
        pub content: String,

        #[source]
        pub source: ListVariableParseError
    }

    impl Default for ListVariablePool {
        fn default() -> Self {
            ListVariablePool(LinkedHashMap::new())
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct ListVariable {
        pub name: String,
        pub r#type: super::variable::VariableType
    }

    impl Parsable for ListVariable {
        type ParseError = ListVariableParseError;
        type ReconstructionError = ();

        fn parse(s: &str) -> Result<Self, Self::ParseError> {
            let (lvar_type, name) = {
                Ok(s.split_once(":")
                    .ok_or_else(||ListVariableParseError::MissingColon)?)
            }?;

            Ok(ListVariable {
                name: name.to_string(),
                r#type: <super::variable::VariableType as TryFrom<u8>>
                    ::try_from(
                        lvar_type
                            .parse()
                            .map_err(|err|ListVariableParseError::VariableTypeIsNotInt(err))?
                    )
                    .map_err(ListVariableParseError::InvalidVariableType)?,
            })
        }

        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            Ok(format!("{}:{}", self.r#type as u8, self.name))
        }
    }

    #[derive(Error, Debug)]
    pub enum ListVariableParseError {
        #[error("couldn't find a colon, is this malformed?")]
        MissingColon,

        #[error("variable type is not a valid integer")]
        VariableTypeIsNotInt(#[source] ParseIntError),

        #[error("variable type {} is not a valid type", .0.value)]
        InvalidVariableType(#[source] InvalidVariableTypeError)
    }
}

pub mod component {
    use serde::{Serialize, Deserialize};
    use crate::parser::Parsable;
    use thiserror::Error;

    #[derive(Debug, Eq, PartialEq)]
    pub struct ComponentPool(pub Vec<Component>);

    impl ComponentPool {
        pub fn parse_iter<'a>(
            newlines_iter: impl Iterator<Item=&'a str>
        ) -> Result<Self, ComponentPoolParseError> {
            let mut result = Vec::new();

            for (count, line) in
                newlines_iter
                    .take_while(|i| *i != "")
                    .enumerate() {

                result.push(
                    Component::parse(line)
                        .map_err(|err| ComponentPoolParseError {
                            count: count as u32,
                            content: line.to_string(),
                            source: err
                        })?
                )
            }

            Ok(ComponentPool(result))
        }
    }

    impl Parsable for ComponentPool {
        type ParseError = ComponentPoolParseError;
        type ReconstructionError = ComponentPoolReconstructionError;

        fn parse(s: &str) -> Result<ComponentPool, Self::ParseError> {
            ComponentPool::parse_iter(s.split("\n"))
        }

        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            let mut result = String::new();

            for (count, component) in self.0.iter().enumerate() {
                result.push_str(
                    component
                        .reconstruct()
                        .map_err(|err| ComponentPoolReconstructionError {
                            count: count as u32,
                            component: component.to_owned(),
                            source: err
                        })?
                        .as_str()
                );
                result.push('\n');
            }

            result = result.trim_end().to_string();

            Ok(result)
        }
    }

    #[derive(Error, Debug)]
    #[error("error while parsing component count {count} in a component pool")]
    pub struct ComponentPoolParseError {
        pub count: u32,
        pub content: String,

        #[source]
        pub source: serde_json::Error
    }

    #[derive(Error, Debug)]
    #[error("error while reconstructing component count {count}")]
    pub struct ComponentPoolReconstructionError {
        pub count: u32,
        pub component: Component,

        #[source]
        pub source: serde_json::Error
    }

    impl Default for ComponentPool {
        fn default() -> Self {
            ComponentPool(vec![])
        }
    }

    #[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
    pub struct Component {
        #[serde(rename = "componentId")]
        pub id: String,

        pub param1: String,
        pub param2: String,
        pub param3: String,

        pub r#type: u8,
    }

    impl Component {
        /// Creates a [`Component`]
        pub fn new(id: String, param1: String, param2: String, param3: String, r#type: u8) -> Component {
            Component { id, param1, param2, param3, r#type }
        }

        /// Creates a [`Component`] without any parameters
        pub fn new_empty(id: String, r#type: u8) -> Component {
            Component { id, param1: "".to_string(), param2: "".to_string(), param3: "".to_string(), r#type }
        }

        /// Creates a [`Component`] with a single parameter
        pub fn new_1param(id: String, param1: String, r#type: u8) -> Component {
            Component { id, param1, param2: "".to_string(), param3: "".to_string(), r#type }
        }

        /// Creates a [`Component`] with two parameters
        pub fn new_2params(id: String, param1: String, param2: String, r#type: u8) -> Component {
            Component { id, param1, param2, param3: "".to_string(), r#type }
        }
    }

    impl Parsable for Component {
        type ParseError = serde_json::Error;
        type ReconstructionError = serde_json::Error;

        fn parse(s: &str) -> Result<Component, Self::ParseError> {
            serde_json::from_str(s)
        }

        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            serde_json::to_string(self)
        }
    }
}

pub mod more_block {
    use ritelinked::LinkedHashMap;
    use crate::parser::Parsable;
    use thiserror::Error;

    #[derive(Debug, Eq, PartialEq)]
    pub struct MoreBlockPool(pub LinkedHashMap<String, MoreBlock>);

    impl MoreBlockPool {
        /// Parses a moreblock pool using an iterator of newlines
        pub fn parse_iter<'a>(
            newline_iter: &mut impl Iterator<Item=&'a str>
        ) -> Result<Self, MoreBlockPoolParseError> {

            let mut more_blocks = Vec::new();

            for (line_count, line) in
                newline_iter
                    .by_ref()
                    .take_while(|i| *i != "")
                    .enumerate() {

                more_blocks.push(
                    MoreBlock::parse(line)
                        .map_err(|err| MoreBlockPoolParseError {
                            count: line_count as u32,
                            content: line.to_string(),
                            source: err
                        })?)
            }

            let mut result = LinkedHashMap::<String, MoreBlock>::new();

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
        type ParseError = MoreBlockPoolParseError;
        type ReconstructionError = ();

        /// Parses a moreblock pool (list of moreblock declarations), make sure to not include its
        /// header into the input
        fn parse(s: &str) -> Result<MoreBlockPool, Self::ParseError> {
            MoreBlockPool::parse_iter(&mut s.split("\n"))
        }

        /// Reconstructs a moreblock pool to its string form
        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            Ok(self.0
                .values()
                .try_fold(String::new(), |ac, i| {
                    Ok(format!("{}\n{}", ac, i.reconstruct().unwrap() /* this will never error */))
                })?
                .trim()
                .to_string())
        }
    }

    #[derive(Error, Debug)]
    #[error("error while parsing moreblock at count {count}")]
    pub struct MoreBlockPoolParseError {
        pub count: u32,
        pub content: String,

        #[source]
        pub source: MoreBlockParseError,
    }

    impl Default for MoreBlockPool {
        fn default() -> Self {
            MoreBlockPool(LinkedHashMap::new())
        }
    }

    /// Represents an item of @ActivityName.java_func
    #[derive(Debug, Clone, Eq, PartialEq)]
    pub struct MoreBlock {
        pub id: String,
        pub spec: String,
    }

    impl Parsable for MoreBlock {
        type ParseError = MoreBlockParseError;
        type ReconstructionError = ();

        /// Parses a moreblock item, example:
        /// ```
        /// execute_shell:execute_shell %s.command
        /// ```
        fn parse(s: &str) -> Result<MoreBlock, Self::ParseError> {
            let (id, spec) = s.split_once(':')
                .ok_or_else(||MoreBlockParseError::MalformedMoreBlock {
                    content: s.to_string()
                })?;

            Ok(MoreBlock {
                id: id.to_string(),
                spec: spec.to_string()
            })
        }

        /// Reconstructs a moreblock into its original form
        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            Ok(format!("{}:{}", self.id, self.spec))
        }
    }

    #[derive(Error, Debug)]
    pub enum MoreBlockParseError {
        #[error("moreblock is malformed, content: {content}")]
        MalformedMoreBlock {
            content: String,
        }
    }
}

pub mod event {
    use serde::{Serialize, Deserialize};
    use crate::parser::Parsable;
    use thiserror::Error;

    #[derive(Debug, Eq, PartialEq)]
    pub struct EventPool(pub Vec<Event>);

    impl EventPool {
        /// Parses an event pool from a newline iterator
        pub fn parse_iter<'a>(
            newline_iter: &mut impl Iterator<Item=&'a str>
        ) -> Result<Self, EventPoolParseError> {

            let mut result = Vec::new();

            // iter until empty string
            for (index, line) in newline_iter
                .by_ref()
                .take_while(|i| *i != "")
                .enumerate() {

                result.push(
                    Event::parse(line)
                        .map_err(|err| EventPoolParseError {
                            content: line.to_string(),
                            count: index as u32,
                            source: err
                        })?
                );
            }

            Ok(EventPool(result))
        }
    }

    impl Parsable for EventPool {
        type ParseError = EventPoolParseError;
        type ReconstructionError = ();

        fn parse(decrypted_content: &str) -> Result<Self, Self::ParseError> {
            EventPool::parse_iter(&mut decrypted_content.split("\n"))
        }

        fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
            Ok(self.0
                .iter()
                .fold(String::new(), |acc, event|
                    format!("{}\n{}", acc, event.reconstruct().unwrap() /* this never fails */ )
                )
                .trim()
                .to_string())
        }
    }

    #[derive(Error, Debug)]
    #[error("error while parsing an event after {count} lines, content: {content}")]
    pub struct EventPoolParseError {
        pub content: String,
        pub count: u32,

        #[source]
        pub source: serde_json::Error,
    }

    impl Default for EventPool {
        fn default() -> Self {
            EventPool(vec![])
        }
    }

    #[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Event {
        pub event_name: String,
        pub event_type: u8,
        pub target_id: String,
        pub target_type: u8,
    }

    impl Parsable for Event {
        type ParseError = serde_json::Error;
        type ReconstructionError = serde_json::Error;

        fn parse(decrypted_content: &str) -> Result<Self, Self::ParseError> {
            serde_json::from_str(decrypted_content)
        }

        fn reconstruct(&self) -> Result<String, Self::ParseError> {
            serde_json::to_string(self)
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
    type ParseError = BlockContainerHeaderParseError;
    type ReconstructionError = ();

    /// Parses the header of a block container
    fn parse(s: &str) -> Result<BlockContainerHeader, Self::ParseError> {
        if !s.starts_with("@") {
            return Err(BlockContainerHeaderParseError::DoesntStartWithAtSign)
        }

        let mut parts = s.split(".java_");
        let screen_name = parts.next()   // [1..] to get rid of the @ at the start
            .ok_or_else(||BlockContainerHeaderParseError::NoScreenName)?[1..].to_string();

        let container_name = parts.next()
            .ok_or_else(||BlockContainerHeaderParseError::NoContainerName)?.to_string();

        Ok(BlockContainerHeader {
            screen_name,
            container_name
        })
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        Ok(format!("@{}.java_{}", self.screen_name, self.container_name))
    }
}

#[derive(Error, Debug)]
pub enum BlockContainerHeaderParseError {
    #[error("header does not start with a `@`")]
    DoesntStartWithAtSign,

    #[error("couldn't get the screen name")]
    NoScreenName,

    #[error("couldn't get the container name")]
    NoContainerName,
}

/// Basically a list of blocks
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct BlockContainer(pub Vec<Block>);

impl BlockContainer {
    /// Parses a block container from an iterator of newlines
    fn parse_iter<'a>(
        newline_iter: &mut impl Iterator<Item=&'a str>
    ) -> Result<Self, BlockContainerParseError> {
        let mut result = Vec::<Block>::new();

        for (index, line) in newline_iter.by_ref().take_while(|i| *i != "").enumerate() {
            result.push(
                Block::parse(line)
                    .map_err(|err| BlockContainerParseError {
                        block_count: index as u32,
                        source: err
                    })?);
        }

        Ok(BlockContainer(result))
    }
}

impl Parsable for BlockContainer {
    type ParseError = BlockContainerParseError;
    type ReconstructionError = BlockContainerReconstructionError;

    /// This just parses a list of blocks, do not include the header
    fn parse(s: &str) -> Result<BlockContainer, Self::ParseError> {
        BlockContainer::parse_iter(&mut s.split("\n"))
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        let mut result = String::new();

        for (index, block) in self.0.iter().enumerate() {
            result.push_str(
                block.reconstruct()
                    .map_err(|err| BlockContainerReconstructionError {
                        block_count: index as u32,
                        block: block.clone(),
                        source: err
                    })?
                    .as_str()
            );
            result.push('\n');
        }

        Ok(result)
    }
}

#[derive(Error, Debug)]
#[error("error while parsing a block of a block container")]
pub struct BlockContainerParseError {
    pub block_count: u32,

    #[source]
    pub source: serde_json::Error,
}

#[derive(Error, Debug)]
#[error("error while reconstructing block {block:?}")]
pub struct BlockContainerReconstructionError {
    pub block_count: u32,
    pub block: Block,

    #[source]
    pub source: serde_json::Error
}

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq)]
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
    type ParseError = serde_json::Error;
    type ReconstructionError = serde_json::Error;

    fn parse(s: &str) -> Result<Block, Self::ParseError> {
        serde_json::from_str(s)
    }

    fn reconstruct(&self) -> Result<String, Self::ReconstructionError> {
        serde_json::to_string(self)
    }
}