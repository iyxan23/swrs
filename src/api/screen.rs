use std::collections::HashMap;
use crate::api::block::Blocks;
use crate::api::component::Component;
use crate::api::layout::Layout;
use crate::api::view::View;
use crate::parser::logic::variable::VariableType;

pub struct Screen {
    pub layout_name: String,
    pub java_name: String,

    pub layout: View,

    pub variables: HashMap<String, Variable>,
    pub more_blocks: HashMap<String, MoreBlock>,
    pub components: HashMap<String, Component>,
    pub events: Vec<Event>,
}

pub struct Variable {
    pub name: String,
    pub r#type: VariableType,
}

pub struct MoreBlock {
    pub name: String,
    pub spec: Spec,
    pub code: Blocks,
}

pub struct Event {
    pub name: String,
    pub event_type: EventType,
}

pub enum EventType {
    ViewEvent {
        id: String
    },
    ComponentEvent {
        id: String,
        component_type: u8,
    },
    ActivityEvent,
}