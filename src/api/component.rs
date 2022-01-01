use crate::SWRSError;

/// An enum that contains all types of components with its parameters
#[derive(Debug, Clone, PartialEq)]
pub enum Component {
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

impl TryFrom<crate::parser::logic::component::Component> for Component {
    type Error = SWRSError;

    fn try_from(value: crate::parser::logic::component::Component) -> Result<Self, Self::Error> {
        Ok(match value.r#type {
            1 => Component::Intent,
            2 => Component::SharedPreferences { path: value.param1 },
            3 => Component::Calendar,
            4 => Component::Vibrator,
            5 => Component::Timer,
            6 => Component::FirebaseDatabase { path: value.param1 },
            7 => Component::Dialog,
            8 => Component::MediaPlayer,
            9 => Component::SoundPool,
            10 => Component::ObjectAnimator,
            11 => Component::Gyroscope,
            12 => Component::FirebaseAuth,
            13 => Component::InterstitialAd,
            14 => Component::FirebaseStorage { path: value.param1 },
            15 => Component::Camera,
            16 => Component::FilePicker { mime_type: value.param1 },
            17 => Component::RequestNetwork,
            18 => Component::TextToSpeech,
            19 => Component::SpeechToText,
            20 => Component::BluetoothConnect,
            21 => Component::LocationManager,
            _ => Err(SWRSError::ParseError(format!(
                "Unknown component type: {}, component id: {}", value.r#type, value.id
            )))?
        })
    }
}