use std::collections::HashMap;
use crate::html_parser::node::Node as DomNode;
use crate::css_parser::declaration_value::DeclarationValue;
use crate::css_parser::declaration_value::DeclarationValue::Keyword;
use crate::layout_tree_builder::display_type::DisplayType;

pub struct StyledNode<'a> {
    pub(crate) dom_node: &'a DomNode,
    pub(crate) css_properties: HashMap<String, DeclarationValue>,
    pub(crate) children: Vec<StyledNode<'a>>
}

impl<'a> StyledNode<'a> {
    fn get_css_value_by_name(&self, css_name: String) -> Option<&DeclarationValue> {
        return self.css_properties.get(&css_name)
    }

    pub fn get_display_value(&self) -> DisplayType {
        let display_value = self.get_css_value_by_name(String::from("display"));

        match display_value {
            Some(Keyword(display_keyword)) => {
                match display_keyword.as_str() {
                    "block" => DisplayType::Block,
                    "inline" => DisplayType::Inline,
                    "none" => DisplayType::None,
                    _ => panic!("None-supported display type. ")
                }
            },
            _ => DisplayType::Inline
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::css_parser::declaration_value::DeclarationValue;
    use crate::html_parser::node::Node;
    use crate::html_parser::node_type::NodeType;
    use crate::layout_tree_builder::display_type::DisplayType;
    use crate::style_tree_builder::styled_node::StyledNode;

    #[test]
    fn test_get_css_value_by_name() {
        let mut styled_node = StyledNode {
            dom_node: &Node {
                children: Vec::new(),
                node_type: NodeType::Text(String::from("text"))
            },
            css_properties: HashMap::from([(String::from("display"), DeclarationValue::Keyword(String::from("none")))]),
            children: Vec::new()
        };
        assert_eq!("none", format!("{}", styled_node.get_css_value_by_name(String::from("display")).unwrap()));

        styled_node.css_properties = HashMap::new();
        assert_eq!(None, styled_node.get_css_value_by_name(String::from("display")));
    }

    #[test]
    fn test_get_display_value() {
        let mut styled_node = StyledNode {
            dom_node: &Node {
                children: Vec::new(),
                node_type: NodeType::Text(String::from("text"))
            },
            css_properties: HashMap::from([(String::from("display"), DeclarationValue::Keyword(String::from("none")))]),
            children: Vec::new()
        };
        assert_eq!(DisplayType::None, styled_node.get_display_value());

        styled_node.css_properties = HashMap::new();
        assert_eq!(DisplayType::Inline, styled_node.get_display_value());
    }
}
