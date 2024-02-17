use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use crate::element_data::ElementData;
use crate::node_type::NodeType;

pub struct Node {
    children: Vec<Node>,
    node_type: NodeType
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