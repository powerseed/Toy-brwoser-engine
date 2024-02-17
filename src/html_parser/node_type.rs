use std::fmt::{Display};
use crate::html_parser::element_data::ElementData;

pub enum NodeType {
    Text(String),
    Element(ElementData)
}