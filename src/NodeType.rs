use crate::ElementData::ElementData;

pub enum NodeType {
    Text(String),
    Element(ElementData)
}