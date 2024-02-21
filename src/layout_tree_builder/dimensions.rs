#[derive(Default)]
pub struct Dimensions {
    content_area: Rectangle,

    padding: Size,
    margin: Size,
    border: Size
}

#[derive(Default)]
struct Rectangle {
    x: f32,
    y: f32,
    width: f32,
    height: f32
}
#[derive(Default)]
struct Size {
    top: f32,
    bottom: f32,
    left: f32,
    right: f32,
}