use std::collections::HashMap;
use std::fmt::{Display};
use crate::html_parser::element_data::ElementData;
use crate::html_parser::node_type::NodeType;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType
}

impl Node {
    pub fn new_text(text: String) -> Self {
        Self {
            children: Vec::new(),
            node_type: NodeType::Text(text)
        }
    }

    pub fn new_element(tag_name: String, attributes: HashMap<String, String>, children: Vec<Node>) -> Self {
        Self {
            children,
            node_type: NodeType::Element(
                ElementData::new(tag_name, attributes)
            )
        }
    }
}