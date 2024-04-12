#[cfg(target_os = "linux")]
use rpi_led_matrix::{LedColor, LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use weather_api::api::WeatherApiResponse;

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
    use std::sync::{Arc, Mutex};

    dotenv::dotenv().ok();

    let canvas = setup();
    let pixel_buffer = canvas.pixel_buffer.clone();
    let width = canvas.width as usize;
    let height = canvas.height as usize;

    let weather_response = Arc::new(Mutex::new(WeatherApiResponse::default()));

    start_draw_loop(canvas, weather_response.clone());

    start_weather_api_polling(weather_response);

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

fn start_draw_loop(
    mut canvas: LedCanvas,
    weather_response: std::sync::Arc<std::sync::Mutex<WeatherApiResponse>>,
) {
    std::thread::spawn(move || {
        let font_lg = LedFont::new(Path::new("fonts/6x12.bdf")).unwrap();
        let font_sm = LedFont::new(Path::new("fonts/5x8.bdf")).unwrap();
        let color = led_color!("#2EC866");

        loop {
            canvas.clear();
            let now = chrono::Local::now();

            let time = now.format("%H:%M").to_string();
            let date = now.format("%a %b %e").to_string();

            canvas.draw_text(&font_lg, time.as_str(), 18, 0, &color, 0, false);
            canvas.draw_text(&font_sm, date.as_str(), 2, 18, &color, 0, false);

            if now.second() != 0 {
                canvas.draw_line(2, 14, 2 + now.second() as i32, 14, &color);
            }

            let weather = weather_response.lock().unwrap();
            weather_api::canvas::draw_weather(&mut canvas, &font_sm, &weather, 1, 33, &color);

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

fn start_weather_api_polling(
    weather_response: std::sync::Arc<std::sync::Mutex<WeatherApiResponse>>,
) {
    std::thread::spawn(move || loop {
        let weather = weather_api::api::get_api_details().expect("Unable to fetch weather");

        println!("{:?}", weather);

        *weather_response.lock().unwrap() = weather;
        std::thread::sleep(std::time::Duration::from_secs(300));
    });
}

pub(crate) fn setup() -> LedCanvas {
    let mut options = LedMatrixOptions::new();
    options.set_cols(64);
    options.set_rows(64);
    options.set_hardware_mapping("adafruit-hat");

    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(4);

    LedMatrix::new(Some(options), Some(rt_options))
        .unwrap()
        .offscreen_canvas()
}
