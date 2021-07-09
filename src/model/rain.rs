use serde::Deserialize;

use super::place::Place;

/// The `RainResponse` struct describes the result of a forecast call.
/// It contains the rain forecast for the next hour.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct RainResponse {
    pub position: Place,
    pub updated_on: u64,
    pub quality: u8,
    pub forecast: Vec<Forecast>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub dt: u64,
    pub rain: u8,
    pub desc: String,
}
