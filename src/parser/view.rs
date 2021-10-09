use std::collections::HashMap;
use json::JsonValue;
use crate::error::{SWRSResult, SWRSError, ParseError};
use crate::parser::project::Color;

#[derive(Debug, PartialEq, Eq)]
pub struct ProjectView {
    pub activities: Vec<Screen>,
    pub custom_views: Vec<Screen>,
}

impl ProjectView {
    pub fn parse<S: AsRef<str>>(file: S) -> SWRSResult<ProjectView> {
        let file = file.as_ref();
        //                 screen name   screen
        let mut screens: HashMap<String, Screen> = HashMap::new();

        let mut iterator = file.split("\n");

        loop {
            let line = iterator.next();
            if line.is_none() { break }
            let line = line.unwrap();

            if line.starts_with("@") && line.ends_with(".xml") {
                // this is a screen
                let screen_name = &line[1..line.len() - 4];
                if !screens.contains_key(screen_name) {
                    screens.insert(screen_name.to_string(), Screen::new_empty(screen_name.to_string()))
                } else {
                    // TODO: warning: there are multiple screens with the same name
                    unreachable!();
                }
            } else if line.starts_with("@") && line.ends_with(".xml_fab") {
                let screen_name = &line[1..line.len() - 8];
                let fab_line = iterator.next();
                if fab_line.is_none() {
                    return Err(
                        SWRSError::ParseError(
                            ParseError::new(
                                "Failed parsing view",
                                &*format!("EOF while trying to read a FAB for the screen {}", screen_name)
                            )
                        )
                    );
                }

                let fab_line = fab_line.unwrap();
                let fab_view = View::parse(fab_line.to_string())?;

                if screens.contains_key(&screen_name) {
                    screens
                        .get_mut(screen_name)
                        .unwrap()
                        .fab_view = Option::Some(fab_view);
                } else {
                    screens
                        .insert(
                            screen_name.to_string(),
                            Screen::new_empty_with_fab(screen_name.to_string(), fab_view)
                        )
                }
            }
        }

        Ok(ProjectView { activities, custom_views })
    }
}

/// This struct represents a layout inside a View (`ProjectView`)
///
/// A layout multiple views and a single fab view (for unknown reason) even though compat is disabled
///
/// A sample of a Layout named "main":
/// ```none
/// @main.xml
/// {"adSize":"","adUnitId":"", ... "type":6} <-|- These are Views contained in the layout
/// {"adSize":"","adUnitId":"", ... "type":1} <-|
/// {"adSize":"","adUnitId":"", ... "type":3} <-|
/// ...
///
/// @main.xml_fab
/// {"adSize":"","adUnitId":"", ... "type":16} <- This is that one weird FAB view where the type is always 16
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct Screen {
    pub name: String,
    pub views: Vec<View>,
    pub fab_view: Option<View>,
}

impl Screen {
    pub fn new_empty(name: String) -> Self {
        Screen {
            name,
            views: Vec::new(),
            fab_view: None,
        }
    }

    pub fn new_empty_with_fab(name: String, fab_view: View) -> Self {
        Screen {
            name,
            views: Vec::new(),
            fab_view: Option::Some(fab_view),
        }
    }
}

/// This struct represents a single View that's contained in a layout.
///
/// This struct is absolutely enormous, It is planned to use Option instead for these fields.
///
/// Sample JSON form:
/// ```json
/// {
///     "adSize":"",
///     "adUnitId":"",
///     "alpha":1.0,
///     "checked":0,
///     "choiceMode":0,
///     "clickable":1,
///     "customView":"",
///     "dividerHeight":1,
///     "enabled":1,
///     "firstDayOfWeek":1,
///     "id":"white_chosen",
///     "image":{
///         "resName":"default_image",
///         "rotate":0,
///         "scaleType":"CENTER"
///     },
///     "indeterminate":"false",
///     "index":0,
///     "layout":{
///         "backgroundColor":-1,
///         "gravity":0,
///         "height":50,
///         "layoutGravity":0,
///         "marginBottom":0,
///         "marginLeft":0,
///         "marginRight":0,
///         "marginTop":0,
///         "orientation":0,
///         "paddingBottom":8,
///         "paddingLeft":8,
///         "paddingRight":8,
///         "paddingTop":8,
///         "weight":0,
///         "weightSum":0,
///         "width":50
///     },
///     "max":100,
///     "parent":"white",
///     "parentType":0,
///     "preId":"",
///     "preIndex":6,
///     "preParent":"linear11",
///     "preParentType":0,
///     "progress":0,
///     "progressStyle":"?android:progressBarStyle",
///     "scaleX":1.0,
///     "scaleY":1.0,
///     "spinnerMode":1,
///     "text":{
///         "hint":"",
///         "hintColor":-10453621,
///         "imeOption":0,
///         "inputType":1,
///         "line":0,
///         "singleLine":0,
///         "text":"",
///         "textColor":-16777216,
///         "textFont":"default_font",
///         "textSize":12,
///         "textType":0
///     },
///     "translationX":0.0,
///     "translationY":0.0,
///     "type":0
/// }
/// ```
pub struct View {
    // What this View is. TODO: Enum
    pub r#type: u8,

