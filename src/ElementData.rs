use std::collections::HashMap;
use std::fmt::{Display, Formatter};

pub struct ElementData {
    tag_name: String,
    attributes: HashMap<String, String>
}

impl ElementData {
    pub fn new(tag_name: String, attributes: HashMap<String, String>) -> Self {
        Self {
            tag_name,
            attributes
        }
    }
}