use std::collections::{HashMap, HashSet};
use crate::css_parser::declaration_value::DeclarationValue;
use crate::css_parser::rule::Rule;
use crate::css_parser::selector::Selector;
use crate::css_parser::stylesheet::Stylesheet;
use crate::html_parser::element_data::ElementData;
use crate::html_parser::node::Node;
use crate::html_parser::node_type::NodeType;
use crate::style_tree_builder::styled_node::StyledNode;

pub mod styled_node;

fn check_if_selector_and_element_match(selector: &Selector, element: &ElementData) -> bool {
    if !check_if_tags_matched(&selector.tag_name, &element.tag_name) {
        return false;
    }

    if !check_if_ids_matched(&selector.id, &element.get_id()) {
        return false;
    }

    if !check_if_classes_matched(&selector.classes, element.get_classes()) {
        return false;
    }

    return true;
}

fn check_if_tags_matched(tag_in_selector: &Option<String>, tag_in_element: &String) -> bool {
    return match tag_in_selector {
        // When a selector asks for a tag specifically, the element must have it.
        Some(tag_in_selector) => {
            return tag_in_selector == tag_in_element
        },
        // When a selector doesn't ask for a tag, keep checking other conditions.
        _ => true
    };
}

fn check_if_ids_matched(id_in_selector: &Option<String>, id_in_element: &Option<&String>) -> bool {
    return match id_in_selector {
        Some(id_in_selector) => {
            return Some(id_in_selector) == *id_in_element
        },
        _ => true
    };
}

fn check_if_classes_matched(classes_in_selector: &Vec<String>, classes_in_element: HashSet<&str>) -> bool {
    for class in classes_in_selector {
        if !classes_in_element.contains(class.as_str()) {
            return false;
        }
    }

    return true;
}

fn check_if_rule_and_element_match<'a>(rule: &'a Rule, element: &ElementData) -> Option<((usize, usize, usize), &'a Rule)> {
    rule.selectors
        .iter()
        .find(|selector| check_if_selector_and_element_match(selector, element))
        .map(|selector| (selector.get_specificity(), rule))
}

fn match_rules_with_element<'a>(rules: &'a Vec<Rule>, element_data: &ElementData) -> Vec<((usize, usize, usize), &'a Rule)> {
    return rules
        .iter()
        .filter_map(|rule| check_if_rule_and_element_match(rule, element_data))
        .collect();
}

fn create_css_properties(stylesheet: &Stylesheet, element_data: &ElementData) -> HashMap<String, DeclarationValue> {
    let mut css_properties = HashMap::new();
    let mut rules = match_rules_with_element(&stylesheet.rules, &element_data);
    rules.sort_by(|(a, _), (b, _)| a.cmp(b));

    for (_, rule) in rules {
        for declaration in &rule.declarations {
            css_properties.insert(declaration.name.clone(), declaration.value.clone());
        }
    }

    return css_properties;
}

