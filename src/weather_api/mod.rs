pub(crate) mod api;
pub(crate) mod bitmaps;
pub(crate) mod canvas;

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
