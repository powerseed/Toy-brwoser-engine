use std::fmt::{Display, Formatter};
use crate::css_parser::declaration::Declaration;
use crate::css_parser::selector::Selector;

pub struct Rule {
    pub(crate) selectors: Vec<Selector>,
    pub(crate) declarations: Vec<Declaration>
}

impl Display for Rule {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let selectors_string = self.selectors.iter().fold(String::from(""), |mut acc, x| {
            acc.push_str(format!("{}", x).as_str());
            acc.push_str(", ");
            return acc;
        });
        let selectors_string = selectors_string.split_at(selectors_string.len() - 2).0;

        write!(f,
               "{} {{ {}}}",
               selectors_string,
               self.declarations.iter().fold(String::from(""), |mut acc, x| {
                   acc.push_str(format!("{}", x).as_str());
                   acc.push_str("; ");
                   return acc;
               })
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::css_parser::color_value::ColorValue;
    use crate::css_parser::declaration::Declaration;
    use crate::css_parser::declaration_value::DeclarationValue;
    use crate::css_parser::length_unit::LengthUnit;
    use crate::css_parser::rule::Rule;
    use crate::css_parser::selector::Selector;

    #[test]
    fn test_parse() {
        let selectors = vec![
            Selector {
                tag_name: Some(String::from("h1")),
                id: None,
                classes: vec![]
            },
            Selector {
                tag_name: Some(String::from("h2")),
                id: None,
                classes: vec![]
            },
            Selector {
                tag_name: Some(String::from("h3")),
                id: None,
                classes: vec![]
            },
        ];

        let declarations = vec![
            Declaration {
                name: String::from("display"),
                value: DeclarationValue::Keyword(String::from("none"))
            },
            Declaration {
                name: String::from("margin-bottom"),
                value: DeclarationValue::Length(20.2, LengthUnit::Px)
            },
            Declaration {
                name: String::from("color"),
                value: DeclarationValue::Color(ColorValue{r: 255, g: 99, b: 71, a: 1})
            }
        ];

        let rule = Rule {
            selectors,
            declarations
        };

        assert_eq!("h1, h2, h3,  {display: none; margin-bottom: 20.2px; color: #rgba(255, 99, 71, 1); }", format!("{}", rule));
    }
}