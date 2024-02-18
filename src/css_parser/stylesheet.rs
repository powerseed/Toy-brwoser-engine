use std::fmt::{Display, Formatter, Pointer, write};
use crate::css_parser::rule::Rule;

pub struct Stylesheet {
    pub(crate) rules: Vec<Rule>
}

impl Display for Stylesheet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut string = self.rules.iter().fold(String::from(""), |mut acc, x| {
            acc.push_str(format!("{}", x).as_str());
            acc.push_str("\n");
            return acc;
        });
        let string = string.split_at(string.len() - 1).0;

        write!(f, "{}", string)
    }
}