use raylib::prelude::*;

pub struct Framebuffer {
    pub image: Image,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: i32, height: i32, background: Color) -> Self {
        Self {
            image: Image::gen_image_color(width, height, background),
            current_color: Color::WHITE,
        }
    }

    pub fn set_color(&mut self, color: Color) {
        self.current_color = color;
    }

    pub fn point(&mut self, x: i32, y: i32) {
        self.image.draw_pixel(x, y, self.current_color);
    }

    pub fn save(&mut self, filename: &str) {
        self.image.export_image(filename);
    }
}