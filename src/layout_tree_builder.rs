use crate::layout_tree_builder::box_type::BoxType;
use crate::layout_tree_builder::display_type::DisplayType;
use crate::layout_tree_builder::layout_box::LayoutBox;
use crate::style_tree_builder::styled_node::StyledNode;

mod dimensions;
mod layout_box;
mod box_type;
pub(crate) mod display_type;

pub fn build_layout_tree<'a>(styled_node: &'a StyledNode) -> LayoutBox<'a> {
    let root_box_type = match styled_node.get_display_value() {
        DisplayType::Inline => BoxType::Inline,
        DisplayType::Block => BoxType::Block,
        DisplayType::None => panic!("Root StyledNode has display: none. ")
    };

    let mut root_layout_box = LayoutBox::new(root_box_type);
    root_layout_box.styled_node = Some(styled_node);

    for child in &styled_node.children {
        match child.get_display_value() {
            DisplayType::Inline => root_layout_box.get_inline_box().children.push(build_layout_tree(child)),
            DisplayType::Block => root_layout_box.children.push(build_layout_tree(child)),
            DisplayType::None => {}
        }
    }

    return root_layout_box;
}