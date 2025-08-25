// framebuffer.rs
use raylib::prelude::*;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color_buffer: Image,
    pub background_color: Color,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, background_color: Color) -> Self {
        let img = Image::gen_image_color(width as i32, height as i32, background_color);
        Self { width, height, color_buffer: img, background_color, current_color: Color::WHITE }
    }

    pub fn clear(&mut self) {
        self.color_buffer = Image::gen_image_color(self.width as i32, self.height as i32, self.background_color);
    }

    pub fn set_current_color(&mut self, c: Color) { self.current_color = c; }
    pub fn set_pixel(&mut self, x: u32, y: u32) {
        if x < self.width && y < self.height {
            self.color_buffer.draw_pixel(x as i32, y as i32, self.current_color);
        }
    }

pub fn swap_buffers(&self, rl: &mut RaylibHandle, th: &RaylibThread) {
    if let Ok(tex) = rl.load_texture_from_image(th, &self.color_buffer) {
        let mut d = rl.begin_drawing(th);
        d.clear_background(Color::BLACK);
        d.draw_texture(&tex, 0, 0, Color::WHITE);


        d.draw_fps(10, 10);
    }
}

}
