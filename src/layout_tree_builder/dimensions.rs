#[derive(Default)]
pub struct Dimensions {
    pub(crate) content_area: Rectangle,

    pub(crate) padding: Size,
    pub(crate) margin: Size,
    pub(crate) border: Size
}

#[derive(Default)]
pub struct Rectangle {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) width: f32,
    pub(crate) height: f32
}
#[derive(Default)]
pub struct Size {
    pub(crate) top: f32,
    pub(crate) bottom: f32,
    pub(crate) left: f32,
    pub(crate) right: f32,
}