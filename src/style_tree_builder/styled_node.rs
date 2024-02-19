use crate::html_parser::node::Node as DomNode;
use crate::css_parser::declaration::Declaration as CssDeclaration;

pub struct StyledNode<'a> {
    dom_node: &'a DomNode,
    css_properties: Vec<CssDeclaration>,
    children: Vec<StyledNode<'a>>
}
