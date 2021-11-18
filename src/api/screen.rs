use crate::api::code::Blocks;
use crate::api::layout::Layout;
use crate::api::view::View;

pub struct Screen {
    pub layout_name: String,
    pub java_name: String,

    pub layout: View,
    pub blocks: Blocks,
}