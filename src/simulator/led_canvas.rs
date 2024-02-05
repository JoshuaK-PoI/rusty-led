use super::LedColor;

use minifb::{Window, WindowOptions};


struct LedFont {
    // ...
}

pub(crate) trait LedCanvasTrait {
    /// Retrieves the width & height of the canvas
    #[must_use]
    fn canvas_size(&self) -> (i32, i32);

    /// Sets the pixel at the given coordinate to the given color.
    fn set(&mut self, x: i32, y: i32, color: &LedColor);

    /// Clears the canvas.
    fn clear(&mut self);

    /// Fills the canvas with the given color.
    fn fill(&mut self, color: &LedColor);

    /// Draws a straight, one pixel wide line using the C++ library.
    ///
    /// Consider using embedded-graphics for more drawing features.
    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &LedColor);



    /// Draws a one pixel wide circle using the C++ library.
    ///
    /// Consider using embedded-graphics for more drawing features.
    fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: &LedColor);

    #[allow(clippy::too_many_arguments)]
    /// Renders text using the C++ library.
    ///
    /// # Panics
    /// If the given `text` fails to convert to a `CString`. This can
    /// occur when there is a null character mid way in the string.
    fn draw_text(
        &mut self,
        font: &LedFont,
        text: &str,
        x: i32,
        y: i32,
        color: &LedColor,
        kerning_offset: i32,
        vertical: bool,
    ) -> i32;
}

pub(crate) struct LedCanvas {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pixel_buffer: Vec<u32>,

    window: Window, // minifb UI window
}

const SCALE: u32 = 16; // Scales pixels to 16x - for 64 width this makes 1024 'real' pixels

impl LedCanvas {
    pub fn new(height: u32, width: u32) -> Self {
        let window = Window::new(
            "LED Matrix Simulator",
            (width * SCALE) as usize,
            (height * SCALE) as usize,
            WindowOptions::default(),
        ).expect("Unable to create window");
        
        // Preallocate vector space for the pixels
        let mut pixel_buffer = Vec::with_capacity((height * width) as usize);
        for _ in 0..(height * width) {
            pixel_buffer.push(LedColor::zero().into());
        }
        
        Self {
            height,
            width,
            pixel_buffer,
            window
        }
    }

    pub(crate) fn flush_buffer(&mut self) {
        self.window.update_with_buffer(&self.pixel_buffer, (self.width) as usize, (self.height) as usize)
            .expect("Unable to update window");
    }
}


impl LedCanvasTrait for LedCanvas {
    fn fill(&mut self, color: &LedColor) {
        println!("Filling canvas with color: {:?}", color);
        for pixel in &mut self.pixel_buffer {
            *pixel = (*color).into();
        }
    }

    fn canvas_size(&self) -> (i32, i32) {
        (self.width as i32, self.height as i32)
    }

    fn set(&mut self, x: i32, y: i32, color: &LedColor) {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return;
        }
        let index = (y * self.width as i32 + x) as usize;
        self.pixel_buffer[index] = (*color).into();
    }

    fn clear(&mut self) {
        for pixel in &mut self.pixel_buffer {
            *pixel = LedColor::zero().into();
        }
    }

    fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32, color: &LedColor) {
        // Bresenham's line algorithm
        let mut x0 = x0;
        let mut y0 = y0;
        let x1 = x1;
        let y1 = y1;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        loop {
            self.set(x0, y0, color);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    fn draw_circle(&mut self, x: i32, y: i32, radius: u32, color: &LedColor) {
        let mut x0 = 0;
        let mut y0 = radius as i32;
        let mut d = 3 - 2 * radius as i32;
        while x0 <= y0 {
            self.set(x + x0, y + y0, color);
            self.set(x + x0, y - y0, color);
            self.set(x - x0, y + y0, color);
            self.set(x - x0, y - y0, color);
            self.set(x + y0, y + x0, color);
            self.set(x + y0, y - x0, color);
            self.set(x - y0, y + x0, color);
            self.set(x - y0, y - x0, color);
            if d < 0 {
                d += 4 * x0 + 6;
            } else {
                d += 4 * (x0 - y0) + 10;
                y0 -= 1;
            }
            x0 += 1;
        }
    }

    fn draw_text(
        &mut self,
        font: &LedFont,
        text: &str,
        x: i32,
        y: i32,
        color: &LedColor,
        kerning_offset: i32,
        vertical: bool,
    ) -> i32 {
        todo!()
    }

}