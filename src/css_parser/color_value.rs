use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ColorValue {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8
}

impl Display for ColorValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a)
    }
}

#[cfg(test)]
mod tests {
    use crate::css_parser::color_value::ColorValue;

    #[test]
    fn test_display() {
        let color_value = ColorValue {
            r: 255,
            g: 99,
            b: 71,
            a: 1
        };

        assert_eq!("rgba(255, 99, 71, 1)", format!("{}", color_value));
    }
}