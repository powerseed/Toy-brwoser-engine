use std::collections::HashSet;
use crate::css_parser::selector::Selector;
use crate::html_parser::element_data::ElementData;

pub mod styled_node;

pub fn are_selector_and_element_matched(selector: &Selector, element: &ElementData) -> bool {
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

pub fn check_if_tags_matched(tag_in_selector: &Option<String>, tag_in_element: &String) -> bool {
    return match tag_in_selector {
        // When a selector asks for a tag specifically, the element must have it.
        Some(tag_in_selector) => {
            return tag_in_selector == tag_in_element
        },
        // When a selector doesn't ask for a tag, keep checking other conditions.
        _ => true
    };
}

pub fn check_if_ids_matched(id_in_selector: &Option<String>, id_in_element: &Option<&String>) -> bool {
    return match id_in_selector {
        Some(id_in_selector) => {
            return Some(id_in_selector) == *id_in_element
        },
        _ => true
    };
}

pub fn check_if_classes_matched(classes_in_selector: &Vec<String>, classes_in_element: HashSet<&str>) -> bool {
    for class in classes_in_selector {
        if !classes_in_element.contains(class.as_str()) {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::style_tree_builder::{check_if_classes_matched, check_if_ids_matched, check_if_tags_matched};

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
}