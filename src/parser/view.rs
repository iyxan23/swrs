use std::collections::HashMap;
use std::str::Split;
use crate::error::{SWRSError, SWRSResult};
use models::AndroidView;
use crate::parser::ProjectData;

#[derive(Debug, PartialEq)]
pub struct View {
    pub screens: Vec<Screen>
}

impl ProjectData for View {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> {
        // This splits the decrypted content into parts of screens
        let mut lines = decrypted_content.split("\n");

        //                                 name  contents
        let mut screen_contents: HashMap<String, String> = HashMap::new();
        //                           name  content
        let mut fab_views: HashMap<String, String> = HashMap::new();

        fn get_screen_contents(lines: &mut Split<&str>) -> SWRSResult<String> {
            let mut result = String::new();

            loop {
                let line = lines.next();
                if line.is_none() { break; }
                let line = line.unwrap();

                // stop on empty line
                if line.trim().is_empty() {
                    // also since we have \n left out on the last line we should trim it
                    result = result.trim_end().to_string();
                    break;
                }

                result.push_str(line);
                result.push_str("\n");
            }

            Ok(result)
        }

        // the plan is to scan these and turn them into lists, then parse the items on the list
        loop {
            let line = lines.next();
            if line.is_none() { break; }
            let line = line.unwrap();

            if line.starts_with("@") {
                if line.ends_with(".xml") {
                    let name = &line[1..line.len() - 4];
                    let content = get_screen_contents(&mut lines)?;

                    screen_contents.insert(name.to_string(), content);
                } else if line.ends_with(".xml_fab") {
                    let name = &line[1..line.len() - 8];
                    let content = lines
                        .next()
                        .ok_or_else(||
                            SWRSError::ParseError(
                                format!("EOF while trying to get {}'s fab content", name)
                            )
                        )?;

                    fab_views.insert(name.to_string(), content.to_string());
                }
            }
        }

        // then build them into a vector of screens
        Ok(View {
            screens:
                screen_contents
                    .drain()
                    .map(|(name, content)| -> SWRSResult<Screen> {
                        Screen::parse(
                            name.as_str(),
                            content.as_str(),
                            fab_views
                                .get(name.as_str())
                                .ok_or_else(||
                                    SWRSError::ParseError(
                                        format!("Cannot find fab view of screen {}", name)
                                    )
                                )?
                        )
                    })
                    .collect::<SWRSResult<Vec<Screen>>>()?
        })
    }

    fn reconstruct(&self) -> SWRSResult<&str> {
        todo!()
    }
}

#[derive(Debug, PartialEq)]
pub struct Screen {
    pub name: String,
    pub views: Vec<AndroidView>,
    pub fab_view: AndroidView,
}

impl Screen {
    /// Note: screen_content & fab_view should not contain its header, the name contained in the
    /// header must be passed through the name parameter
    pub fn parse(name: &str, screen_content: &str, fab_view: &str) -> SWRSResult<Screen> {
        let mut content_iterator = screen_content.split("\n");

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

        // parse the fab view
        let fab_view = AndroidView::parse(fab_view)?;

        let name = name.to_string();

        Ok(Screen { name, views, fab_view })
    }
}

