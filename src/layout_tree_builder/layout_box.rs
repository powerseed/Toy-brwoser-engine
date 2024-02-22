use crate::layout_tree_builder::box_type::BoxType;
use crate::layout_tree_builder::dimensions::Dimensions;

pub struct LayoutBox {
    pub dimensions: Dimensions,
    pub box_type: BoxType,
    pub children: Vec<LayoutBox>
}

impl LayoutBox {
    pub fn new(box_type: BoxType) -> LayoutBox {
        return LayoutBox {
            dimensions: Default::default(),
            box_type,
            children: Vec::new()
        }
    }

    pub fn get_inline_box(&mut self) -> &mut LayoutBox {
        match self.box_type {
            BoxType::Inline | BoxType::Anonymous => self,
            BoxType::Block => {
                match self.children.last() {
                    Some(&LayoutBox{ box_type: BoxType::Anonymous, .. }) => {},
                    _ => self.children.push(LayoutBox::new(BoxType::Anonymous))
                }
                self.children.last().unwrap()
            }
        }
    }
}