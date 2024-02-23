use crate::css_parser::color_value::ColorValue;
use crate::layout_tree_builder::dimensions::Rectangle;

pub enum DisplayCommand {
    SolidColor(ColorValue, Rectangle),
}