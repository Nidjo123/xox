use pixels::Pixels;

use crate::util::color_to_rgba;

pub struct Screen {
    width: u32,
    height: u32,
    pixels: Pixels,
}

impl Screen {
    pub fn new(width: u32, height: u32, pixels: Pixels) -> Screen {
        Screen { 
            pixels, 
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) {
        let rgba = color_to_rgba(color);
        let frame = self.pixels.get_frame();
        for pixel in frame.chunks_exact_mut(4) {
            pixel.copy_from_slice(&rgba);
        }
    }

    pub fn resize_surface(&mut self, width: u32, height: u32) {
        self.pixels.resize_surface(width, height);
    }

    pub fn render(&mut self) {
        self.pixels.render()
            .expect("screen rendering failed");
    }
}