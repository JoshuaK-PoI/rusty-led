#[cfg(target_os = "linux")]
use rpi_led_matrix::{LedColor, LedMatrix, LedMatrixOptions, LedRuntimeOptions};

#[cfg(any(target_os = "macos", target_os = "windows"))]
mod simulator;

#[cfg(any(target_os = "macos", target_os = "windows"))]
use crate::simulator::{
    led_canvas::{LedCanvas, LedCanvasTrait, LedFont},
    LedColor, LedMatrix, LedMatrixOptions, LedRuntimeOptions,
};

mod led;
mod weather_api;

use chrono::Timelike;
use std::path::Path;

const REFRESH_RATE_MS: u32 = 500;

#[cfg(target_os = "linux")]
fn main() {
    let canvas = setup();
    start_draw_loop(canvas);
}

#[cfg(any(target_os = "macos", target_os = "windows"))]
fn main() {
    dotenv::dotenv().ok();

    let canvas = setup();
    let pixel_buffer = canvas.pixel_buffer.clone();
    let width = canvas.width as usize;
    let height = canvas.height as usize;

    start_draw_loop(canvas);

    // Offset the refresh polling for the window by half the rate
    // to minimize screen glitches due to incomplete buffer writes
    std::thread::sleep(std::time::Duration::from_millis(REFRESH_RATE_MS as u64 / 2));

    // Polling should be invoked last as this runs on the main thread and is blocking
    // (must be run on main thread because minifb cannot create a window from a thread)
    start_window_polling(
        pixel_buffer,
        width,
        height,
        std::time::Duration::from_millis(REFRESH_RATE_MS as u64),
    );
}

fn start_draw_loop(mut canvas: LedCanvas) {
    std::thread::spawn(move || {
        let font_lg = LedFont::new(Path::new("fonts/6x12.bdf")).unwrap();
        let font_sm = LedFont::new(Path::new("fonts/5x8.bdf")).unwrap();
        let color = led_color!("#2EC866");

        // @TODO: This needs to be run on a timeout in a separate thread
        // to periodically re-fetch weather data
        let weather = weather_api::api::get_api_details().expect("Unable to fetch weather");

        loop {
            canvas.clear();
            let now = chrono::Local::now();

            let time = now.format("%H:%M").to_string();
            let date = now.format("%a %b %e").to_string();

            canvas.draw_text(&font_lg, time.as_str(), 18, 0, &color, 0, false);

            if now.second() % 30 > 14 {
                canvas.draw_text(
                    &font_sm,
                    format!(
                        "{}{}",
                        weather.current.temperature_2m, weather.current_units.temperature_2m
                    )
                    .as_str(),
                    2,
                    18,
                    &color,
                    0,
                    false,
                );
            } else {
                canvas.draw_text(&font_sm, date.as_str(), 2, 18, &color, 0, false);
            }

            if now.second() != 0 {
                canvas.draw_line(2, 14, 2 + now.second() as i32, 14, &color);
            }

            std::thread::sleep(std::time::Duration::from_millis(REFRESH_RATE_MS as u64));
        }
    });
}

fn start_window_polling(
    pixel_buffer: std::sync::Arc<std::sync::Mutex<Vec<u32>>>,
    width: usize,
    height: usize,
    refresh_rate: std::time::Duration,
) {
    let mut window = minifb::Window::new(
        "LED matrix",
        width as usize,
        height as usize,
        minifb::WindowOptions {
            resize: false,
            scale: minifb::Scale::X16,
            ..minifb::WindowOptions::default()
        },
    )
    .expect("Failed to create window.");

    loop {
        window
            .update_with_buffer(
                &pixel_buffer.lock().unwrap(),
                width as usize,
                height as usize,
            )
            .unwrap();
        std::thread::sleep(refresh_rate);
    }
}

pub(crate) fn setup() -> LedCanvas {
    let mut options = LedMatrixOptions::new();
    options.set_cols(64);
    options.set_hardware_mapping("adafruit-hat");

    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(4);

    LedMatrix::new(Some(options), Some(rt_options))
        .unwrap()
        .offscreen_canvas()
}

fn weather_bounding_box(canvas: &mut LedCanvas, x: i32, y: i32, len: i32) {
    canvas.draw_line(x, y, x + len, y, &led_color!("#FF00FF"));
    canvas.draw_line(x, y, x, y + len, &led_color!("#FF00FF"));
    canvas.draw_line(x + len, y, x + len, y + len, &led_color!("#FF00FF"));
    canvas.draw_line(x, y + len, x + len, y + len, &led_color!("#FF00FF"));
}

#[cfg(test)]
mod tests {

    use crate::led_color;
    #[cfg(not(target_os = "linux"))]
    use crate::simulator::{
        led_canvas::{LedCanvasTrait, LedFont},
        LedColor,
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

        let font = LedFont::new(Path::new("fonts/6x10.bdf")).expect("Unable to load font");

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

            scroll_x += scroll_speed;
            scroll_x %= canvas.width as i32;

            scroll_y += scroll_speed;
            scroll_y %= canvas.height as i32;

            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    }
}
