use std::fmt::{Display, Formatter, write};
use crate::element_data::ElementData;

pub enum NodeType {
    Text(String),
    Element(ElementData)
}