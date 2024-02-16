use std::collections::HashMap;
use crate::ElementData::ElementData;
use crate::NodeType::NodeType;

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

    pub fn new_element(children: Vec<Node>, tag_name: String, attributes: HashMap<String, String>) -> Self {
        Self {
            children,
            node_type: NodeType::Element(
                ElementData::new(tag_name, attributes)
            )
        }
    }
}

