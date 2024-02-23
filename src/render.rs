use crate::css_parser::color_value::ColorValue;
use crate::css_parser::declaration_value::DeclarationValue;
use crate::layout_tree_builder::box_type::BoxType;
use crate::layout_tree_builder::dimensions::Rectangle;
use crate::layout_tree_builder::layout_box::LayoutBox;
use crate::render::display_command::DisplayCommand;

mod display_command;
mod canvas;

fn build_display_list(layout_root: &LayoutBox) -> Vec<DisplayCommand> {
    let mut list = Vec::new();
    render_layout_box(&mut list, layout_root);
    return list;
}

fn render_layout_box(list: &mut Vec<DisplayCommand>, layout_box: &LayoutBox) {
    render_background(list, layout_box);
    render_borders(list, layout_box);

    for child in &layout_box.children {
        render_layout_box(list, child);
    }
}

fn render_background(list: &mut Vec<DisplayCommand>, layout_box: &LayoutBox) {
    get_color(layout_box, "background").map(|color|
        list.push(DisplayCommand::SolidColor(color, layout_box.dimensions.border_box())));
}

fn get_color(layout_box: &LayoutBox, name: &str) -> Option<ColorValue> {
    match layout_box.box_type {
        BoxType::Block | BoxType::Inline => match layout_box.styled_node.unwrap().get_css_value_by_name(name.to_string()) {
            Some(DeclarationValue::Color(color)) => Some(*color),
            _ => None
        },
        BoxType::Anonymous => None
    }
}

fn render_borders(list: &mut Vec<DisplayCommand>, layout_box: &LayoutBox) {
    let color = match get_color(layout_box, "border-color") {
        Some(color) => color,
        _ => return // bail out if no border-color is specified
    };

    let d = &layout_box.dimensions;
    let border_box = d.border_box();

    // Left border
    list.push(DisplayCommand::SolidColor(color, Rectangle {
        x: border_box.x,
        y: border_box.y,
        width: d.border.left,
        height: border_box.height,
    }));

    // Right border
    list.push(DisplayCommand::SolidColor(color, Rectangle {
        x: border_box.x + border_box.width - d.border.right,
        y: border_box.y,
        width: d.border.right,
        height: border_box.height,
    }));

    // Top border
    list.push(DisplayCommand::SolidColor(color, Rectangle {
        x: border_box.x,
        y: border_box.y,
        width: border_box.width,
        height: d.border.top,
    }));

    // Bottom border
    list.push(DisplayCommand::SolidColor(color, Rectangle {
        x: border_box.x,
        y: border_box.y + border_box.height - d.border.bottom,
        width: border_box.width,
        height: d.border.bottom,
    }));
}