use std::fmt::{Display, Formatter};

pub struct Selector {
    pub(crate) tag_name: Option<String>,
    pub(crate) id: Option<String>,
    pub(crate) classes: Vec<String>
}

impl Selector {
    pub fn get_specificity(&self) -> (usize, usize, usize) {
        return (self.id.iter().count(), self.classes.len(), self.tag_name.iter().count());
    }
}

impl Display for Selector {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "{}{}{}",
               if let Some(tag_name) = &self.tag_name {
                   format!("{}", tag_name)
               } else {
                   String::from("")
               },
               if let Some(id) = &self.id {
                   format!("#{}", id)
               } else {
                   String::from("")
               },
               self.classes.iter().fold(String::from(""), |mut acc, x| {
                   acc.push_str(".");
                   acc.push_str(x.as_str());
                   return acc;
               })
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::css_parser::selector::Selector;

    #[test]
    fn test_display() {
        let selector = Selector {
            tag_name: Some(String::from("div")),
            id: Some(String::from("answer")),
            classes: vec![String::from("note"), String::from("button"), String::from("topbar")]
        };

        println!("{}", selector);
    }
}