    // Essentials (Can be used in most Views)
    pub id: String,
    pub alpha: f32,
    pub layout: LayoutConfig,
    pub enabled: bool,
    pub clickable: bool,
    pub scale_x: f32,
    pub scale_y: f32,
    pub translation_x: f32,
    pub translation_y: f32,
    pub parent: Option<String>, // This is an Option<> because FAB views doesn't have parent for an unknown reason
    pub parent_type: u8, // TODO: Enum,

    // Views that have text
    pub text: TextConfig,

    // AdView-specific
    pub ad_size: String,
    pub ad_unit_id: String,

    // CheckBox-specific
    pub checked: bool,

    // Spinner-specific
    pub choice_mode: bool,

    // ListView and Spinner specific
    pub custom_view: String,

    // Spinner-specific
    pub spinner_mode: u8,

    // ListView-specific
    pub divider_height: u8,

    // ImageView-specific
    pub image: ImageConfig,

    // ProgressBar-specific
    pub indeterminate: bool,
    pub max: u8,
    pub progress: u8,
    pub progress_style: String,

    // Unknown
    pub first_day_of_week: u8,
    pub index: u16,
    pub pre_id: Option<String>, // This is an Option<> because FAB views doesn't have pre_id for an unknown reason
    pub pre_index: u16,
    pub pre_parent: String,
    pub pre_parent_type: u8,
}

#[derive(Debug)]
pub struct ImageConfig {
    pub res_name: String,
    pub rotate: u16,
    pub scale_type: String // TODO: Enum
}

#[derive(Debug)]
pub struct LayoutConfig {
    pub height: u16,
    pub width: u16,
    pub background_color: Color,
    pub gravity: u8, // TODO: Enum
    pub layout_gravity: u8, // TODO: Enum
    pub orientation: u8, // TODO: Enum
    pub margin_bottom: u16,
    pub margin_left: u16,
    pub margin_right: u16,
    pub margin_top: u16,
    pub padding_bottom: u16,
    pub padding_left: u16,
    pub padding_right: u16,
    pub padding_top: u16,
    pub weight: u16,
    pub weight_sum: u16,
}

#[derive(Debug)]
pub struct TextConfig {
    pub hint: String,
    pub hint_color: Color,
    pub ime_option: u8, // TODO: Enum
    pub input_type: u8, // TODO: Enum
    pub line: u8, // line count
    pub single_line: bool,
    pub text: String,
    pub text_color: Color,
    pub text_font: String,
    pub text_size: String,
    pub text_type: u8, // TODO: Enum
}

impl View {
    pub fn parse(view: String) -> SWRSResult<View> {
        let parsed = json::parse(&*view);
        if parsed.is_err() {
            return Err(
                SWRSError::ParseError(
                    ParseError::new(
                        "Failed parsing the view",
                        &*format!("Failed on parsing JSON of the received View. \nSnippet:\n\t{}", view)
                    )
                )
            );
        }

        let parsed = parsed.unwrap();

        // TODO: DO THIS
        Ok(
            View {
                r#type: 0,
                id: "".to_string(),
                alpha: 0.0,
                layout: LayoutConfig {
                    height: 0,
                    width: 0,
                    background_color: Default::default(),
                    gravity: 0,
                    layout_gravity: 0,
                    orientation: 0,
                    margin_bottom: 0,
                    margin_left: 0,
                    margin_right: 0,
                    margin_top: 0,
                    padding_bottom: 0,
                    padding_left: 0,
                    padding_right: 0,
                    padding_top: 0,
                    weight: 0,
                    weight_sum: 0
                },
                enabled: false,
                clickable: false,
                scale_x: 0.0,
                scale_y: 0.0,
                translation_x: 0.0,
                translation_y: 0.0,
                parent: "".to_string(),
                parent_type: 0,
                text: TextConfig {
                    hint: "".to_string(),
                    hint_color: Default::default(),
                    ime_option: 0,
                    input_type: 0,
                    line: 0,
                    single_line: false,
                    text: "".to_string(),
                    text_color: Default::default(),
                    text_font: "".to_string(),
                    text_size: "".to_string(),
                    text_type: 0
                },
                ad_size: "".to_string(),
                ad_unit_id: "".to_string(),
                checked: false,
                choice_mode: false,
                custom_view: "".to_string(),
                spinner_mode: 0,
                divider_height: 0,
                image: ImageConfig {
                    res_name: "".to_string(),
                    rotate: 0,
                    scale_type: "".to_string()
                },
                indeterminate: false,
                max: 0,
                progress: 0,
                progress_style: "".to_string(),
                first_day_of_week: 0,
                index: 0,
                pre_id: "".to_string(),
                pre_index: 0,
                pre_parent: "".to_string(),
                pre_parent_type: 0
            }
        )
    }
}