use std::collections::{HashMap, HashSet};
use std::fmt::{Display};

pub struct ElementData {
    pub tag_name: String,
    pub(crate) attributes: HashMap<String, String>
}

impl ElementData {
    pub fn new(tag_name: String, attributes: HashMap<String, String>) -> Self {
        Self {
            tag_name,
            attributes
        }
    }

    pub fn get_id(&self) -> Option<&String> {
        return self.attributes.get("id");
    }

    pub fn get_classes(&self) -> HashSet<&str> {
        match self.attributes.get("class") {
            Some(classes) => classes.split(" ").collect(),
            _ => HashSet::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::html_parser::element_data::ElementData;

    #[test]
    fn test_get_classes() {
        let element_data = ElementData {
            tag_name: String::from("div"),
            attributes: HashMap::from(
                [
                    (String::from("class"), String::from("class_one class_two class_three")),
                    (String::from("name"), String::from("abc"))
                ]
            )
        };

        assert_eq!(3, element_data.get_classes().len());
        assert!(element_data.get_classes().contains("class_one"));
        assert!(element_data.get_classes().contains("class_two"));
        assert!(element_data.get_classes().contains("class_three"));
    }
}