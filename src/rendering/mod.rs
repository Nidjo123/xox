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

    fn set_pixel(&mut self, x: i32, y: i32, color: &[u8; 4]) {
        let (x, y) = self.pixels.clamp_pixel_pos((x as isize, y as isize));
        let mut frame = self.pixels.get_frame();
        let idx = (y * self.width as usize + x) * 4;
        frame[idx..idx+4].copy_from_slice(color);
    }

    // fantastic resource: http://members.chello.at/%7Eeasyfilter/Bresenham.pdf
    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: u32) {
        let mut frame = self.pixels.get_frame();
        let color = color_to_rgba(color);
        
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.set_pixel(x, y, &color);
            let e2 = 2 * err;
            if e2 >= dy {
                if x == x1 {
                    break;
                }
                err += dy;
                x += sx;
            } else if e2 <= dx {
                if y == y1 {
                    break;
                }
                err += dx;
                y += sy;
            }
        }
    }

    pub fn render(&mut self) {
        self.pixels.render().expect("screen rendering failed");
    }
}
