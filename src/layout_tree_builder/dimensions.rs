#[derive(Default)]
pub struct Dimensions {
    pub(crate) content_area: Rectangle,

    pub(crate) padding: Size,
    pub(crate) margin: Size,
    pub(crate) border: Size
}

#[derive(Default)]
pub struct Rectangle {
    x: f32,
    y: f32,
    pub(crate) width: f32,
    height: f32
}
#[derive(Default)]
pub struct Size {
    top: f32,
    bottom: f32,
    pub(crate) left: f32,
    pub(crate) right: f32,
}