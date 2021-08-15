use serde::Deserialize;
use std::fmt;

/// The `Place` struct describes a location.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Place {
    pub lat: f32,
    pub lon: f32,
    pub alti: Option<i16>,
    pub name: String,
    pub country: String,
    pub dept: Option<String>,
    pub rain_product_available: Option<u8>,
    pub timezone: Option<String>,
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
