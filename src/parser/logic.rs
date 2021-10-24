use std::collections::HashMap;

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
    pub type ComponentPool = Vec<Component>;

    pub struct Component {}
}

pub mod more_block {
    pub type MoreBlockPool = Vec<MoreBlock>;

    pub struct MoreBlock {}
}

type Event = BlocksContainer;

pub struct BlocksContainer {

}

pub struct Block {

}