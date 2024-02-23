use std::iter::repeat;
use crate::css_parser::color_value::ColorValue;
use crate::layout_tree_builder::dimensions::Rectangle;
use crate::layout_tree_builder::layout_box::LayoutBox;
use crate::render::build_display_list;
use crate::render::display_command::DisplayCommand;

struct Canvas {
    pixels: Vec<ColorValue>,
    width: usize,
    height: usize,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        let white = ColorValue { r: 255, g: 255, b: 255, a: 255 };
        return Canvas {
            pixels: repeat(white).take(width * height).collect(),
            width,
            height,
        }
    }

    fn paint_item(&mut self, item: &DisplayCommand) {
        match item {
            &DisplayCommand::SolidColor(color, rect) => {
                // Clip the rectangle to the canvas boundaries.
                let x0 = rect.x.clamp(0.0, self.width as f32) as usize;
                let y0 = rect.y.clamp(0.0, self.height as f32) as usize;
                let x1 = (rect.x + rect.width).clamp(0.0, self.width as f32) as usize;
                let y1 = (rect.y + rect.height).clamp(0.0, self.height as f32) as usize;

                for y in (y0 .. y1) {
                    for x in (x0 .. x1) {
                        self.pixels[x + y * self.width] = color;
                    }
                }
            }
        }
    }

    fn paint(layout_root: &LayoutBox, bounds: Rectangle) -> Canvas {
        let display_list = build_display_list(layout_root);
        let mut canvas = Canvas::new(bounds.width as usize, bounds.height as usize);
        for item in display_list {
            canvas.paint_item(&item);
        }
        return canvas;
    }
}