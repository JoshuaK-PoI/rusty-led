use crate::{
    led_color,
    simulator::{
        led_canvas::{LedCanvas, LedCanvasTrait},
        led_color::LedColor,
    },
};

struct Weather {
    temperature: f64,
    wind: f64,
    humidity: f64,
    condition: WeatherCondition,
}

const LATITUDE: f64 = 52.10;
const LONGITUDE: f64 = 6.05;

pub(crate) enum WeatherCondition {
    Unknown,
    ClearSky,
    MainlyClear,
    PartlyCloudy,
    Overcast,
    Fog,
    RimeFog,
    DrizzleLight,
    DrizzleModerate,
    DrizzleDense,
    FreezingDrizzleLight,
    FreezingDrizzleDense,
    RainSlight,
    RainModerate,
    RainHeavy,
    FreezingRainLight,
    FreezingRainHeavy,
    SnowFallSlight,
    SnowFallModerate,
    SnowFallHeavy,
    SnowGrains,
    RainShowersSlight,
    RainShowersModerate,
    RainShowersViolent,
    SnowShowersSlight,
    SnowShowersHeavy,
    ThunderstormSlight,
    ThunderstormModerate,
    ThunderstormHeavy,
}

impl From<usize> for WeatherCondition {
    fn from(value: usize) -> Self {
        match value {
            0 => WeatherCondition::ClearSky,
            1 => WeatherCondition::MainlyClear,
            2 => WeatherCondition::PartlyCloudy,
            3 => WeatherCondition::Overcast,
            45 => WeatherCondition::Fog,
            48 => WeatherCondition::RimeFog,
            51 => WeatherCondition::DrizzleLight,
            53 => WeatherCondition::DrizzleModerate,
            55 => WeatherCondition::DrizzleDense,
            56 => WeatherCondition::FreezingDrizzleLight,
            57 => WeatherCondition::FreezingDrizzleDense,
            61 => WeatherCondition::RainSlight,
            63 => WeatherCondition::RainModerate,
            65 => WeatherCondition::RainHeavy,
            66 => WeatherCondition::FreezingRainLight,
            67 => WeatherCondition::FreezingRainHeavy,
            71 => WeatherCondition::SnowFallSlight,
            73 => WeatherCondition::SnowFallModerate,
            75 => WeatherCondition::SnowFallHeavy,
            77 => WeatherCondition::SnowGrains,
            80 => WeatherCondition::RainShowersSlight,
            81 => WeatherCondition::RainShowersModerate,
            82 => WeatherCondition::RainShowersViolent,
            85 => WeatherCondition::SnowShowersSlight,
            86 => WeatherCondition::SnowShowersHeavy,
            95 => WeatherCondition::ThunderstormSlight,
            96 => WeatherCondition::ThunderstormModerate,
            99 => WeatherCondition::ThunderstormHeavy,
            _ => WeatherCondition::Unknown,
        }
    }
}

