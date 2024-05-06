#[cfg(target_os = "linux")]
use rpi_led_matrix::{
    LedCanvas, LedColor, LedFont
};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use crate::simulator::{
    led_canvas::{LedCanvas, LedCanvasTrait, LedFont},
    LedColor,
};

use super::WeatherCondition;
use super::{api::WeatherApiResponse, bitmaps, weather_code_bitmap};

pub(crate) fn draw_weather(
    canvas: &mut LedCanvas,
    font: &LedFont,
    weather: &WeatherApiResponse,
    x: usize,
    y: usize,
    color: &LedColor,
) {
    let x = x as i32;
    let y = y as i32;

    canvas.draw_text(
        &font,
        format!(
            "{}{}",
            weather.current.temperature_2m, weather.current_units.temperature_2m
        )
        .as_str(),
        x,
        y,
        color,
        0,
        false,
    );

    canvas.draw_text(
        &font,
        format!(
            "{}{}",
            weather.current.relative_humidity_2m, weather.current_units.relative_humidity_2m
        )
        .as_str(),
        x,
        y + 10,
        color,
        0,
        false,
    );

    draw_weather_image(canvas, x + 49, y, weather.current.weather_code.into());
    draw_wind_direction(
        canvas,
        x + 49,
        y + 18,
        weather.current.wind_direction_10m as i32,
    );
    canvas.draw_text(
        &font,
        format!(
            "{}{}",
            weather.current.wind_speed_10m, weather.current_units.wind_speed_10m
        )
        .as_str(),
        x,
        y + 20,
        color,
        0,
        false,
    );
}

fn draw_weather_image(canvas: &mut LedCanvas, x: i32, y: i32, weather_code: WeatherCondition) {
    let image_width = 15i32;
    draw_pixels(
        canvas,
        x,
        y,
        image_width,
        weather_code_bitmap(weather_code.into()),
    );
}

fn draw_wind_direction(canvas: &mut LedCanvas, x: i32, y: i32, angle: i32) {
    let image_width = 9i32;
    draw_pixels(
        canvas,
        x,
        y,
        image_width,
        get_wind_direction_from_angle(angle),
    );
}

fn draw_pixels(canvas: &mut LedCanvas, mut x: i32, mut y: i32, width: i32, pixels: &'static [u32]) {
    let max_x = x + width;
    for pixel in pixels {
        canvas.set(x, y, &(LedColor { 
            red: (*pixel >> 16) as u8,
            green: (*pixel >> 8) as u8,
            blue: *pixel as u8
        }));
        x += 1;
        if x == max_x {
            x = x - width;
            y += 1;
        }
    }
}

fn get_wind_direction_from_angle(angle: i32) -> &'static [u32] {
    // The incoming angle is the origin of the wind
    // To reflect this correctly in the wind indicator, we need to rotate it by 180 degrees
    let angle = (angle + 180) % 360;

    match angle {
        0..=22 => &bitmaps::WIND_DIRECTION_N,
        23..=67 => &bitmaps::WIND_DIRECTION_NE,
        68..=112 => &bitmaps::WIND_DIRECTION_E,
        113..=157 => &bitmaps::WIND_DIRECTION_SE,
        158..=202 => &bitmaps::WIND_DIRECTION_S,
        203..=247 => &bitmaps::WIND_DIRECTION_SW,
        248..=292 => &bitmaps::WIND_DIRECTION_W,
        293..=337 => &bitmaps::WIND_DIRECTION_NW,
        338..=360 => &bitmaps::WIND_DIRECTION_N,
        _ => &bitmaps::WIND_DIRECTIONS_UNKNOWN,
    }
}
