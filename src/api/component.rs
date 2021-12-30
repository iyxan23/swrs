use crate::SWRSError;

// todo: add more params
/// An enum that contains all types of components with its parameters
#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    Intent,
    SharedPreferences,
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
    FirebaseStorage,
    Camera,
    FilePicker,
    RequestNetwork,
    TextToSpeech,
    SpeechToText,
    BluetoothConnect,
    LocationManager,
}

impl TryFrom<crate::parser::logic::component::Component> for Component {
    type Error = SWRSError;

    fn try_from(value: crate::parser::logic::component::Component) -> Result<Self, Self::Error> {
        todo!()
    }
}