pub mod models {
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use serde::{Deserialize, Serialize};
    use crate::color::Color;
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::serde_util::{bool_to_one_zero, bool_to_str};
    use crate::parser::view::models::layout::gravity::Gravity;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct AndroidView {
        pub ad_size: String, // ""
        pub ad_unit_id: String, // ""
        pub alpha: f32, // 1.0

        #[serde(with = "bool_to_one_zero")]
        pub checked: bool, // (int) 0
        pub choice_mode: ChoiceMode, // None

        #[serde(with = "bool_to_one_zero")]
        pub clickable: bool, // (int) 1
        pub custom_view: String, // ""

        /// Divider height of a listview (in dp)
        pub divider_height: u16, // 0

        #[serde(with = "bool_to_one_zero")]
        pub enabled: bool, // (int) 0

        /// Sets the first day of a week for a calendar
        /// (https://developer.android.com/reference/android/widget/CalendarView#setFirstDayOfWeek(int))
        pub first_day_of_week: u8, // 1
        pub id: String, // "something1"
        pub image: ImageConfig,

        #[serde(with = "bool_to_str")]
        pub indeterminate: bool, // (str) "false"
        pub index: u32, // 0
        pub layout: LayoutConfig,
        pub max: u32, // 100

        #[serde(default)] // <- this value is not present in fab views
        pub parent: String, // "something1"
        pub parent_type: i8, // 0 - note: can be -1 for some reason

        #[serde(default)] // <- this value is not present in fab views
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

        pub fn new_empty(id: &str, r#type: u8, parent_id: &str, parent_type: i8) -> AndroidView {
            AndroidView {
                ad_size: "".to_string(),
                ad_unit_id: "".to_string(),
                alpha: 1.0,
                checked: false,
                choice_mode: ChoiceMode::None,
                clickable: true,
                custom_view: "".to_string(),
                divider_height: 0,
                enabled: true,
                first_day_of_week: 1,
                id: id.to_string(),
                image: Default::default(),
                indeterminate: false,
                index: 0,
                layout: Default::default(),
                max: 100,
                parent: parent_id.to_string(),
                parent_type,
                pre_id: "".to_string(),
                pre_index: 0,
                pre_parent_type: 0,
                progress: 0,
                progress_style: "".to_string(),
                scale_x: 1.0,
                scale_y: 1.0,
                spinner_mode: SpinnerMode::Dropdown,
                text: Default::default(),
                translation_x: 0.0,
                translation_y: 0.0,
                r#type
            }
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

    impl Default for ImageConfig {
        fn default() -> Self {
            ImageConfig {
                rotate: 0,
                scale_type: image::ImageScaleType::Center
            }
        }
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

        pub gravity: Gravity, // 0 - Enum?
        pub layout_gravity: Gravity, // 0 - Enum?

        pub height: layout::Size, // -2: MATCH_PARENT
        pub width: layout::Size, // -1: WRAP_CONTENT

        pub margin_bottom: u32, // 0
        pub margin_left: u32, // 0
        pub margin_right: u32, // 0
        pub margin_top: u32, // 0

        pub padding_bottom: u32, // 8
        pub padding_left: u32, // 8
        pub padding_right: u32, // 8
        pub padding_top: u32, // 8

        pub orientation: layout::Orientation, // 1: vertical

        pub weight: u32, // 0
        pub weight_sum: u32, // 0
    }

    impl Default for LayoutConfig {
        fn default() -> Self {
            LayoutConfig {
                background_color: Color::from(16777215),
                gravity: Default::default(),
                height: layout::Size::MatchParent,
                width: layout::Size::WrapContent,
                layout_gravity: Default::default(),
                margin_bottom: 0,
                margin_left: 0,
                margin_right: 0,
                margin_top: 0,
                orientation: layout::Orientation::Vertical,
                padding_bottom: 8,
                padding_left: 8,
                padding_right: 8,
                padding_top: 8,
                weight: 0,
                weight_sum: 0,
            }
        }
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

        pub mod gravity {
            use serde::{Serialize, Deserialize};

            pub const NONE              : u8 = 0;
            pub const CENTER_HORIZONTAL : u8 = 1;
            pub const LEFT              : u8 = 3;
            pub const RIGHT             : u8 = 5;
            pub const CENTER_VERTICAL   : u8 = 16;
            pub const CENTER            : u8 = 17;
            pub const TOP               : u8 = 48;
            pub const BOTTOM            : u8 = 80;

            #[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
            pub struct Gravity(pub u8);

            impl Gravity {
                pub fn new(value: u8) -> Gravity { Gravity(value) }

                pub fn center_horizontal(&self) -> bool { self.0 & CENTER_HORIZONTAL == CENTER_HORIZONTAL }
                pub fn left(&self) -> bool              { self.0 & LEFT == LEFT }
                pub fn right(&self) -> bool             { self.0 & RIGHT == RIGHT }
                pub fn center_vertical(&self) -> bool   { self.0 & CENTER_VERTICAL == CENTER_VERTICAL }
                pub fn center(&self) -> bool            { self.0 & CENTER == CENTER }
                pub fn top(&self) -> bool               { self.0 & TOP == TOP }
                pub fn bottom(&self) -> bool            { self.0 & BOTTOM == BOTTOM }
            }

            impl Default for Gravity {
                fn default() -> Self {
                    Gravity(NONE)
                }
            }
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

        #[serde(with = "bool_to_one_zero")]
        pub single_line: bool, // (int) 0
        pub text: String, // ""
        pub text_color: Color, // -16777216
        pub text_font: String, // "default_font",
        pub text_size: u32, // 12,
        pub text_type: text::TextType, // 0: Normal
    }

    impl Default for TextConfig {
        fn default() -> Self {
            TextConfig {
                hint: "".to_string(),
                hint_color: Color::from(-10453621_i32 as u32),
                ime_option: text::ImeOption::Normal,
                input_type: text::InputType::Text,
                line: 0,
                single_line: false,
                text: "".to_string(),
                text_color: Color::from(-16777216_i32 as u32),
                text_font: "default_font".to_string(),
                text_size: 12,
                text_type: text::TextType::Normal,
            }
        }
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