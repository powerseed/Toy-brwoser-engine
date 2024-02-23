use crate::css_parser::declaration_value::DeclarationValue;
use crate::css_parser::declaration_value::DeclarationValue::{Keyword, Length};
use crate::css_parser::length_unit::LengthUnit;
use crate::css_parser::length_unit::LengthUnit::Px;
use crate::layout_tree_builder::box_type::BoxType;
use crate::layout_tree_builder::dimensions::Dimensions;
use crate::style_tree_builder::styled_node::StyledNode;

pub struct LayoutBox<'a> {
    pub styled_node: Option<&'a StyledNode<'a>>,
    pub dimensions: Dimensions,
    pub box_type: BoxType,
    pub children: Vec<LayoutBox<'a>>
}

impl<'a> LayoutBox<'a> {
    pub fn new(box_type: BoxType) -> LayoutBox<'a> {
        return LayoutBox {
            styled_node: None,
            dimensions: Default::default(),
            box_type,
            children: Vec::new()
        }
    }

    pub fn get_inline_box(&mut self) -> &mut LayoutBox<'a> {
        match self.box_type {
            BoxType::Inline | BoxType::Anonymous => self,
            BoxType::Block => {
                match self.children.last() {
                    Some(&LayoutBox{ box_type: BoxType::Anonymous, .. }) => {},
                    _ => self.children.push(LayoutBox::new(BoxType::Anonymous))
                }
                self.children.last_mut().unwrap()
            }
        }
    }

    pub fn create_layout(&mut self, dimensions: &Dimensions) {
        match self.box_type {
            BoxType::Block => self.create_block_layout(dimensions),
            BoxType::Inline => todo!(),
            BoxType::Anonymous => todo!()
        }
    }

    fn create_block_layout(&mut self, parent_block_dimensions: &Dimensions) {
        self.calculate_width(parent_block_dimensions);
        // self.calculate_position();
        // self.calculate_children();
        // self.calculate_height();
    }

    fn calculate_width_from_css(&self) -> (f32, DeclarationValue, DeclarationValue, DeclarationValue, DeclarationValue, DeclarationValue, DeclarationValue, DeclarationValue) {
        let styled_node = self.styled_node.unwrap();

        let auto_width = Keyword("auto".to_string());
        let width = match styled_node.get_css_value_by_name("width".to_string()) {
            Some(declaration_value) => {
                declaration_value.clone()
            },
            _ => auto_width
        };

        let zero_length = Length(0.0, LengthUnit::Px);
        let padding_left = match styled_node.get_css_value_by_name("padding-left".to_string()) {
            Some(padding_left) => padding_left.clone(),
            _ => zero_length.clone()
        };

        let padding_right = match styled_node.get_css_value_by_name("padding-right".to_string()) {
            Some(padding_right) => padding_right.clone(),
            _ => zero_length.clone()
        };

        let margin_left = match styled_node.get_css_value_by_name("margin-left".to_string()) {
            Some(margin_left) => margin_left.clone(),
            _ => zero_length.clone()
        };

        let margin_right = match styled_node.get_css_value_by_name("margin-right".to_string()) {
            Some(margin_right) => margin_right.clone(),
            _ => zero_length.clone()
        };

        let border_left = match styled_node.get_css_value_by_name("border-left".to_string()) {
            Some(border_left) => border_left.clone(),
            _ => zero_length.clone()
        };

        let border_right = match styled_node.get_css_value_by_name("border-right".to_string()) {
            Some(border_right) => border_right.clone(),
            _ => zero_length.clone()
        };

        let width_sum = [&width, &padding_left, &padding_right, &margin_left, &margin_right, &border_left, &border_right]
            .iter()
            .map(|width| width.length_to_numerical_value())
            .sum();

        return (width_sum, width, margin_left, margin_right, padding_left, padding_right, border_left, border_right)
    }

    fn calculate_width(&mut self, parent_block_dimensions: &Dimensions) {
        let (width_sum,
            mut width,
            mut margin_left,
            mut margin_right,
            padding_left,
            padding_right,
            border_left,
            border_right) = self.calculate_width_from_css();

        if width != Keyword("auto".to_string()) && width_sum > parent_block_dimensions.content_area.width {
            if margin_left == Keyword("auto".to_string()) {
                margin_left = Length(0.0, Px);
            }
            if margin_right == Keyword("auto".to_string()) {
                margin_right = Length(0.0, Px);
            }
        }

        let underflow = parent_block_dimensions.content_area.width - width_sum;

        match (width == Keyword("auto".to_string()), margin_left == Keyword("auto".to_string()), margin_right == Keyword("auto".to_string())) {
            (false, false, false) => {
                margin_right = Length(margin_right.length_to_numerical_value() + underflow, Px);
            }

            (false, false, true) => { margin_right = Length(underflow, Px); }
            (false, true, false) => { margin_left  = Length(underflow, Px); }

            (true, _, _) => {
                if margin_left == Keyword("auto".to_string()) { margin_left = Length(0.0, Px); }
                if margin_right == Keyword("auto".to_string()) { margin_right = Length(0.0, Px); }

                if underflow >= 0.0 {
                    width = Length(underflow, Px);
                } else {
                    width = Length(0.0, Px);
                    margin_right = Length(margin_right.length_to_numerical_value() + underflow, Px);
                }
            }

            (false, true, true) => {
                margin_left = Length(underflow / 2.0, Px);
                margin_right = Length(underflow / 2.0, Px);
            }
        }

        self.dimensions.content_area.width = width.length_to_numerical_value();

        self.dimensions.padding.left = padding_left.length_to_numerical_value();
        self.dimensions.padding.right = padding_right.length_to_numerical_value();

        self.dimensions.border.left = border_left.length_to_numerical_value();
        self.dimensions.border.right = border_right.length_to_numerical_value();

        self.dimensions.margin.left = margin_left.length_to_numerical_value();
        self.dimensions.margin.right = margin_right.length_to_numerical_value();
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::css_parser::declaration_value::DeclarationValue::{Keyword, Length};
    use crate::css_parser::length_unit::LengthUnit;
    use crate::html_parser::node::Node;
    use crate::html_parser::node_type::NodeType;
    use crate::layout_tree_builder::box_type::BoxType;
    use crate::layout_tree_builder::layout_box::LayoutBox;
    use crate::style_tree_builder::styled_node::StyledNode;

    #[test]
    fn test_calculate_width() {
        let mut layout_box = LayoutBox::new(BoxType::Block);
        let styled_node = StyledNode {
            dom_node: &Node {
                children: Vec::new(),
                node_type: NodeType::Text("abc".to_string())
            },
            css_properties: HashMap::from([
                ("width".to_string(), Length(7.5, LengthUnit::Px)),
                ("margin-left".to_string(), Keyword("auto".to_string())),
                ("margin-right".to_string(), Length(2.2, LengthUnit::Px)),
                ("border-left".to_string(), Length(15.6, LengthUnit::Px)),
                ("border-right".to_string(), Length(6.3, LengthUnit::Px)),
                ("padding-left".to_string(), Length(7.8, LengthUnit::Px)),
                ("padding-right".to_string(), Length(8.9, LengthUnit::Px)),
            ]),
            children: vec![],
        };
        layout_box.styled_node = Some(&styled_node);

        assert_eq!(
            (48.3, Length(7.5, LengthUnit::Px), Keyword("auto".to_string()), Length(2.2, LengthUnit::Px), Length(7.8, LengthUnit::Px), Length(8.9, LengthUnit::Px), Length(15.6, LengthUnit::Px), Length(6.3, LengthUnit::Px)),
            layout_box.calculate_width_from_css()
        );
    }
}