extern crate swrs;

use swrs::color::Color;
use swrs::parser::view::models::image::ImageScaleType;
use swrs::parser::view::models::layout::gravity::Gravity;
use swrs::parser::view::models::layout::{gravity, Orientation, Size};
use swrs::parser::view::models::text::{ImeOption, InputType, TextType};
use swrs::parser::view::models::{
    AndroidView, ChoiceMode, ImageConfig, LayoutConfig, SpinnerMode, TextConfig,
};
use swrs::parser::view::{Screen, View};
use swrs::parser::Parsable;

fn main() {
    let view = r#"@main.xml
{"adSize":"","adUnitId":"","alpha":1.0,"checked":0,"choiceMode":0,"clickable":1,"customView":"","dividerHeight":1,"enabled":1,"firstDayOfWeek":1,"id":"vscroll1","image":{"rotate":0,"scaleType":"CENTER"},"indeterminate":"false","index":0,"layout":{"backgroundColor":16777215,"gravity":0,"height":-2,"layoutGravity":0,"marginBottom":8,"marginLeft":0,"marginRight":0,"marginTop":0,"orientation":-1,"paddingBottom":0,"paddingLeft":0,"paddingRight":0,"paddingTop":0,"weight":1,"weightSum":0,"width":-1},"max":100,"parent":"root","parentType":0,"preId":"vscroll1","preIndex":0,"preParentType":0,"progress":0,"progressStyle":"?android:progressBarStyle","scaleX":1.0,"scaleY":1.0,"spinnerMode":1,"text":{"hint":"","hintColor":-10453621,"imeOption":0,"inputType":1,"line":0,"singleLine":0,"text":"","textColor":-16777216,"textFont":"default_font","textSize":12,"textType":0},"translationX":0.0,"translationY":0.0,"type":12}

@main.xml_fab
{"adSize":"","adUnitId":"","alpha":1.0,"checked":0,"choiceMode":0,"clickable":1,"customView":"","dividerHeight":1,"enabled":1,"firstDayOfWeek":1,"id":"_fab","image":{"rotate":0,"scaleType":"CENTER"},"indeterminate":"false","index":0,"layout":{"backgroundColor":16777215,"gravity":0,"height":-2,"layoutGravity":85,"marginBottom":16,"marginLeft":16,"marginRight":16,"marginTop":16,"orientation":-1,"paddingBottom":0,"paddingLeft":0,"paddingRight":0,"paddingTop":0,"weight":0,"weightSum":0,"width":-2},"max":100,"parentType":-1,"preIndex":0,"preParentType":0,"progress":0,"progressStyle":"?android:progressBarStyle","scaleX":1.0,"scaleY":1.0,"spinnerMode":1,"text":{"hint":"","hintColor":-10453621,"imeOption":0,"inputType":1,"line":0,"singleLine":0,"text":"","textColor":-16777216,"textFont":"default_font","textSize":12,"textType":0},"translationX":0.0,"translationY":0.0,"type":16}"#;

    let expected = View {
        screens: vec![Screen {
            name: "main".to_string(),
            views: vec![
                AndroidView {
                    ad_size: "".to_string(),
                    ad_unit_id: "".to_string(),
                    alpha: 1.0,
                    checked: false,
                    choice_mode: ChoiceMode::None,
                    clickable: true,
                    custom_view: "".to_string(),
                    divider_height: 1,
                    enabled: true,
                    first_day_of_week: 1,
                    id: "vscroll1".to_string(),
                    image: ImageConfig {
                        rotate: 0,
                        scale_type: ImageScaleType::Center
                    },
                    indeterminate: false,
                    index: 0,
                    layout: LayoutConfig {
                        background_color: Color::from(0xffffff),
                        gravity: Gravity(gravity::NONE),
                        layout_gravity: Gravity(gravity::NONE),
                        height: Size::MatchParent,
                        width: Size::WrapContent,
                        margin_bottom: 8,
                        margin_left: 0,
                        margin_right: 0,
                        margin_top: 0,
                        padding_bottom: 0,
                        padding_left: 0,
                        padding_right: 0,
                        padding_top: 0,
                        orientation: Orientation::Horizontal,
                        weight: 1,
                        weight_sum: 0
                    },
                    max: 100,
                    parent: "root".to_string(),
                    parent_type: 0,
                    pre_id: "vscroll1".to_string(),
                    pre_index: 0,
                    pre_parent_type: 0,
                    progress: 0,
                    progress_style: "?android:progressBarStyle".to_string(),
                    scale_x: 1.0,
                    scale_y: 1.0,
                    spinner_mode: SpinnerMode::Dropdown,
                    text: TextConfig {
                        hint: "".to_string(),
                        hint_color: Color::from(0x607d8b),
                        ime_option: ImeOption::Normal,
                        input_type: InputType::Text,
                        line: 0,
                        single_line: false,
                        text: "".to_string(),
                        text_color: Color::from(0x000000),
                        text_font: "default_font".to_string(),
                        text_size: 12,
                        text_type: TextType::Normal
                    },
                    translation_x: 0.0,
                    translation_y: 0.0,
                    r#type: 12
                }
            ],
            fab_view: AndroidView {
                ad_size: "".to_string(),
                ad_unit_id: "".to_string(),
                alpha: 1.0,
                checked: false,
                choice_mode: ChoiceMode::None,
                clickable: true,
                custom_view: "".to_string(),
                divider_height: 1,
                enabled: true,
                first_day_of_week: 1,
                id: "_fab".to_string(),
                image: ImageConfig {
                    rotate: 0,
                    scale_type: ImageScaleType::Center
                },
                indeterminate: false,
                index: 0,
                layout: LayoutConfig {
                    background_color: Color::from(0xffffff),
                    gravity: Gravity(gravity::NONE),
                    layout_gravity: Gravity(gravity::BOTTOM | gravity::RIGHT),
                    height: Size::MatchParent,
                    width: Size::MatchParent,
                    margin_bottom: 16,
                    margin_left: 16,
                    margin_right: 16,
                    margin_top: 16,
                    padding_bottom: 0,
                    padding_left: 0,
                    padding_right: 0,
                    padding_top: 0,
                    orientation: Orientation::Horizontal,
                    weight: 0,
                    weight_sum: 0
                },
                max: 100,
                parent: "".to_string(),
                parent_type: -1,
                pre_id: "".to_string(),
                pre_index: 0,
                pre_parent_type: 0,
                progress: 0,
                progress_style: "?android:progressBarStyle".to_string(),
                scale_x: 1.0,
                scale_y: 1.0,
                spinner_mode: SpinnerMode::Dropdown,
                text: TextConfig {
                    hint: "".to_string(),
                    hint_color: Color::from(0x607d8b),
                    ime_option: ImeOption::Normal,
                    input_type: InputType::Text,
                    line: 0,
                    single_line: false,
                    text: "".to_string(),
                    text_color: Color::from(0x000000),
                    text_font: "default_font".to_string(),
                    text_size: 12,
                    text_type: TextType::Normal
                },
                translation_x: 0.0,
                translation_y: 0.0,
                r#type: 16
            }
        }],
    };

    let parsed_view = View::parse(view).unwrap();

    assert_eq!(expected, parsed_view);
    println!("{:?}", parsed_view);
}
