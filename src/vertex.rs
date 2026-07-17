#[derive(Clone, Copy)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

impl Vertex {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}