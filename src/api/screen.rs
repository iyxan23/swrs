use std::collections::HashMap;
use std::convert::Infallible;
use crate::{LinkedHashMap, SWRSError};
use crate::api::block::Blocks;
use crate::api::block::spec::Spec;
use crate::api::component::Component;
use crate::api::view::{screen_to_view, View};
use crate::parser::file::{FileItem, KeyboardSetting, Orientation, Theme};
use crate::parser::file::Theme::Default;
use crate::parser::logic::component::ComponentPool;
use crate::parser::logic::event::EventPool;
use crate::parser::logic::more_block::MoreBlockPool;
use crate::parser::logic::ScreenLogic;
use crate::parser::logic::variable::{Variable, VariablePool, VariableType};
use crate::parser::view::Screen as ViewScreen;
use crate::SWRSResult;

/// A model that represents a screen / activity in a project
#[derive(Debug, Clone, PartialEq)]
pub struct Screen {
    /// The layout name of this screen (without the .xml part); Retrieved from the `view` file
    pub layout_name: String,

    /// The java name of this screen (with an "Activity" after it); Retrieved from the `logic` file
    pub java_name: String,

    /// The root view of layout of the screen
    pub layout: View,

    /// All the global variables in this screen
    pub variables: LinkedHashMap<String, Variable>,

    /// All the moreblocks in this screen
    pub more_blocks: LinkedHashMap<String, MoreBlock>,

    /// All the components in this screen
    pub components: LinkedHashMap<String, Component>,

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

/// A model that represents a moreblock
#[derive(Debug, Clone, PartialEq)]
pub struct MoreBlock {
    pub name: String,
    pub spec: Spec,
    pub code: Blocks,
}

/// A model that represents an event
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub name: String,
    pub event_type: EventType,
    pub code: Blocks,
}

#[derive(Debug, Clone, PartialEq)]
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

impl Screen {
    pub fn from_parsed(
        layout_name: String,
        logic_name: String,
        file_entry: FileItem,
        view_entry: ViewScreen,
        logic_entry: ScreenLogic,
    ) -> SWRSResult<Self> {
        Ok(Screen {
            layout_name,
            java_name: logic_name,
            layout: screen_to_view(view_entry)?,
            variables: logic_entry.variables.unwrap_or_default().0,

            // basically just converts these parser's list of moreblocks/components/events into our
            // type defined in this module
            more_blocks: logic_entry.more_blocks.unwrap_or_default().0
                .into_iter()
                .map(|(mb_id, mb)|
                    todo!("implement converting parser's moreblock struct into our moreblock (that contains Blocks)"))
                .collect::<SWRSResult<LinkedHashMap<String, MoreBlock>>>()?,

            components: logic_entry.components.unwrap_or_default().0
                .into_iter()
                .map(|cmp| {
                    let id = cmp.id.clone();
                    Component::try_from(cmp)
                        .map(|conv_cmp| (id, conv_cmp))
                })
                .collect::<SWRSResult<LinkedHashMap<String, Component>>>()?,

            events: logic_entry.events.unwrap_or_default().0
                .into_iter()
                .map(|event|
                    todo!("implement converting parser's event struct into our event (that contains Blocks)"))
                .collect::<SWRSResult<Vec<Event>>>()?,

            fullscreen_enabled: file_entry.options.fullscreen_enabled,
            toolbar_enabled: file_entry.options.toolbar_enabled,
            drawer_enabled: file_entry.options.drawer_enabled,
            fab_enabled: file_entry.options.fab_enabled,
            orientation: file_entry.orientation,
            theme: file_entry.theme,
            keyboard_setting: file_entry.keyboard_setting
        })
    }
}