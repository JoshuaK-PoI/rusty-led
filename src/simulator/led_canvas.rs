use std::{
    path::Path,
    sync::{Arc, Mutex},
};

use super::LedColor;

pub(crate) struct LedFont {
    pub(crate) font: bdf::Font,
}

impl LedFont {
    pub fn new(bdf_file: &Path) -> Result<Self, &'static str> {
        let font = bdf::open(bdf_file).map_err(|_| "Failed to open BDF font file")?;
        Ok(Self { font })
    }
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
    );
}

pub(crate) struct LedCanvas {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) pixel_buffer: Arc<Mutex<Vec<u32>>>,
}

impl LedCanvas {
    pub(crate) fn new(height: u32, width: u32) -> Self {
        // Preallocate vector space for the pixels
        // Make into Arc<Mutex<>> to allow for concurrent access
        let pixel_buffer = Arc::new(Mutex::new(Vec::with_capacity((height * width) as usize)));
        // Note: pixel_buffer.fill() does not work here
        for _ in 0..(height * width) {
            pixel_buffer.lock().unwrap().push(LedColor::zero().into());
        }

        assert_eq!(
            pixel_buffer.lock().unwrap().len(),
            (height * width) as usize
        );

        Self {
            width,
            height,
            pixel_buffer,
        }
    }
}

impl LedCanvasTrait for LedCanvas {
    fn fill(&mut self, color: &LedColor) {
        for pixel in &mut self.pixel_buffer.lock().unwrap().iter_mut() {
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
        self.pixel_buffer.lock().unwrap()[index] = (*color).into();
    }

    fn clear(&mut self) {
        self.fill(&LedColor::zero())
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
        mut start_x: i32,
        mut start_y: i32,
        color: &LedColor,
        mut kerning_offset: i32,
        vertical: bool,
    ) {
        kerning_offset = std::cmp::max(0, kerning_offset);

        for c in text.chars() {
            let glyph = font.font.glyphs().get(&c).unwrap();
            let pixels = glyph.pixels();
            for ((x, y), draw) in pixels {
                if draw {
                    if vertical {
                        self.set(
                            (x as i32 + start_x) % self.width as i32,
                            (y as i32 + start_y) % self.height as i32,
                            color,
                        );
                    } else {
                        self.set(
                            (x as i32 + start_x) % self.width as i32,
                            (y as i32 + start_y) % self.height as i32,
                            color,
                        );
                    }
                }
            }

            if vertical {
                start_y += glyph.height() as i32 + kerning_offset;
                start_y %= self.height as i32;
            } else {
                start_x += glyph.width() as i32 + kerning_offset;
                start_x %= self.width as i32;
            }
        }
    }
}
