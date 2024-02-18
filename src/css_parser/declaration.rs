use std::fmt::{Display, Formatter};
use crate::css_parser::declaration_value::DeclarationValue;

pub struct Declaration {
    pub(crate) name: String,
    pub(crate) value: DeclarationValue
}

impl Display for Declaration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

#[cfg(test)]
mod tests {
    use crate::css_parser::color_value::ColorValue;
    use crate::css_parser::declaration::Declaration;
    use crate::css_parser::declaration_value::DeclarationValue;
    use crate::css_parser::length_unit::LengthUnit;

    #[test]
    fn test_display() {
        let keyword = Declaration {
            name: String::from("display"),
            value: DeclarationValue::Keyword(String::from("none"))
        };

        let length = Declaration {
            name: String::from("margin-bottom"),
            value: DeclarationValue::Length(20.2, LengthUnit::Px)
        };

        let color = Declaration {
            name: String::from("color"),
            value: DeclarationValue::Color(ColorValue{r: 255, g: 99, b: 71, a: 1})
        };

        assert_eq!("display: none", format!("{}", keyword));
        assert_eq!("margin-bottom: 20.2px", format!("{}", length));
        assert_eq!("color: #rgba(255, 99, 71, 1)", format!("{}", color));
    }
}