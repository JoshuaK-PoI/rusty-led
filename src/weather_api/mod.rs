use crate::{
    led_color,
    simulator::{
        led_canvas::{LedCanvas, LedCanvasTrait},
        led_color::LedColor,
    },
};

pub(crate) mod api;
pub(crate) mod bitmaps;
pub(crate) mod canvas;

use bitmaps::*;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
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

pub(crate) fn weather_code_bitmap<'a>(condition: WeatherCondition) -> &'a [u32] {
    match condition {
        WeatherCondition::ClearSky => &bitmaps::CLEAR_SKY,
        WeatherCondition::MainlyClear => &bitmaps::MAINLY_CLEAR,
        WeatherCondition::PartlyCloudy => &bitmaps::PARTLY_CLOUDY,
        WeatherCondition::Overcast => &bitmaps::OVERCAST,
        WeatherCondition::Fog => &bitmaps::FOG,
        WeatherCondition::RimeFog => &bitmaps::RIME_FOG,
        WeatherCondition::DrizzleLight => &bitmaps::DRIZZLE_LIGHT,
        WeatherCondition::Unknown => &bitmaps::UNKNOWN,
        WeatherCondition::DrizzleModerate => &bitmaps::DRIZZLE_MODERATE,
        WeatherCondition::DrizzleDense => &bitmaps::DRIZZLE_DENSE,
        WeatherCondition::FreezingDrizzleLight => &bitmaps::FREEZING_DRIZZLE_LIGHT,
        WeatherCondition::FreezingDrizzleDense => &bitmaps::FREEZING_DRIZZLE_DENSE,
        WeatherCondition::RainSlight => &bitmaps::RAIN_SLIGHT,
        WeatherCondition::RainModerate => &bitmaps::RAIN_MODERATE,
        WeatherCondition::RainHeavy => &bitmaps::RAIN_HEAVY,
        WeatherCondition::FreezingRainLight => &bitmaps::FREEZING_RAIN_LIGHT,
        WeatherCondition::FreezingRainHeavy => &bitmaps::FREEZING_RAIN_HEAVY,
        WeatherCondition::SnowFallSlight => &bitmaps::SNOW_FALL_SLIGHT,
        WeatherCondition::SnowFallModerate => &bitmaps::SNOW_FALL_MODERATE,
        WeatherCondition::SnowFallHeavy => &bitmaps::SNOW_FALL_HEAVY,
        WeatherCondition::SnowGrains => &bitmaps::SNOW_GRAINS,
        WeatherCondition::RainShowersSlight => &bitmaps::RAIN_SHOWERS_SLIGHT,
        WeatherCondition::RainShowersModerate => &bitmaps::RAIN_SHOWERS_MODERATE,
        WeatherCondition::RainShowersViolent => &bitmaps::RAIN_SHOWERS_VIOLENT,
        WeatherCondition::SnowShowersSlight => &bitmaps::SNOW_SHOWERS_SLIGHT,
        WeatherCondition::SnowShowersHeavy => &bitmaps::SNOW_SHOWERS_HEAVY,
        WeatherCondition::ThunderstormSlight => &bitmaps::THUNDERSTORM_SLIGHT,
        WeatherCondition::ThunderstormModerate => &bitmaps::THUNDERSTORM_MODERATE,
        WeatherCondition::ThunderstormHeavy => &bitmaps::THUNDERSTORM_HEAVY,
    }
}
