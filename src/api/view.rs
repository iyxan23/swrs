use crate::color::Color;
use crate::error::SWRSResult;
use crate::parser::view::models::layout;
use crate::parser::view::View as RawView;

/// A model that represents a single view
///
/// This struct contains fields that are common to all views, a field that holds an enum of every
/// (sketchware original) view types with its used fields (`view`) (if the view type isn't
/// recognized, it will be set as None) and another field that holds the raw view, just in case you
/// needed it.
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

    /// The raw view of this View. This may be used to access every fields of this view in its raw
    /// form. Changes made to this are NOT going to be accounted in the reconstruction of this view
    /// unfortunately.
    pub raw: RawView,
}

/// A struct that stores 4 `u32` values (top, right, bottom, and left). Used as a model of
/// padding and margin
pub struct SidesValue {
    pub top: u32,
    pub right: u32,
    pub bottom: u32,
    pub left: u32
}

/// An enum that contains every sketchware original view types and its necessary fields, any other
/// fields that aren't used in the specific view type will be neglected.
pub enum ViewType {

}