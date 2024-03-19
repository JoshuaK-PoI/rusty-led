use anyhow::Result;
use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct WeatherApiResponse {
    pub(crate) latitude: f64,
    pub(crate) longitude: f64,
    pub(crate) generationtime_ms: f64,
    pub(crate) utc_offset_seconds: i32,
    pub(crate) timezone: String,
    pub(crate) timezone_abbreviation: String,
    pub(crate) elevation: f64,
    pub(crate) current_units: CurrentUnits,
    pub(crate) current: Current,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct CurrentUnits {
    pub(crate) time: String,
    pub(crate) interval: String,
    pub(crate) temperature_2m: String,
    pub(crate) relative_humidity_2m: String,
    pub(crate) wind_speed_10m: String,
    pub(crate) wind_direction_10m: String,
    pub(crate) weather_code: String,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub(crate) struct Current {
    pub(crate) time: String,
    pub(crate) interval: usize,
    pub(crate) temperature_2m: f64,
    pub(crate) relative_humidity_2m: f64,
    pub(crate) wind_speed_10m: f64,
    pub(crate) wind_direction_10m: usize,
    pub(crate) weather_code: usize,
}

pub(crate) fn get_api_details() -> Result<WeatherApiResponse> {
    let url = "https://api.open-meteo.com/v1/forecast";

    let latitude = dotenv::var("LOCATION_LAT").expect("LOCATION_LAT must be set");
    let longitude = dotenv::var("LOCATION_LON").expect("LOCATION_LON must be set");

    let parameters = [
        "temperature_2m",
        "relative_humidity_2m",
        "wind_speed_10m",
        "wind_direction_10m",
        "weather_code",
    ];

    let body = Client::new()
        .get(url)
        .query(&[
            ("latitude", latitude),
            ("longitude", longitude),
            ("current", parameters.join(",")),
        ])
        .send()?
        .text()?;

    serde_json::from_str(&body).map_err(Into::into)
}
