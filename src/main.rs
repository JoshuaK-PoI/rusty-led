use std::path::Path;
use chrono::Timelike;

#[cfg(target_os = "linux")]
use rpi_led_matrix::{LedColor, LedMatrix, LedMatrixOptions, LedRuntimeOptions};

#[cfg(not(target_os = "linux"))]
mod simulator;
#[cfg(not(target_os = "linux"))]
use crate::simulator::{
    led_canvas::{LedCanvas, LedCanvasTrait, LedFont},
    LedMatrix, LedMatrixOptions, LedRuntimeOptions, LedColor
};

const REFRESH_RATE_MS: u32 = 16;

macro_rules! led_color {
    ($hex:literal) => {{
        let hex = if $hex.starts_with("#") {
            &$hex[1..]
        } else {
            $hex
        };

        LedColor::from(u32::from_str_radix(hex, 16).unwrap())
    }};
}

fn main() {
    let mut canvas = setup();
    let font_lg = LedFont::new(Path::new("fonts/6x12.bdf")).unwrap();
    let font_sm = LedFont::new(Path::new("fonts/5x7.bdf")).unwrap();
    let color = led_color!("#2EC866");

    let mut toggle_colon = true;

    loop {
        canvas.clear();
        let now = chrono::Local::now();

        let time = if toggle_colon {
            now.format("%H:%M").to_string()
        } else {
            now.format("%H %M").to_string()
        };

        let date = now.format("%a %b %e").to_string();

        canvas.draw_text(
            &font_lg,
            time.as_str(),
            0,
            0,
            &color,
            0,
            false,
        );

        canvas.draw_text(
            &font_sm,
            date.as_str(),
            0,
            16,
            &color,
            0,
            false,
        );

        let line = (now.second() as f32 / 60.0 * 60.0) as i32;
        canvas.draw_line(0, 0, 1, 0, &color);
        canvas.draw_line(62, 0, 63, 0, &color);
        canvas.draw_line(2, 0, 2 + line, 0, &color);

        #[cfg(not(target_os = "linux"))]
        canvas.flush_buffer();
        
        std::thread::sleep(std::time::Duration::from_millis(REFRESH_RATE_MS as u64 * 60));
        toggle_colon = !toggle_colon;
    }

}

fn setup() -> LedCanvas {
    let mut options = LedMatrixOptions::new();
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat");

    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(4);

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();
    canvas.set_refresh_rate(std::time::Duration::from_millis(REFRESH_RATE_MS as u64));
    canvas.clear();

    canvas
}

#[cfg(test)]
mod tests {

    #[cfg(not(target_os = "linux"))]
    use crate::simulator::{
        led_canvas::{LedCanvasTrait, LedFont}, LedColor
    };

    use std::path::Path;

    #[test]
    fn test_pattern_with_color_output() {
        let mut canvas = super::setup();

        let colors = vec![
            led_color!("#2EC866"),
            led_color!("#2EF816"),
            led_color!("#2EFA66"),
            led_color!("#2EFC16"),
            led_color!("#2EFE66"),
            led_color!("#2EFE16"),
            led_color!("#2EFE66"),
        ];

        let font = LedFont::new(Path::new("fonts/6x10.bdf"))
            .expect("Unable to load font");

        let scroll_speed: i32 = 2;

        let mut scroll_x: i32 = 0;
        let mut scroll_y: i32 = 0;

        for i in 0..colors.len() {
            canvas.clear();
            let color = colors[i];

            // Draw a box outline with diagonals through the middle
            // and a circle in the center
            // Offset the test pattern by i every iteration
            canvas.draw_line(0, 0, 63, 0, &color);
            canvas.draw_line(63, 0, 63, 31, &color);
            canvas.draw_line(63, 31, 0, 31, &color);
            canvas.draw_line(0, 31, 0, 0, &color);
            canvas.draw_line(0, 0, 63, 31, &color);
            canvas.draw_line(63, 0, 0, 31, &color);
            canvas.draw_circle(32, 16, 10, &color);

            canvas.draw_text(
                &font,
                Into::<String>::into(color).as_str(),
                scroll_x,
                scroll_y,
                &led_color!("#2EC866"),
                0,
                false,
            );

            // @FIXME:
            // Because the simulator uses a double buffer
            // (our own pixel buffer and the minifb window buffer)
            // there is a race condition where the next
            // draw_line or draw_circle will be called before
            // the previous draw call has finished.
            // This is not an issue with the real LED matrix
            // because that directly talks to GPIO.
            // Ideally, the simulator should be updated to
            // mimic the hardware more closely.
            // For now, this 'flush_buffer' call is a workaround
            // to make the simulator work as expected.
            #[cfg(not(target_os = "linux"))]
            canvas.flush_buffer();

            scroll_x += scroll_speed;
            scroll_x %= canvas.width as i32;

            scroll_y += scroll_speed;
            scroll_y %= canvas.height as i32;

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
