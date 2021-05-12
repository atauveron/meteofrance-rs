use serde::Deserialize;
use std::fmt;

/// The `Response` struct describes the result of a forecast call.
#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
struct ForecastResponse {
    pub position: Place,
    pub updated_on: u64,
    pub daily_forecast: Vec<String>,
    pub forecast: Vec<String>,
    pub probability_forecast: Vec<String>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Place {
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

#[allow(dead_code, non_snake_case)]
#[derive(Deserialize)]
struct DailyForecast {
    pub dt: u64,
    pub T: TemperatureDaily,
    pub humidity: Humidity,
    pub uv: u8,
    pub weather12H: Weather,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct TemperatureDaily {
    pub min: f32,
    pub max: f32,
    pub sea: Option<f32>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Humidity {
    pub min: u8,
    pub max: u8,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Weather {
    pub icon: String,
    pub desc: String,
}
