#[derive(Default)]
pub struct Dimensions {
    pub(crate) content_area: Rectangle,

    pub(crate) padding: Size,
    pub(crate) margin: Size,
    pub(crate) border: Size
}

impl Dimensions {
    fn padding_box(&self) -> Rectangle {
        return self.content_area.expanded_by(&self.padding);
    }

    fn border_box(&self) -> Rectangle {
        return self.padding_box().expanded_by(&self.border);
    }

    pub(crate) fn margin_box(&self) -> Rectangle {
        return self.border_box().expanded_by(&self.margin);
    }
}

#[derive(Default)]
pub struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32
}

impl Rectangle {
    fn expanded_by(&self, size: &Size) -> Rectangle {
        return Rectangle {
            x: self.x - size.left,
            y: self.y - size.top,
            width: self.width + size.left + size.right,
            height: self.height + size.top + size.bottom,
        }
    }
}

#[derive(Default)]
pub struct Size {
    pub(crate) top: f32,
    pub(crate) bottom: f32,
    pub(crate) left: f32,
    pub(crate) right: f32,
}