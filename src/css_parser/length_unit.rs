use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq)]
pub enum LengthUnit {
    Px
}

impl Display for LengthUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            LengthUnit::Px => write!(f, "px")
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_display() {
        let length_unit = crate::css_parser::length_unit::LengthUnit::Px;
        assert_eq!("px", format!("{}", length_unit));
    }
}