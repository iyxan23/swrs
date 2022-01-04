use crate::color::Color;
use crate::parser::view::models::{AndroidView, image, layout, SpinnerMode, text};
use crate::parser::view::Layout;
use crate::{SWRSError, SWRSResult};
use crate::parser::view::models::layout::Orientation;

/// A model that represents a single view
///
/// This struct contains fields that are common to all views, a field that holds an enum of every
/// (sketchware original) view types with its used fields (`view`) (if the view type isn't
/// recognized, it will be set as None) and another field that holds the raw view, just in case you
/// needed it.
#[derive(Debug, Clone, PartialEq)]
pub struct View {
    /// The id of this view, must be unique in the layout it belongs to; this is used to identify
    /// and find views in your java code
    pub id: String,

    /// The background color of this view
    pub background_color: Color,

    pub height: layout::Size,
    pub width: layout::Size,

    pub padding: SidesValue,
    pub margin: SidesValue,

    /// The weight of this view; Weight is a value that defines how much of the parent layout this
    /// view should occupy.
    ///
    /// For example, if a layout contains two views and both views has the weight of `1`, both views
    /// will get divided in half.
    pub weight: u32,

    /// The weight sum of this view; Weight sum is a value that defines the maximum sum of weight
    /// of all children of this layout.
    pub weight_sum: u32,

    /// The layout gravity of this view; Layout gravity is a value that defines the gravity of this
    /// view in the parent layout.
    pub layout_gravity: layout::gravity::Gravity,

    /// The view-type-specific fields are stored in this enum, will give out `None` if this view's
    /// type is not recognized.
    pub view: Option<ViewType>,

    /// The children of this view
    pub children: Vec<View>,

    /// The raw view of this View. This may be used to access every fields of this view in its raw
    /// form. Changes made to this are NOT going to be accounted in the reconstruction of this view
    /// unfortunately.
    pub raw: AndroidView,
}

impl View {
    pub fn find_id(&self, id: &str) -> Option<&View> {
        if self.id == id {
            Some(self)
        } else {
            // recurse on children
            self.children
                .iter()
                .find_map(|i| i.find_id(id))
        }
    }

    pub fn find_id_mut(&mut self, id: &str) -> Option<&mut View> {
        if self.id == id {
            Some(self)
        } else {
            // recurse on children
            self.children
                .iter_mut()
                .find_map(|i| i.find_id_mut(id))
        }
    }
}

impl TryFrom<AndroidView> for View {
    type Error = SWRSError;

    fn try_from(value: AndroidView) -> Result<Self, Self::Error> {
        Ok(View {
            id: value.id.clone(),
            background_color: value.layout.background_color,
            height: value.layout.height,
            width: value.layout.width,
            padding: SidesValue {
                top: value.layout.padding_top,
                right: value.layout.padding_right,
                bottom: value.layout.padding_bottom,
                left: value.layout.padding_left,
            },
            margin: SidesValue {
                top: value.layout.margin_top,
                right: value.layout.margin_right,
                bottom: value.layout.margin_bottom,
                left: value.layout.margin_left,
            },
            weight: value.layout.weight,
            weight_sum: value.layout.weight_sum,
            layout_gravity: value.layout.layout_gravity,
            // ignore the Err because the function only Err-s when the view type is unknown
            view: if let Ok(res) = ViewType::from_view(&value) { Some(res) } else { None },
            children: vec![],
            raw: value.clone()
        })
    }
}

/// A struct that stores 4 `u32` values (top, right, bottom, and left). Used as a model of
/// padding and margin
#[derive(Debug, Clone, PartialEq)]
pub struct SidesValue {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32
}

/// An enum that contains every sketchware original view types and its necessary fields, any other
/// fields that aren't used in the specific view type will be neglected.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ViewType {
    LinearLayout {
        orientation: layout::Orientation,
        gravity: layout::gravity::Gravity,
    },
    ScrollView {
        orientation: layout::Orientation,
        gravity: layout::gravity::Gravity,
    },
    Button {
        text: String,
        text_color: Color,
        text_size: u32,
        text_style: text::TextType,
    },
    TextView {
        text: String,
        text_color: Color,
        text_size: u32,
        single_line: bool,
        text_font: String,
        text_style: text::TextType,
        lines: u32,
    },
    EditText {
        text: String,
        text_color: Color,
        text_size: u32,
        single_line: bool,
        text_font: String,
        text_style: text::TextType,
        lines: u32,

        hint: String,
        hint_color: Color,
        ime_option: text::ImeOption,
        input_type: text::InputType,
    },
    ImageView {
        image_res_name: String,
        image_scale_type: image::ImageScaleType,
    },
    WebView,
    ProgressBar {
        max_progress: u32,
        progress: u32,
        indeterminate: bool,
        progress_style: String,
    },
    ListView {
        divider_height: u32,
        custom_view: String,
    },
    Spinner {
        spinner_mode: SpinnerMode,
    },
    CheckBox {
        checked: bool,

        text: String,
        text_color: Color,
        text_size: u32,
        text_font: String,
        text_style: text::TextType,
    },
    Switch {
        checked: bool,

        text: String,
        text_color: Color,
        text_size: u32,
        text_font: String,
        text_style: text::TextType,
    },
    SeekBar {
        max_progress: u32,
        progress: u32,
    },
    CalendarView {
        first_day_of_week: u8,
    },
    Fab {
        image_res_name: String,
    },
    AdView {
        adview_size: String,
    },
    MapView
}

