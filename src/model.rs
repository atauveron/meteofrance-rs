use serde::Deserialize;
use std::fmt;

/// The `Response` struct describes the result of a forecast call.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ForecastResponse {
    pub position: Place,
    pub updated_on: u64,
    pub daily_forecast: Vec<DailyForecast>,
    pub forecast: Vec<Forecast>,
    pub probability_forecast: Vec<ProbabilityForecast>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Place {
    pub lat: f32,
    pub lon: f32,
    pub alti: i16,
    pub name: String,
    pub country: String,
    pub dept: Option<String>,
    pub rain_product_available: u8,
    pub timezone: String,
    pub insee: Option<String>,
}

impl fmt::Display for Place {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.dept {
            None => write!(f, "{} - {}", self.name, self.country),
            Some(dept) => write!(f, "{} ({}) - {}", self.name, dept, self.country),
        }
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DailyForecast {
    pub dt: u64,
    #[serde(rename = "T")]
    pub temp: TemperatureDaily,
    pub humidity: Humidity,
    pub uv: Option<u8>,
    #[serde(rename = "weather12H")]
    pub weather_12h: Weather,
    pub sun: RiseSet,
}

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
    pub min: f32,
    pub max: f32,
    pub sea: Option<f32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Humidity {
    pub min: u8,
    pub max: u8,
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
