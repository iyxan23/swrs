pub struct Logic {
    pub screens: Vec<ScreenLogic>
}

pub struct ScreenLogic {
    pub events: Vec<Event>,
    pub variables: variable::VariablePool,
    pub components: component::ComponentPool,
    pub more_blocks: more_block::MoreBlockPool,
}

pub mod variable {
    pub type VariablePool = Vec<Variable>;

    pub struct Variable {
        pub r#type: VariableType,
        pub name: String,
    }

    #[repr(u8)]
    pub enum VariableType {
        Boolean,
        Integer,
        String,
        HashMap, // <String, Object>
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