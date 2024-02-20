use std::fmt::{Display, Formatter};
use crate::css_parser::color_value::ColorValue;
use crate::css_parser::length_unit::LengthUnit;

#[derive(Debug, Clone)]
pub enum DeclarationValue {
    Keyword(String),
    Length(f32, LengthUnit),
    Color(ColorValue)
}

impl Display for DeclarationValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DeclarationValue::Keyword(string) => write!(f, "{}", string),
            DeclarationValue::Length(number, unit) => write!(f, "{}{}", number, unit),
            DeclarationValue::Color(color_value) => write!(f, "#{}", color_value)
        }
    }
}



#[cfg(test)]
mod tests {
    use crate::css_parser::color_value::ColorValue;
    use crate::css_parser::declaration_value::DeclarationValue;
    use crate::css_parser::length_unit::LengthUnit;

    #[test]
    fn test_display() {
        let keyword = DeclarationValue::Keyword(String::from("none"));
        let length = DeclarationValue::Length(13.1, LengthUnit::Px);
        let color = DeclarationValue::Color(ColorValue{r: 255, g: 99, b: 71, a: 1});

        assert_eq!("none", format!("{}", keyword));
        assert_eq!("13.1px", format!("{}", length));
        assert_eq!("#rgba(255, 99, 71, 1)", format!("{}", color));
    }
}