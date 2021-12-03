use std::collections::HashMap;
use crate::api::block::Blocks;
use crate::api::block::spec::Spec;
use crate::api::component::Component;
use crate::api::layout::Layout;
use crate::api::view::View;
use crate::parser::file::{ActivityOptions, KeyboardSetting, Orientation, Theme};
use crate::parser::logic::variable::VariableType;

/// A model that represents a screen / activity in a project
pub struct Screen {
    /// The layout name of this screen (without the .xml part); Retrieved from the `view` file
    pub layout_name: String,

    /// The java name of this screen (with an "Activity" after it); Retrieved from the `logic` file
    pub java_name: String,

    /// The root view of layout of the screen
    pub layout: View,

    /// All the global variables in this screen
    pub variables: HashMap<String, Variable>,

    /// All the moreblocks in this screen
    pub more_blocks: HashMap<String, MoreBlock>,

    /// All the components in this screen
    pub components: HashMap<String, Component>,

    /// All the events in this screen
    pub events: Vec<Event>,

    pub fullscreen_enabled: bool,
    pub toolbar_enabled: bool,
    pub drawer_enabled: bool,
    pub fab_enabled: bool,

    pub orientation: Orientation,
    pub theme: Theme,
    pub keyboard_setting: KeyboardSetting,
}

/// A model that represents a global variable
pub struct Variable {
    pub name: String,
    pub variable_type: VariableType,
}

/// A model that represents a moreblock
pub struct MoreBlock {
    pub name: String,
    pub spec: Spec,
    pub code: Blocks,
}

/// A model that represents an event
pub struct Event {
    pub name: String,
    pub event_type: EventType,
    pub code: Blocks,
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