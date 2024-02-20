use std::collections::HashMap;
use crate::html_parser::node::Node as DomNode;
use crate::css_parser::declaration_value::DeclarationValue as CssDeclarationValue;

pub struct StyledNode<'a> {
    pub(crate) dom_node: &'a DomNode,
    pub(crate) css_properties: HashMap<String, CssDeclarationValue>,
    pub(crate) children: Vec<StyledNode<'a>>
}
