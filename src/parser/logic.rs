use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::color::Color;
use crate::error::{SWRSError, SWRSResult};

pub struct Logic {
    pub screens: HashMap<String, ScreenLogic>
}

pub struct ScreenLogic {
    pub name: String,
    pub events: Vec<Event>,
    pub variables: variable::VariablePool,
    pub components: component::ComponentPool,
    pub more_blocks: more_block::MoreBlockPool,
}

pub mod variable {
    use std::collections::HashMap;
    use std::convert::TryFrom;
    use crate::error::{SWRSError, SWRSResult};

    #[derive(Debug, Eq, PartialEq)]
    pub struct VariablePool(HashMap<String, Variable>);

    impl VariablePool {
        /// Parses a variable pool, do not include the header in the input
        pub fn parse(s: &str) -> SWRSResult<VariablePool> {
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

        pub fn variable(&self, name: &str) -> Option<&Variable> {
            self.0.get(name)
        }

        pub fn variables(&self) -> Vec<(&String, &Variable)> {
            self.0.iter().collect()
        }
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Variable {
        pub name: String,
        pub r#type: VariableType,
    }

    impl Variable {
        pub fn parse(s: &str) -> SWRSResult<Variable> {
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

    pub struct ComponentPool(Vec<Component>);

    impl ComponentPool {
        pub fn parse(s: &str) -> SWRSResult<ComponentPool> {
            Ok(ComponentPool(
                s.split("\n")
                    .map(Component::parse)
                    .collect::<SWRSResult<Vec<Component>>>()?
            ))
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

    impl Component {
        pub fn parse(s: &str) -> SWRSResult<Component> {
            serde_json::from_str(s)
                .map_err(|e| SWRSError::ParseError(
                    format!("Failed to parse component: {}", e.to_string())
                ))
        }
    }
}

pub mod more_block {
    pub type MoreBlockPool = Vec<MoreBlock>;

    pub struct MoreBlock {}
}

type Event = BlocksContainer;

pub struct BlocksContainer {

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

impl Block {
    pub fn parse(s: &str) -> SWRSResult<Block> {
        serde_json::from_str(s)
            .map_err(|e|SWRSError::ParseError(e.to_string()))
    }
}