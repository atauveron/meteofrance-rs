use serde::Deserialize;

use super::place::Place;

/// The `ForecastResponse` struct describes the result of a forecast call.
/// It contains the hourly and daily weather forecast.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ForecastResponse {
    pub position: Place,
    pub updated_on: u64,
    pub daily_forecast: Vec<DailyForecast>,
    pub forecast: Vec<Forecast>,
    pub probability_forecast: Vec<ProbabilityForecast>,
}

/// The `DailyForecast` struct contains forecast information for a given day.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DailyForecast {
    pub dt: u64,
    #[serde(rename = "T")]
    pub temp: TemperatureDaily,
    pub humidity: Humidity,
    pub uv: Option<u8>,
    #[serde(rename = "weather12H")]
    pub weather_12h: Option<Weather>,
    pub sun: RiseSet,
}

/// The `Forecast` struct contains forecast information.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub dt: u64,
    #[serde(rename = "T")]
    pub temp: Temperature,
    pub humidity: u8,
    pub sea_level: f32,
    pub wind: Wind,
    pub rain: Precipitation,
    pub snow: Precipitation,
    pub iso0: u16,
    // FIXME This field is either an altitude (u16) or string "Non pertinent"
    // #[serde(rename = "rain snow limit")]
    // pub rain_snow_limit: u16,
    pub clouds: u8,
    pub weather: Weather,
}

/// The `ProbabilityForecast` struct contains information about the probability of various hazards
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ProbabilityForecast {
    pub dt: u64,
    pub rain: Precipitation,
    pub snow: Precipitation,
    pub freezing: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TemperatureDaily {
    pub min: Option<f32>,
    pub max: Option<f32>,
    pub sea: Option<f32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Humidity {
    pub min: Option<u8>,
    pub max: Option<u8>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Weather {
    pub icon: String,
    pub desc: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RiseSet {
    pub rise: u64,
    pub set: u64,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Temperature {
    pub value: f32,
    pub windchill: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Wind {
    pub speed: f32,
    pub gust: f32,
    pub direction: i32,
    icon: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Precipitation {
    #[serde(rename = "1h")]
    pub one_hour: Option<f32>,
    #[serde(rename = "3h")]
    pub three_hours: Option<f32>,
    #[serde(rename = "6h")]
    pub six_hours: Option<f32>,
}
