use crate::layout_tree_builder::box_type::BoxType;
use crate::layout_tree_builder::dimensions::Dimensions;

pub struct LayoutBox {
    dimensions: Dimensions,
    box_type: BoxType,
    children: Vec<LayoutBox>
}

impl LayoutBox {
    pub fn new(box_type: BoxType) -> LayoutBox {
        return LayoutBox {
            dimensions: Default::default(),
            box_type,
            children: Vec::new()
        }
    }
}