pub fn create_styled_node<'a>(dom_node: &'a Node, stylesheet: &Stylesheet) -> StyledNode<'a> {
    let css_properties = match &dom_node.node_type {
        NodeType::Text(_) => HashMap::new(),
        NodeType::Element(element) => create_css_properties(stylesheet, &element)
    };

    return StyledNode {
        dom_node,
        css_properties,
        children: dom_node.children.iter().map(|child_node| create_styled_node(child_node, stylesheet)).collect()
    };
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use std::ptr;
    use crate::css_parser::declaration::Declaration;
    use crate::css_parser::declaration_value::DeclarationValue;
    use crate::css_parser::rule::Rule;
    use crate::css_parser::selector::Selector;
    use crate::css_parser::stylesheet::Stylesheet;
    use crate::html_parser::element_data::ElementData;
    use crate::style_tree_builder::{check_if_classes_matched, check_if_ids_matched, check_if_rule_and_element_match, check_if_tags_matched, create_css_properties, match_rules_with_element};

    #[test]
    fn test_check_if_tags_matched() {
        let tag_in_selector = Some(String::from("div"));
        let tag_in_element = String::from("div");

        assert!(check_if_tags_matched(&tag_in_selector, &tag_in_element));

        let tag_in_element = String::from("dive");
        assert!(!check_if_tags_matched(&tag_in_selector, &tag_in_element));

        let tag_in_selector = None;
        assert!(check_if_tags_matched(&tag_in_selector, &tag_in_element));
    }

    #[test]
    fn test_check_if_ids_matched() {
        let string_abc_1 = String::from("abc");
        let string_abc_2 = String::from("abc");
        let id_in_selector = Some(string_abc_1);
        let id_in_element = Some(&string_abc_2);

        assert!(check_if_ids_matched(&id_in_selector, &id_in_element));

        let string_abcd = String::from("abcd");
        let id_in_element = Some(&string_abcd);
        assert!(!check_if_ids_matched(&id_in_selector, &id_in_element));

        let id_in_element = None;
        assert!(!check_if_ids_matched(&id_in_selector, &id_in_element));

        let id_in_selector = None;
        assert!(check_if_ids_matched(&id_in_selector, &id_in_element));

        let id_in_element = Some(&string_abc_2);
        assert!(check_if_ids_matched(&id_in_selector, &id_in_element));
    }

    #[test]
    fn test_check_if_classes_matched() {
        let classes_in_selector = vec![String::from("class_one"), String::from("class_two"), String::from("class_three")];
        let classes_in_element = HashSet::from(["class_one", "class_two", "class_three"]);
        assert!(check_if_classes_matched(&classes_in_selector, classes_in_element));

        let classes_in_element = HashSet::from(["class_one", "class_three"]);
        assert!(!check_if_classes_matched(&classes_in_selector, classes_in_element));

        let classes_in_selector = vec![];
        let classes_in_element = HashSet::from(["class_one", "class_two", "class_three"]);
        assert!(check_if_classes_matched(&classes_in_selector, classes_in_element));
    }

    #[test]
    fn test_match_rules_with_element() {
        let selectors_one = vec![
            Selector {
                tag_name: Some(String::from("div")),
                id: None,
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: Some(String::from("abc")),
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: None,
                classes: vec![String::from("class_one"), String::from("class_two")]
            },
        ];

        let selectors_two = vec![
            Selector {
                tag_name: Some(String::from("div")),
                id: None,
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: Some(String::from("abc")),
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: None,
                classes: vec![String::from("class_one"), String::from("class_two"), String::from("class_three")]
            },
        ];

        let rules = vec![
            Rule {
                selectors: selectors_one,
                declarations: Vec::new()
            },
            Rule {
                selectors: selectors_two,
                declarations: Vec::new()
            },
        ];

        let element_matches = ElementData {
            tag_name: String::from("p"),
            attributes: HashMap::from([(String::from("class"), String::from("class_one class_two"))])
        };

        let result = match_rules_with_element(&rules, &element_matches);
        assert_eq!(1, result.len());
    }

    #[test]
    fn test_check_if_rule_and_element_match() {
        let selectors = vec![
            Selector {
                tag_name: Some(String::from("div")),
                id: None,
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: Some(String::from("abc")),
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: None,
                classes: vec![String::from("class_one"), String::from("class_two")]
            },
        ];

        let rule = Rule {
            selectors,
            declarations: Vec::new()
        };

        let element_matches = ElementData {
            tag_name: String::from("p"),
            attributes: HashMap::from([(String::from("class"), String::from("class_one class_two"))])
        };

        let result = check_if_rule_and_element_match(&rule, &element_matches).unwrap();
        assert_eq!((0, 2, 0), result.0);
        assert!(ptr::eq(&rule, result.1));

        let element_not_match = ElementData {
            tag_name: String::from("p"),
            attributes: HashMap::new()
        };
        let result = check_if_rule_and_element_match(&rule, &element_not_match);
        assert!(result.is_none());
    }

    #[test]
    fn test_create_css_properties() {
        let selectors_one = vec![
            Selector {
                tag_name: Some(String::from("div")),
                id: None,
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: Some(String::from("abc")),
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: None,
                classes: vec![String::from("class_one"), String::from("class_two")]
            },
        ];

        let selectors_two = vec![
            Selector {
                tag_name: Some(String::from("div")),
                id: None,
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: Some(String::from("abc")),
                classes: Vec::new()
            },
            Selector {
                tag_name: None,
                id: None,
                classes: vec![String::from("class_one"), String::from("class_two"), String::from("class_three")]
            },
        ];

        let rules = vec![
            Rule {
                selectors: selectors_one,
                declarations: vec![
                    Declaration {
                        name: String::from("display"),
                        value: DeclarationValue::Keyword(String::from("none"))
                    }
                ]
            },
            Rule {
                selectors: selectors_two,
                declarations: vec![
                    Declaration {
                        name: String::from("appearance"),
                        value: DeclarationValue::Keyword(String::from("auto"))
                    }
                ]
            },
        ];

        let stylesheet = Stylesheet {
            rules
        };

        let element_matches = ElementData {
            tag_name: String::from("p"),
            attributes: HashMap::from([(String::from("class"), String::from("class_one class_two"))])
        };

        let result = create_css_properties(&stylesheet, &element_matches);
        assert_eq!("{\"display\": Keyword(\"none\")}", format!("{:?}", result));
    }
}