use std::collections::HashMap;
use crate::error::{SWRSResult, SWRSError, ParseError};
use crate::parser::project::Color;

#[derive(Debug)]
pub struct ProjectView {
    pub screens: Vec<Screen>,
}

// TODO: instead of reading line by line, what about splitting the entire view by "\n@" so that it
//       would only split each screens with its name on the first line

impl ProjectView {
    pub fn parse<S: AsRef<str>>(view: S) -> SWRSResult<ProjectView> {
        let file = view.as_ref();
        //                 screen name   screen
        let mut screens: HashMap<String, Screen> = HashMap::new();
        let mut current_screen: Option<String> = None;

        let mut iterator = file.split("\n");

        // TODO: Move these screen parse code over to Screen
        loop {
            let line = iterator.next();
            if line.is_none() { break }
            let line = line.unwrap();

            if line.starts_with("@") && line.ends_with(".xml") {
                // this is a screen
                let screen_name = &line[1..line.len() - 4];
                if !screens.contains_key(screen_name) {
                    screens.insert(screen_name.to_string(), Screen::new_empty(screen_name.to_string()));
                    current_screen = Option::Some(screen_name.to_string())
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

                if screens.contains_key(screen_name) {
                    screens
                        .get_mut(screen_name)
                        .unwrap()
                        .fab_view = Option::Some(fab_view);
                } else {
                    screens
                        .insert(
                            screen_name.to_string(),
                            Screen::new_empty_with_fab(screen_name.to_string(), fab_view)
                        );
                }
            } else {
                // this is a view of a screen, probably
                let view = View::parse(line.to_string());

                // if we failed to parse the view, then just skip it, this might just be a blank space between another screen / fab
                if view.is_err() { continue }

                let view = view.unwrap();

                // push the view on the topmost screen
                screens
                    .get_mut(current_screen.clone().unwrap().as_str())
                    .unwrap()
                    .views
                    .push(view);
            }
        }

        Ok(ProjectView { screens: screens.drain().map(|s| s.1).collect() })
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
#[derive(Debug)]
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
#[derive(Debug)]
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
    pub parent_type: i8, // TODO: Enum,

    // Views that have text
    pub text: TextConfig,

    // AdView-specific
    pub ad_size: String,
    pub ad_unit_id: String,

    // CheckBox-specific
    pub checked: bool,

    // Spinner-specific
    pub choice_mode: u8, // TODO: Enum

    // ListView and Spinner specific
    pub custom_view: String,

    // Spinner-specific
    pub spinner_mode: u8,

    // ListView-specific
    pub divider_height: u16,

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
    pub pre_parent: Option<String>, // This is an Option<> because FAB views doesn't have this for an unknown reason
    pub pre_parent_type: u8,
}

#[derive(Debug)]
pub struct ImageConfig {
    pub res_name: Option<String>,
    pub rotate: u16,
    pub scale_type: String // TODO: Enum
}

#[derive(Debug)]
pub struct LayoutConfig {
    pub height: Size,
    pub width: Size,
    pub background_color: Color,
    pub gravity: u8, // TODO: Enum
    pub layout_gravity: u8, // TODO: Enum
    pub orientation: i8, // TODO: Enum
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
pub enum Size {
    MatchParent,
    WrapContent,
    Fixed(u16)
}

impl From<i16> for Size {
    fn from(num: i16) -> Self {
        if num == -1 {
            Size::WrapContent
        } else if num == -2 {
            Size::MatchParent
        } else if num > 0 {
            Size::Fixed(num as u16)
        } else {
            // weird, the num is negative but its not -1 or -2
            Size::Fixed(0)
        }
    }
}

#[derive(Debug)]
pub struct TextConfig {
    pub hint: String,
    pub hint_color: Color,
    pub ime_option: u8, // TODO: Enum
    pub input_type: u8, // TODO: Enum
    pub line: u16, // line count
    pub single_line: bool,
    pub text: String,
    pub text_color: Color,
    pub text_font: String,
    pub text_size: u16,
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

        Ok(
            View {
                r#type: parsed["type"].as_u8().unwrap(),
                id: parsed["id"].as_str().unwrap().to_string(),
                alpha: parsed["alpha"].as_f32().unwrap(),
                layout: {
                    let layout = &parsed["layout"];

                    LayoutConfig {
                        height: Size::from(layout["height"].as_i16().unwrap()),
                        width: Size::from(layout["width"].as_i16().unwrap()),
                        background_color: Color::from(layout["backgroundColor"].as_f32().unwrap() as i32 as u32),
                        gravity: layout["gravity"].as_u8().unwrap(),
                        layout_gravity: layout["layoutGravity"].as_u8().unwrap(),
                        orientation: layout["orientation"].as_i8().unwrap(),
                        margin_bottom: layout["marginBottom"].as_u16().unwrap(),
                        margin_left: layout["marginLeft"].as_u16().unwrap(),
                        margin_right: layout["marginRight"].as_u16().unwrap(),
                        margin_top: layout["marginTop"].as_u16().unwrap(),
                        padding_bottom: layout["paddingBottom"].as_u16().unwrap(),
                        padding_left: layout["paddingLeft"].as_u16().unwrap(),
                        padding_right: layout["paddingRight"].as_u16().unwrap(),
                        padding_top: layout["paddingTop"].as_u16().unwrap(),
                        weight: layout["weight"].as_u16().unwrap(),
                        weight_sum: layout["weightSum"].as_u16().unwrap(),
                    }
                },
                enabled: parsed["enabled"].as_u8().unwrap() == 1,
                clickable: parsed["clickable"].as_u8().unwrap() == 1,
                scale_x: parsed["scaleX"].as_f32().unwrap(),
                scale_y: parsed["scaleY"].as_f32().unwrap(),
                translation_x: parsed["translationX"].as_f32().unwrap(),
                translation_y: parsed["translationY"].as_f32().unwrap(),
                parent: parsed["parent"].as_str().map(|s| s.to_string()),
                parent_type: parsed["parentType"].as_i8().unwrap(),
                text: {
                    let text = &parsed["text"];

                    TextConfig {
                        hint: text["hint"].as_str().unwrap().to_string(),
                        hint_color: Color::from(text["hintColor"].as_f32().unwrap() as i32 as u32),
                        ime_option: text["imeOption"].as_u8().unwrap(),
                        input_type: text["inputType"].as_u8().unwrap(),
                        line: text["line"].as_u16().unwrap(),
                        single_line: text["singleLine"].as_u8().unwrap() == 1,
                        text: text["text"].as_str().unwrap().to_string(),
                        text_color: Color::from(text["textColor"].as_f32().unwrap() as i32 as u32),
                        text_font: text["textFont"].as_str().unwrap().to_string(),
                        text_size: text["textSize"].as_u16().unwrap(),
                        text_type: text["textType"].as_u8().unwrap(),
                    }
                },
                ad_size: parsed["adSize"].as_str().unwrap().to_string(),
                ad_unit_id: parsed["adUnitId"].as_str().unwrap().to_string(),
                checked: parsed["checked"].as_u8().unwrap() == 1,
                choice_mode: parsed["choiceMode"].as_u8().unwrap(),
                custom_view: parsed["customView"].as_str().unwrap().to_string(),
                spinner_mode: parsed["spinnerMode"].as_u8().unwrap(),
                divider_height: parsed["dividerHeight"].as_u16().unwrap(),
                image: {
                    let image = &parsed["image"];

                    ImageConfig {
                        res_name: image["resName"].as_str().map(|s| s.to_string()),
                        rotate: image["rotate"].as_u16().unwrap(),
                        scale_type: image["scaleType"].as_str().unwrap().to_string(),
                    }
                },
                indeterminate: parsed["indeterminate"].as_str().unwrap() == "true",
                max: parsed["max"].as_u8().unwrap(),
                progress: parsed["progress"].as_u8().unwrap(),
                progress_style: parsed["progressStyle"].as_str().unwrap().to_string(),
                first_day_of_week: parsed["firstDayOfWeek"].as_u8().unwrap(),
                index: parsed["index"].as_u16().unwrap(),
                pre_id: parsed["preId"].as_str().map(|s| s.to_string()),
                pre_index: parsed["preIndex"].as_u16().unwrap(),
                pre_parent: parsed["preParent"].as_str().map(|s| s.to_string()),
                pre_parent_type: parsed["preParentType"].as_u8().unwrap()
            }
        )
    }
}