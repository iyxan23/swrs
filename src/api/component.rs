use thiserror::Error;
use crate::parser::logic::component::{Component as ParserComponent};

/// An enum that contains all kinds of components with its parameters
#[derive(Debug, Clone, PartialEq)]
pub enum ComponentKind {
    Intent,
    SharedPreferences {
        path: String, // param1
    },
    Calendar,
    Vibrator,
    Timer,
    FirebaseDatabase {
        path: String, // param1
    },
    Dialog,
    MediaPlayer,
    SoundPool,
    ObjectAnimator,
    Gyroscope,
    FirebaseAuth,
    InterstitialAd,
    FirebaseStorage {
        path: String, // param1
    },
    Camera,
    FilePicker {
        mime_type: String, // param1
    },
    RequestNetwork,
    TextToSpeech,
    SpeechToText,
    BluetoothConnect,
    LocationManager,
}

impl ComponentKind {
    /// Constructs a [`ComponentKind`] using [`ParserComponent`]
    pub fn from_parser_component(component: &ParserComponent) -> Result<ComponentKind, UnknownComponentType> {
        Ok(match component.r#type {
            1 => ComponentKind::Intent,
            2 => ComponentKind::SharedPreferences { path: component.param1.to_owned() },
            3 => ComponentKind::Calendar,
            4 => ComponentKind::Vibrator,
            5 => ComponentKind::Timer,
            6 => ComponentKind::FirebaseDatabase { path: component.param1.to_owned() },
            7 => ComponentKind::Dialog,
            8 => ComponentKind::MediaPlayer,
            9 => ComponentKind::SoundPool,
            10 => ComponentKind::ObjectAnimator,
            11 => ComponentKind::Gyroscope,
            12 => ComponentKind::FirebaseAuth,
            13 => ComponentKind::InterstitialAd,
            14 => ComponentKind::FirebaseStorage { path: component.param1.to_owned() },
            15 => ComponentKind::Camera,
            16 => ComponentKind::FilePicker { mime_type: component.param1.to_owned() },
            17 => ComponentKind::RequestNetwork,
            18 => ComponentKind::TextToSpeech,
            19 => ComponentKind::SpeechToText,
            20 => ComponentKind::BluetoothConnect,
            21 => ComponentKind::LocationManager,
            _ => Err(UnknownComponentType {
                component_type: component.r#type,
                component_id: component.id.to_owned()
            })?
        })
    }

    /// Transforms [`ComponentKind`] back to [`ParserComponent`]
    pub fn into_parser_component(self, id: String) -> ParserComponent {
        match self {
            ComponentKind::Intent => ParserComponent::new_empty(id, 1),
            ComponentKind::SharedPreferences { path } => ParserComponent::new_1param(id, path, 2),
            ComponentKind::Calendar => ParserComponent::new_empty(id, 3),
            ComponentKind::Vibrator => ParserComponent::new_empty(id, 4),
            ComponentKind::Timer => ParserComponent::new_empty(id, 5),
            ComponentKind::FirebaseDatabase { path } => ParserComponent::new_1param(id, path, 6),
            ComponentKind::Dialog => ParserComponent::new_empty(id, 7),
            ComponentKind::MediaPlayer => ParserComponent::new_empty(id, 8),
            ComponentKind::SoundPool => ParserComponent::new_empty(id, 9),
            ComponentKind::ObjectAnimator => ParserComponent::new_empty(id, 10),
            ComponentKind::Gyroscope => ParserComponent::new_empty(id, 11),
            ComponentKind::FirebaseAuth => ParserComponent::new_empty(id, 12),
            ComponentKind::InterstitialAd => ParserComponent::new_empty(id, 13),
            ComponentKind::FirebaseStorage { path } => ParserComponent::new_1param(id, path, 14),
            ComponentKind::Camera => ParserComponent::new_empty(id, 15),
            ComponentKind::FilePicker { mime_type } => ParserComponent::new_1param(id, mime_type, 16),
            ComponentKind::RequestNetwork => ParserComponent::new_empty(id, 17),
            ComponentKind::TextToSpeech => ParserComponent::new_empty(id, 18),
            ComponentKind::SpeechToText => ParserComponent::new_empty(id, 19),
            ComponentKind::BluetoothConnect => ParserComponent::new_empty(id, 20),
            ComponentKind::LocationManager => ParserComponent::new_empty(id, 21),
        }
    }
}

#[derive(Error, Debug)]
#[error("unknown component type `{component_type}` of component id {component_id}")]
pub struct UnknownComponentType {
    pub component_type: u8,
    pub component_id: String,
}