// todo: add more params
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