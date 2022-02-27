use std::str::FromStr;
use crate::{LinkedHashMap, SWRSError};
use crate::api::block::Blocks;
use crate::api::block::spec::Spec;
use crate::api::component::{ComponentKind, UnknownComponentType};
use crate::api::view::{parse_raw_layout, View};
use crate::parser::file::{FileItem, KeyboardSetting, Orientation, Theme};
use crate::parser::logic::{BlockContainer, ScreenLogic};
use crate::parser::logic::list_variable::ListVariable;
use crate::parser::logic::variable::Variable;
use crate::parser::view::Layout as ViewScreen;
use crate::SWRSResult;
use thiserror::Error;

/// A model that represents a screen / activity in a project
#[derive(Debug, Clone, PartialEq)]
pub struct Screen {
    /// The layout name of this screen (without the .xml part); Retrieved from the `view` file
    pub layout_name: String,

    /// The java name of this screen (with an "Activity" after it); Retrieved from the `logic` file
    pub java_name: String,

    /// The root view of layout of the screen
    pub layout: Vec<View>,

    /// All the global variables in this screen
    pub variables: LinkedHashMap<String, Variable>,

    /// ALl the global list variables in this screen
    ///
    /// Planned to be merged with variables
    pub list_variables: LinkedHashMap<String, ListVariable>,

    /// All the moreblocks in this screen
    pub more_blocks: LinkedHashMap<String, MoreBlock>,

    /// All the components in this screen
    pub components: LinkedHashMap<String, ComponentKind>,

    /// All the events in this screen
    pub events: Vec<Event>,

    /// The fab of this view (if exists)
    pub fab: Option<View>,

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

type ParserMoreBlock = crate::parser::logic::more_block::MoreBlock;

impl MoreBlock {
    /// Turns this into a [`crate::parser::logic::more_block::MoreBlock`], returns its converted
    /// struct and its blocks ([`Blocks`])
    pub fn into_parser_more_block(self) -> (ParserMoreBlock, Blocks) {
        (ParserMoreBlock { id: self.name, spec: self.spec.to_string() }, self.code)
    }
}

/// A model that represents an event
#[derive(Debug, Clone, PartialEq)]
pub struct Event {
    pub name: String,
    pub event_type: EventType,
    pub code: Blocks,
}

impl Event {
    /// Generates the block container id from the event name and type
    pub fn get_block_container_id(&self) -> String {
        // todo: check component event and other activity events
        match &self.event_type {
            EventType::ViewEvent { id } => format!("{}_{}", id, self.name),
            EventType::ComponentEvent { id, .. } => format!("{}_{}", id, self.name),
            EventType::ActivityEvent => match self.name.as_str() {
                "onCreate" => format!("{}_initializeLogic", self.name),
                _ => format!("{}_{}", self.name, self.name)
            }
        }
    }

    /// Turns this into a [`crate::parser::logic::event::Event`], returns its converted struct and
    /// its blocks ([`Blocks`])
    pub fn into_parser_event(self) -> (ParserEvent, Blocks) {
        let mut event = ParserEvent {
            event_name: self.name,
            event_type: 0,
            target_id: "".to_string(),
            target_type: 0
        };

        self.event_type.apply_to_parser_event(&mut event);

        (event, self.code)
    }
}

type ParserEvent = crate::parser::logic::event::Event;

impl TryFrom<ParserEvent> for Event {
    type Error = UnknownEventType;

    /// Creates an api::event from a parser::event, note that the returned event has no code
    fn try_from(value: ParserEvent) -> Result<Self, Self::Error> {
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
    pub fn from_parser_event(event: &ParserEvent) -> Result<EventType, UnknownEventType> {
        Ok(match event.event_type {
            1 => EventType::ViewEvent { id: event.target_id.to_owned() },
            2 => EventType::ComponentEvent {
                id: event.target_id.to_owned(),
                component_type: event.target_type,
            },
            3 => EventType::ActivityEvent,
                // if you asked, no ActivityEvent doesn't have its activity event name written
                // on target_type, its already on the event name

            _ => Err(UnknownEventType {
                event_name: event.event_name.to_owned(),
                event_type: event.event_type
            })?
        })
    }

    pub fn apply_to_parser_event(self, event: &mut ParserEvent) {
        match self {
            EventType::ViewEvent { id } => {
                event.event_type = 1;
                event.target_id = id;
            }

            EventType::ComponentEvent { id, component_type } => {
                event.event_type = 2;
                event.target_id = id;
                event.target_type = component_type;
            }

            EventType::ActivityEvent => event.event_type = 3
        }
    }
}

#[derive(Error, Debug)]
#[error("unknown event type `{event_type}` for event `{event_name}`")]
pub struct UnknownEventType {
    pub event_name: String,
    pub event_type: u8,
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
                "Unable to associate the blocks of more block {}:\n{}", more_block.id, err
            )))?
    })
}

impl Screen {
    pub fn from_parsed(
        layout_name: String,
        logic_name: String,
        file_entry: FileItem,
        view_entry: ViewScreen,
        mut logic_entry: ScreenLogic,
        fab: Option<View>,
    ) -> Result<Self, ScreenConstructionError> {
        Ok(Screen {
            layout_name,
            java_name: logic_name,
            layout: parse_raw_layout(view_entry)?,
            variables: logic_entry.variables.unwrap_or_default().0,
            list_variables: logic_entry.list_variables.unwrap_or_default().0,

            // basically just converts these parser's list of moreblocks/components/events into our
            // type defined in this module
            more_blocks: logic_entry.more_blocks.unwrap_or_default().0
                .into_iter()
                .map(|(mb_id, mb)|
                    Ok((mb_id.to_owned(), associate_blocks_with_more_block(
                        logic_entry.block_containers
                            .remove(&*format!("{}_moreBlock", mb_id))
                            .unwrap_or_else(||BlockContainer(vec![])),
                        mb
                    )?))
                )
                .collect::<Result<LinkedHashMap<String, MoreBlock>, _>>()?,

            components: logic_entry.components.unwrap_or_default().0
                .into_iter()
                .map(|cmp| {
                    let id = cmp.id.clone();
                    ComponentKind::from_parser_component(&cmp)
                        .map(|cmp| ((id, cmp)))
                })
                .collect::<Result<LinkedHashMap<String, ComponentKind>, UnknownComponentType>>()
                .map_err(ScreenConstructionError::UnknownComponentType)?,

            events: logic_entry.events.unwrap_or_default().0
                .into_iter()
                .map(|event| {
                    let mut event = Event::try_from(event)
                        .map_err(ScreenConstructionError::UnknownEventType)?;

                    let code = logic_entry.block_containers
                        .remove(event.get_block_container_id().as_str())
                        .ok_or_else(||ScreenConstructionError::MissingBlocks {
                            event_name: event.name.to_owned()
                        })?;

                    event.code = Blocks::try_from(code)
                        .map_err(|err| todo!())?;

                    Ok(event)
                })
                .collect::<Result<Vec<Event>, ScreenConstructionError>>()?,

            fab,
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

#[derive(Error, Debug)]
pub enum ScreenConstructionError {
    #[error("{0}")]
    UnknownEventType(#[from] UnknownEventType),

    #[error("{0}")]
    UnknownComponentType(#[from] UnknownComponentType),

    #[error("couldn't find the block container of event `{event_name}`")]
    MissingBlocks {
        event_name: String
    }
}