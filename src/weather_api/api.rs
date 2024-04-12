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

impl WeatherApiResponse {
    pub(crate) fn default() -> WeatherApiResponse {
        WeatherApiResponse {
            latitude: 0.0,
            longitude: 0.0,
            generationtime_ms: 0.0,
            utc_offset_seconds: 0,
            timezone: "".to_string(),
            timezone_abbreviation: "".to_string(),
            elevation: 0.0,
            current_units: CurrentUnits {
                time: "".to_string(),
                interval: "".to_string(),
                temperature_2m: "".to_string(),
                relative_humidity_2m: "".to_string(),
                wind_speed_10m: "".to_string(),
                wind_direction_10m: "".to_string(),
                weather_code: "".to_string(),
            },
            current: Current {
                time: "".to_string(),
                interval: 0,
                temperature_2m: 0.0,
                relative_humidity_2m: 0.0,
                wind_speed_10m: 0.0,
                wind_direction_10m: 0,
                weather_code: 0,
            },
        }
    }
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

const URL: &str = "https://api.open-meteo.com/v1/forecast";

#[cfg(not(feature = "mock"))]
pub(crate) fn get_api_details() -> Result<WeatherApiResponse> {
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
        .get(URL)
        .query(&[
            ("latitude", latitude),
            ("longitude", longitude),
            ("current", parameters.join(",")),
        ])
        .send()?
        .text()?;

    serde_json::from_str(&body).map_err(Into::into)
}

#[cfg(feature = "mock")]
pub(crate) fn get_api_details() -> Result<WeatherApiResponse> {
    let body = r#"{
        "latitude": 51.5074,
        "longitude": 0.1278,
        "generationtime_ms": 0.0024892583952,
        "utc_offset_seconds": 3600,
        "timezone": "Europe/London",
        "timezone_abbreviation": "BST",
        "elevation": 0,
        "current_units": {
            "time": "s",
            "interval": "s",
            "temperature_2m": "°C",
            "relative_humidity_2m": "%",
            "wind_speed_10m": "m/s",
            "wind_direction_10m": "°",
            "weather_code": ""
        },
        "current": {
            "time": "2021-07-20T14:00:00Z",
            "interval": 0,
            "temperature_2m": 20.3,
            "relative_humidity_2m": 60,
            "wind_speed_10m": 3.6,
            "wind_direction_10m": 245,
            "weather_code": 0
        }
    }"#;

    serde_json::from_str(&body).map_err(Into::into)
}