impl ViewType {
    /// Converts an [`&AndroidView`] into [`ViewType`]
    pub fn from_view(android_view: &AndroidView) -> SWRSResult<Self> {
        // https://github.com/Iyxan23/sketchware-data/blob/main/data/view-types.md
        Ok(match android_view.r#type {
            0 => ViewType::LinearLayout {
                orientation: android_view.layout.orientation,
                gravity: android_view.layout.gravity,
            },
            2 => ViewType::ScrollView {
                orientation: Orientation::Horizontal,
                gravity: android_view.layout.gravity,
            },
            3 => ViewType::Button {
                text: android_view.text.text.clone(),
                text_color: android_view.text.text_color,
                text_size: android_view.text.text_size,
                text_style: android_view.text.text_type,
            },
            4 => ViewType::TextView {
                text: android_view.text.text.clone(),
                text_color: android_view.text.text_color,
                text_size: android_view.text.text_size,
                single_line: android_view.text.single_line,
                text_font: android_view.text.text_font.clone(),
                text_style: android_view.text.text_type,
                lines: android_view.text.line,
            },
            5 => ViewType::EditText {
                text: android_view.text.text.clone(),
                text_color: android_view.text.text_color,
                text_size: android_view.text.text_size,
                single_line: android_view.text.single_line,
                text_font: android_view.text.text_font.clone(),
                text_style: android_view.text.text_type,
                lines: android_view.text.line,
                hint: android_view.text.hint.clone(),
                hint_color: android_view.text.hint_color,
                ime_option: android_view.text.ime_option,
                input_type: android_view.text.input_type,
            },
            6 => ViewType::ImageView {
                image_res_name: android_view.image.res_name.as_ref()
                    .ok_or_else(||SWRSError::ParseError(format!(
                        "res_name is not present in the view id {} while the type is an ImageView",
                        android_view.id
                    )))?.clone(),
                image_scale_type: android_view.image.scale_type,
            },
            7 => ViewType::WebView,
            8 => ViewType::ProgressBar {
                max_progress: android_view.max,
                progress: android_view.progress,
                indeterminate: android_view.indeterminate,
                progress_style: android_view.progress_style.clone(),
            },
            9 => ViewType::ListView {
                divider_height: android_view.divider_height,
                custom_view: android_view.custom_view.clone(),
            },
            10 => ViewType::Spinner {
                spinner_mode: android_view.spinner_mode,
            },
            11 => ViewType::CheckBox {
                checked: android_view.checked,
                text: android_view.text.text.clone(),
                text_color: android_view.text.text_color,
                text_size: android_view.text.text_size,
                text_font: android_view.text.text_font.clone(),
                text_style: android_view.text.text_type,
            },
            12 => ViewType::ScrollView {
                orientation: Orientation::Vertical,
                gravity: android_view.layout.gravity,
            },
            13 => ViewType::Switch {
                checked: android_view.checked,
                text: android_view.text.text.clone(),
                text_color: android_view.text.text_color,
                text_size: android_view.text.text_size,
                text_font: android_view.text.text_font.clone(),
                text_style: android_view.text.text_type,
            },
            14 => ViewType::SeekBar {
                max_progress: android_view.max,
                progress: android_view.progress,
            },
            15 => ViewType::CalendarView { first_day_of_week: android_view.first_day_of_week },
            16 => ViewType::Fab {
                image_res_name: android_view.image.res_name.as_ref()
                    .ok_or_else(||SWRSError::ParseError(format!(
                        "res_name is not present in the view id {} while the type is a FAB",
                        android_view.id
                    )))?.clone(),
            },
            17 => ViewType::AdView { adview_size: android_view.ad_size.clone() },
            18 => ViewType::MapView,
            _ => Err(SWRSError::ParseError(format!(
                "Unknown view type: {}",
                android_view.r#type
            )))?
        })
    }
}

/// Converts a parser's raw [`Layout`] into [`View`]
pub fn raw_layout_to_view(screen_view: Layout) -> SWRSResult<View> {
    let mut root_view = Option::<View>::None;

    for view in screen_view.0 {
        if let Some(r) = &mut root_view {
            // get the parent id, find the parent on root view, and append the view to the parent
            let parent_id =
                view.parent.as_ref().ok_or_else(||SWRSError::ParseError(format!(
                    "View `{}` doesn't have a parent field",
                    r.id
                )))?;

            let parent = r.find_id_mut(&parent_id)
                .ok_or_else(||SWRSError::ParseError(format!(
                    "Couldn't find the parent of view `{}` - Parent id: `{}`",
                    view.id, parent_id
                )))?;

            parent.children.push(view.try_into()?);
        } else {
            // set the first view to be the root view
            root_view = Some(view.try_into()?);
            continue;
        }
    }

    root_view
        .ok_or_else(||SWRSError::ParseError("View pool is empty, and it shouldn't be".to_string()))
}