use crate::css_parser::color_value::ColorValue;
use crate::css_parser::length_unit::LengthUnit;

pub enum DeclarationValue {
    Keyword(String),
    Length(f32, LengthUnit),
    Color(ColorValue)
}