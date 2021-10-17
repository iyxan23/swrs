use serde::{Serialize, Deserialize};
use serde_repr::{Serialize_repr, Deserialize_repr};
use crate::color::Color;

#[derive(Debug, Eq, PartialEq)]
pub struct View {
    pub screens: Vec<Screen>
}

#[derive(Debug, Eq, PartialEq)]
pub struct Screen {
    pub views: Vec<AndroidView>,
    pub fab_view: AndroidView,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct AndroidView {
    pub adSize: String, // ""
    pub adUnitId: String, // ""
    pub alpha: f32, // 1.0
    pub checked: bool, // (int) 0
    pub choiceMode: ChoiceMode, // None
    pub clickable: bool, // (int) 1
    pub customView: String, // ""

    /// Divider height of a listview (in dp)
    pub dividerHeight: u16, // 0
    pub enabled: bool, // (int) 0

    /// Sets the first day of a week for a calendar
    /// (https://developer.android.com/reference/android/widget/CalendarView#setFirstDayOfWeek(int))
    pub firstDayOfWeek: u8, // 1
    pub id: String, // "something1"
    pub image: ImageConfig,
    pub indeterminate: bool, // (str) "false"
    pub index: u32, // 0
    pub layout: LayoutConfig,
    pub max: u32, // 100
    pub parent: String, // "something1"
    pub parentType: u8, // 0
    pub preId: String, // ""
    pub preIndex: u32, // 0
    pub preParentType: u8, // 0
    pub progress: u32, // 0
    pub progressStyle: String, // "?android:progressBarStyle", Enum?
    pub scaleX: f32, // 1.0
    pub scaleY: f32, // 1.0
    pub spinnerMode: SpinnerMode, // 1: Dropdown
    pub text: TextConfig,
    pub translationX: f32, // 0.0
    pub translationY: f32, // 0.0
    pub r#type: u8, // 0
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum ChoiceMode {
    None = 0,
    Single = 1,
    Multi = 2,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum SpinnerMode {
    Dialog = 0,
    Dropdown = 1,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ImageConfig {
    pub rotate: i16, // 0
    pub scale_type: ImageScaleType, // CENTER
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ImageScaleType {
    Center,
    FitXy,
    FitStart,
    FitCenter,
    FitEnd,
    CenterCrop,
    CenterInside,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct LayoutConfig {
    pub backgroundColor: Color, // 16777215,
    pub gravity: u8, // 0 - Enum?
    pub height: Size, // -2: MATCH_PARENT
    pub layoutGravity: u8, // 0 - Enum?
    pub marginBottom: u32, // 0
    pub marginLeft: u32, // 0
    pub marginRight: u32, // 0
    pub marginTop: u32, // 0
    pub orientation: Orientation, // 1: vertical
    pub paddingBottom: u32, // 8
    pub paddingLeft: u32, // 8
    pub paddingRight: u32, // 8
    pub paddingTop: u32, // 8
    pub weight: u32, // 0
    pub weightSum: u32, // 0
    pub width: Size, // -1: WRAP_CONTENT
}

#[derive(Debug, Eq, PartialEq)]
// TODO: impl serialize n deserialize for Size
pub enum Size {
    MatchParent, // -2
    WrapContent, // -1
    Fixed(u16)
}

#[derive(Debug, Eq, PartialEq)]
#[repr(i8)]
pub enum Orientation {
    Vertical = 1,
    Horizontal = -1,
}

#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TextConfig {
    pub hint: String, // ""
    pub hintColor: Color, // -10453621
    pub imeOption: ImeOption, // 1: None
    pub inputType: InputType, // 1: Text
    pub line: u32, // 0
    pub singleLine: bool, // (int) 0
    pub text: String, // ""
    pub textColor: Color, // -16777216
    pub textFont: String, // "default_font",
    pub textSize: u32, // 12,
    pub textType: TextType, // 0: Normal
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum ImeOption {
    Normal = 0,
    None = 1,
    Go = 2,
    Search = 3,
    Send = 4,
    Next = 5,
    Done = 6,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u16)]
pub enum InputType {
    NumberDecimal = 8194,
    NumberSigned = 4098,
    NumberSignedDecimal = 12290,
    Password = 129,
    Phone = 3,
    Text = 1,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
#[repr(u8)]
pub enum TextType {
    Normal = 0,
    Bold = 1,
    Italic = 2,
    BoldItalic = 3,
}