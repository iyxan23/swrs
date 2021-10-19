use crate::error::{SWRSError, SWRSResult};
use models::AndroidView;

#[derive(Debug, PartialEq)]
pub struct View {
    pub screens: Vec<Screen>
}

#[derive(Debug, PartialEq)]
pub struct Screen {
    pub name: String,
    pub views: Vec<AndroidView>,
    pub fab_view: AndroidView,
}

impl Screen {
    /// Note: the parameter screen_content & fab_view must include the header "@{name}.xml(_fab)"
    /// at the beginning
    pub fn parse(screen_content: &str, fab_view: &str) -> SWRSResult<Screen> {
        let mut content_iterator = screen_content.split("\n");

        // get the name from the header
        let name = {
            let header =
                (&mut content_iterator)
                    .next()
                    .ok_or_else(||
                        SWRSError::ParseError("EOF whilst trying to read header".to_string())
                    )?;

            if !header.ends_with(".xml") || !header.starts_with("@") {
                Err(SWRSError::ParseError("View header does not have either .xml at the end or @ at the beginning".to_string()))
            } else {
                Ok(&header[1..header.len() - 4])
            }
        }?.to_string();

        // parse the views inside screen_content
        let mut views: Vec<AndroidView> = vec![];
        loop {
            let line = content_iterator.next();
            if line.is_none() { break; }
            let line = line.unwrap();

            views.push(
                AndroidView::parse(line)?
            );
        }

        // get & parse the fab view
        let fab_view =
            AndroidView::parse(
                fab_view
                .split("\n")
                .collect::<Vec<&str>>()
                .get(1)
                .ok_or_else(||
                    SWRSError::ParseError("Couldn't get fab_view's header".to_string())
                )?
            )?;

        Ok(Screen { name, views, fab_view })
    }
}

pub mod models {
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use serde::{Deserialize, Serialize};
    use crate::color::Color;
    use crate::error::{SWRSError, SWRSResult};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct AndroidView {
        pub ad_size: String, // ""
        pub ad_unit_id: String, // ""
        pub alpha: f32, // 1.0
        pub checked: bool, // (int) 0
        pub choice_mode: ChoiceMode, // None
        pub clickable: bool, // (int) 1
        pub custom_view: String, // ""

        /// Divider height of a listview (in dp)
        pub divider_height: u16, // 0
        pub enabled: bool, // (int) 0

        /// Sets the first day of a week for a calendar
        /// (https://developer.android.com/reference/android/widget/CalendarView#setFirstDayOfWeek(int))
        pub first_day_of_week: u8, // 1
        pub id: String, // "something1"
        pub image: ImageConfig,
        pub indeterminate: bool, // (str) "false"
        pub index: u32, // 0
        pub layout: LayoutConfig,
        pub max: u32, // 100
        pub parent: String, // "something1"
        pub parent_type: u8, // 0
        pub pre_id: String, // ""
        pub pre_index: u32, // 0
        pub pre_parent_type: u8, // 0
        pub progress: u32, // 0
        pub progress_style: String, // "?android:progressBarStyle", Enum?
        pub scale_x: f32, // 1.0
        pub scale_y: f32, // 1.0
        pub spinner_mode: SpinnerMode, // 1: Dropdown
        pub text: TextConfig,
        pub translation_x: f32, // 0.0
        pub translation_y: f32, // 0.0
        pub r#type: u8, // 0
    }

    impl AndroidView {
        pub fn parse(decrypted_content: &str) -> SWRSResult<AndroidView> {
            serde_json::from_str(decrypted_content)
                .map_err(|e| SWRSError::ParseError(e.to_string()))
        }
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
    #[serde(rename_all = "camelCase")]
    pub struct ImageConfig {
        pub rotate: i16, // 0
        pub scale_type: image::ImageScaleType, // CENTER
    }

    pub mod image {
        use serde::{Serialize, Deserialize};

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
    }

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct LayoutConfig {
        pub background_color: Color, // 16777215,
        pub gravity: u8, // 0 - Enum?
        pub height: layout::Size, // -2: MATCH_PARENT
        pub layout_gravity: u8, // 0 - Enum?
        pub margin_bottom: u32, // 0
        pub margin_left: u32, // 0
        pub margin_right: u32, // 0
        pub margin_top: u32, // 0
        pub orientation: layout::Orientation, // 1: vertical
        pub padding_bottom: u32, // 8
        pub padding_left: u32, // 8
        pub padding_right: u32, // 8
        pub padding_top: u32, // 8
        pub weight: u32, // 0
        pub weight_sum: u32, // 0
        pub width: layout::Size, // -1: WRAP_CONTENT
    }

    pub mod layout {
        use serde::{Deserialize, Serialize, Deserializer, Serializer};
        use serde_repr::{Serialize_repr, Deserialize_repr};

        #[derive(Debug, Eq, PartialEq)]
        pub enum Size {
            MatchParent, // -2
            WrapContent, // -1
            Fixed(i32)
        }

        impl Serialize for Size {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
                serializer.serialize_i32(match self {
                    Size::MatchParent => { -2 }
                    Size::WrapContent => { -1 }
                    Size::Fixed(num) => { *num }
                })
            }
        }

        impl<'de> Deserialize<'de> for Size {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
                let num = i32::deserialize(deserializer)?;

                Ok(match num {
                    -2 => { Size::MatchParent }
                    -1 => { Size::WrapContent }
                    _ => { Size::Fixed(num) }
                })
            }
        }

        #[derive(Debug, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
        #[repr(i8)]
        pub enum Orientation {
            Vertical = 1,
            Horizontal = -1,
        }
    }

    #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct TextConfig {
        pub hint: String, // ""
        pub hint_color: Color, // -10453621
        pub ime_option: text::ImeOption, // 1: None
        pub input_type: text::InputType, // 1: Text
        pub line: u32, // 0
        pub single_line: bool, // (int) 0
        pub text: String, // ""
        pub text_color: Color, // -16777216
        pub text_font: String, // "default_font",
        pub text_size: u32, // 12,
        pub text_type: text::TextType, // 0: Normal
    }

    pub mod text {
        use serde_repr::{Deserialize_repr, Serialize_repr};

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
    }
}