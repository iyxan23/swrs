use crate::LinkedHashMap;
use crate::error::{SWRSError, SWRSResult};
use models::AndroidView;
use crate::parser::Parsable;

#[derive(Debug, Clone, PartialEq)]
pub struct View {
    /// All the layouts contained in this View file, it can be either a screen layout or a
    /// customview
    pub layouts: LinkedHashMap<String, Layout>,

    /// All the FABs contained in this view file
    pub fabs: LinkedHashMap<String, AndroidView>,
}

impl Parsable for View {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> {
        let mut lines = decrypted_content.split("\n");

        let mut layouts = LinkedHashMap::<String, Layout>::new();
        let mut fabs = LinkedHashMap::<String, AndroidView>::new();

        while let Some(line) = lines.next() {
            if !line.starts_with("@") { break; }

            let (screen_name, container_type) =
                &line[1..]
                    .split_once(".")
                    .ok_or_else(||SWRSError::ParseError(
                        "Cannot separate header of a screen into screen name & container type"
                            .to_string()
                    ))?;

            if *container_type == "xml" {
                let screen = Layout::parse_iter(&mut lines)
                    .map_err(|e|SWRSError::ParseError(format!(
                        "Error whilst trying to parse screen named {}: {}",
                        screen_name, e
                    )))?;

                layouts.insert(screen_name.to_string(), screen);

            } else if *container_type == "xml_fab" {
                let fab_view =
                    AndroidView::parse(
                        lines.next()
                            .ok_or_else(||SWRSError::ParseError(format!(
                                "EOF whilst trying to parse the fab view of {}",
                                screen_name
                            )))?
                    )?;

                fabs.insert(screen_name.to_string(), fab_view);
            }
        }

        Ok(View { layouts, fabs })
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        Ok(format!(
            "{}\n\n{}",
            self.layouts
                .iter()
                .try_fold(String::new(), |acc, i| {
                    Ok(format!("{}@{}.xml\n{}\n\n", acc, i.0, i.1.reconstruct()?))
                })?
                .trim(),
            self.fabs
                .iter()
                .try_fold(String::new(), |acc, i| {
                    Ok(format!("{}@{}.xml_fab\n{}\n\n", acc, i.0, i.1.reconstruct()?))
                })?
                .trim()
        ))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Layout(pub Vec<AndroidView>);

impl Layout {
    /// Parses an iterator that iterates over newlines
    ///
    /// Must skip the header part
    pub fn parse_iter<'a>(newline_iter: &mut impl Iterator<Item=&'a str>) -> SWRSResult<Self> {
        Ok(Layout(
            newline_iter
                .by_ref()
                .take_while(|i|*i != "")
                .map(AndroidView::parse)
                .collect::<SWRSResult<Vec<AndroidView>>>()?
        ))
    }
}

impl Parsable for Layout {
    fn parse(decrypted_content: &str) -> SWRSResult<Self> {
        Layout::parse_iter(&mut decrypted_content.split("\n"))
    }

    fn reconstruct(&self) -> SWRSResult<String> {
        Ok(
            self.0
                .iter()
                .try_fold(String::new(), |acc, i|
                    Ok(format!("{}\n{}", acc, i.reconstruct()?))
                )?
                .trim()
                .to_string()
        )
    }
}

pub mod models {
    use serde_repr::{Deserialize_repr, Serialize_repr};
    use serde::{Deserialize, Serialize};
    use crate::color::Color;
    use crate::error::{SWRSError, SWRSResult};
    use crate::parser::Parsable;
    use crate::parser::serde_util::{bool_to_one_zero, bool_to_str};
    use crate::parser::view::models::layout::gravity::Gravity;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
        pub divider_height: u32, // 0

        #[serde(with = "bool_to_one_zero")]
        pub enabled: bool, // (int) 0

        /// Sets the first day of a week for a calendar
        /// (<https://developer.android.com/reference/android/widget/CalendarView#setFirstDayOfWeek(int)>)
        pub first_day_of_week: u8, // 1
        pub id: String, // "something1"
        pub image: ImageConfig,

        #[serde(with = "bool_to_str")]
        pub indeterminate: bool, // (str) "false"
        pub index: u32, // 0
        pub layout: LayoutConfig,
        pub max: u32, // 100

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)] // <- this value is not present in fab views
        pub parent: Option<String>, // "something1"
        pub parent_type: i8, // 0 - note: can be -1 for some reason ¯\_(ツ)_/¯

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)] // <- this value is not present in fab views
        pub pre_id: Option<String>, // ""
        pub pre_index: i32, // 0 - note: can be -1 for some reason ¯\_(ツ)_/¯

        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub pre_parent: Option<String>,
        pub pre_parent_type: i8, // 0 - note: can be -1 for some reason ¯\_(ツ)_/¯
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
                parent: Some(parent_id.to_string()),
                parent_type,
                pre_id: Some("".to_string()),
                pre_index: 0,
                pre_parent: None,
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

    impl Parsable for AndroidView {
        fn parse(decrypted_content: &str) -> SWRSResult<Self> {
            serde_json::from_str(decrypted_content)
                .map_err(|e|SWRSError::ParseError(e.to_string()))
        }

        fn reconstruct(&self) -> SWRSResult<String> {
            serde_json::to_string(self)
                .map_err(|e|SWRSError::ReconstructionError(e.to_string()))
        }
    }

    #[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
    #[repr(u8)]
    pub enum ChoiceMode {
        None = 0,
        Single = 1,
        Multi = 2,
    }

    #[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
    #[repr(u8)]
    pub enum SpinnerMode {
        Dialog = 0,
        Dropdown = 1,
    }

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct ImageConfig {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(default)]
        pub res_name: Option<String>,
        pub rotate: i16, // 0
        pub scale_type: image::ImageScaleType, // CENTER
    }

    impl Default for ImageConfig {
        fn default() -> Self {
            ImageConfig {
                res_name: None,
                rotate: 0,
                scale_type: image::ImageScaleType::Center
            }
        }
    }

    pub mod image {
        use serde::{Serialize, Deserialize};

        #[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
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

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    #[serde(rename_all = "camelCase")]
    pub struct LayoutConfig {
        pub background_color: Color, // 16777215,

        pub gravity: Gravity, // 0 - Enum?

        pub height: layout::Size, // -2: MATCH_PARENT

        pub layout_gravity: Gravity, // 0 - Enum?

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

        #[derive(Debug, Clone, Copy, Eq, PartialEq)]
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

        #[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
        #[repr(i8)]
        pub enum Orientation {
            Vertical = 1,
            Unspecified = 0,
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

            #[derive(Debug, Clone, Copy, Serialize, Deserialize, Eq, PartialEq)]
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

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

        #[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
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

        #[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
        #[repr(u16)]
        pub enum InputType {
            NumberDecimal = 8194,
            NumberSigned = 4098,
            NumberSignedDecimal = 12290,
            Password = 129,
            Phone = 3,
            Text = 1,
        }

        #[derive(Debug, Clone, Copy, Serialize_repr, Deserialize_repr, Eq, PartialEq)]
        #[repr(u8)]
        pub enum TextType {
            Normal = 0,
            Bold = 1,
            Italic = 2,
            BoldItalic = 3,
        }
    }
}