impl Into<String> for WeatherCondition {
    fn into(self) -> String {
        match self {
            WeatherCondition::Unknown => "Unknown".to_string(),
            WeatherCondition::ClearSky => "Clear Sky".to_string(),
            WeatherCondition::MainlyClear => "Mainly Clear".to_string(),
            WeatherCondition::PartlyCloudy => "Partly Cloudy".to_string(),
            WeatherCondition::Overcast => "Overcast".to_string(),
            WeatherCondition::Fog => "Fog".to_string(),
            WeatherCondition::RimeFog => "Rime Fog".to_string(),
            WeatherCondition::DrizzleLight => "Drizzle Light".to_string(),
            WeatherCondition::DrizzleModerate => "Drizzle Moderate".to_string(),
            WeatherCondition::DrizzleDense => "Drizzle Dense".to_string(),
            WeatherCondition::FreezingDrizzleLight => "Freezing Drizzle Light".to_string(),
            WeatherCondition::FreezingDrizzleDense => "Freezing Drizzle Dense".to_string(),
            WeatherCondition::RainSlight => "Rain Slight".to_string(),
            WeatherCondition::RainModerate => "Rain Moderate".to_string(),
            WeatherCondition::RainHeavy => "Rain Heavy".to_string(),
            WeatherCondition::FreezingRainLight => "Freezing Rain Light".to_string(),
            WeatherCondition::FreezingRainHeavy => "Freezing Rain Heavy".to_string(),
            WeatherCondition::SnowFallSlight => "Snow Fall Slight".to_string(),
            WeatherCondition::SnowFallModerate => "Snow Fall Moderate".to_string(),
            WeatherCondition::SnowFallHeavy => "Snow Fall Heavy".to_string(),
            WeatherCondition::SnowGrains => "Snow Grains".to_string(),
            WeatherCondition::RainShowersSlight => "Rain Showers Slight".to_string(),
            WeatherCondition::RainShowersModerate => "Rain Showers Moderate".to_string(),
            WeatherCondition::RainShowersViolent => "Rain Showers Violent".to_string(),
            WeatherCondition::SnowShowersSlight => "Snow Showers Slight".to_string(),
            WeatherCondition::SnowShowersHeavy => "Snow Showers Heavy".to_string(),
            WeatherCondition::ThunderstormSlight => "Thunderstorm Slight".to_string(),
            WeatherCondition::ThunderstormModerate => "Thunderstorm Moderate".to_string(),
            WeatherCondition::ThunderstormHeavy => "Thunderstorm Heavy".to_string(),
        }
    }
}

const COLORS: [u32; 3] = [
    0xFFFFFF, // Cloud (white)
    0xFFFF00, // Sun (yellow)
    0x0000FF, // Rain (blue)
];

pub(crate) fn weather_code_to_image(
    canvas: &mut LedCanvas,
    x: i32,
    y: i32,
    condition: WeatherCondition,
) {
    match condition {
        WeatherCondition::ClearSky => {
            canvas.draw_circle(x + 6, y + 6, 5, &led_color!(COLORS[1]));
        }
        WeatherCondition::MainlyClear => todo!("Mainly Clear"),
        WeatherCondition::PartlyCloudy => todo!("Partly Cloudy"),
        WeatherCondition::Overcast => todo!("Overcast"),
        WeatherCondition::Fog => todo!("Fog"),
        WeatherCondition::RimeFog => todo!("Rime Fog"),
        WeatherCondition::DrizzleLight => todo!("Drizzle Light"),
        WeatherCondition::Unknown => todo!(),
        WeatherCondition::DrizzleModerate => todo!(),
        WeatherCondition::DrizzleDense => todo!(),
        WeatherCondition::FreezingDrizzleLight => todo!(),
        WeatherCondition::FreezingDrizzleDense => todo!(),
        WeatherCondition::RainSlight => todo!(),
        WeatherCondition::RainModerate => todo!(),
        WeatherCondition::RainHeavy => todo!(),
        WeatherCondition::FreezingRainLight => todo!(),
        WeatherCondition::FreezingRainHeavy => todo!(),
        WeatherCondition::SnowFallSlight => todo!(),
        WeatherCondition::SnowFallModerate => todo!(),
        WeatherCondition::SnowFallHeavy => todo!(),
        WeatherCondition::SnowGrains => todo!(),
        WeatherCondition::RainShowersSlight => todo!(),
        WeatherCondition::RainShowersModerate => todo!(),
        WeatherCondition::RainShowersViolent => todo!(),
        WeatherCondition::SnowShowersSlight => todo!(),
        WeatherCondition::SnowShowersHeavy => todo!(),
        WeatherCondition::ThunderstormSlight => todo!(),
        WeatherCondition::ThunderstormModerate => todo!(),
        WeatherCondition::ThunderstormHeavy => todo!(),
    }
}
