use std::str::FromStr;
use crate::{LinkedHashMap, SWRSError};
use crate::api::block::Blocks;
use crate::api::block::spec::Spec;
use crate::api::component::Component;
use crate::api::view::{screen_to_view, View};
use crate::parser::file::{FileItem, KeyboardSetting, Orientation, Theme};
use crate::parser::logic::{BlockContainer, ScreenLogic};
use crate::parser::logic::variable::Variable;
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

impl TryFrom<crate::parser::logic::event::Event> for Event {
    type Error = SWRSError;

    /// Creates an api::event from a parser::event, note that the returned event has no code
    fn try_from(value: crate::parser::logic::event::Event) -> Result<Self, Self::Error> {
        Ok(Event {
            event_type: EventType::from_parser_event(&value)?,
            name: value.event_name,
            code: Default::default()
        })
    }
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

impl EventType {
    pub fn from_parser_event(event: &crate::parser::logic::event::Event) -> SWRSResult<EventType> {
        Ok(match event.event_type {
            1 => EventType::ViewEvent { id: event.target_id.to_owned() },
            2 => EventType::ComponentEvent {
                id: event.target_id.to_owned(),
                component_type: event.target_type,
            },
            3 => EventType::ActivityEvent,
                // if you asked, no ActivityEvent doesn't have its activity event name written
                // on target_type, its already on the event name

            _ => Err(SWRSError::ParseError(format!(
                "Unknown event_type of event {}; should be either 1, 2, or 3",
                event.event_name.to_owned()
            )))?
        })
    }
}

fn associate_blocks_with_more_block(
    blocks: BlockContainer,
    more_block: crate::parser::logic::more_block::MoreBlock,
) -> SWRSResult<MoreBlock> {
    Ok(MoreBlock {
        name: more_block.id.to_owned(),
        spec: Spec::from_str(&*more_block.spec)?,
        code: Blocks::try_from(blocks)
            .map_err(|err|SWRSError::ParseError(format!(
                "Unable to associate the blocks of more block {}", more_block.id
            )))?
    })
}

fn associate_blocks_with_event(
    blocks: BlockContainer,
    event: crate::parser::logic::event::Event,
) -> SWRSResult<Event> {
    let mut empty_converted_event = Event::try_from(event)?;
    empty_converted_event.code = Blocks::try_from(blocks)?;

    Ok(empty_converted_event)
}

impl Screen {
    pub fn from_parsed(
        layout_name: String,
        logic_name: String,
        file_entry: FileItem,
        view_entry: ViewScreen,
        mut logic_entry: ScreenLogic,
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
                    Ok((mb_id.to_owned(), associate_blocks_with_more_block(
                        logic_entry.block_containers
                            .remove(mb_id.as_str())
                            .ok_or_else(||SWRSError::ParseError(format!(
                                "Unable to find the blocks for more block {}", mb_id
                            )))?,
                        mb
                    )?))
                )
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
                    associate_blocks_with_event(
                        logic_entry.block_containers
                            .remove(event.event_name.as_str())
                            .ok_or_else(||SWRSError::ParseError(format!(
                                "Unable to find blocks for event {}", event.event_name
                            )))?,
                        event
                    )
                )
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