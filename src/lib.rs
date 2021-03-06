//! A rust client for MétéoFrance's private API
#![forbid(unsafe_code)]

mod constants;
mod model;

use std::time::Duration;

use model::forecast::ForecastResponse;
use model::forecast_v2::ForecastResponse as ForecastResponseV2;
use model::place::Place;
use model::rain::RainResponse;

pub struct MeteoFranceClient {
    token: String,
}

impl Default for MeteoFranceClient {
    fn default() -> Self {
        MeteoFranceClient {
            token: String::from(constants::METEOFRANCE_API_TOKEN),
        }
    }
}

impl MeteoFranceClient {
    pub fn new() -> MeteoFranceClient {
        Default::default()
    }

    pub fn with_token(token: String) -> MeteoFranceClient {
        MeteoFranceClient { token }
    }

    /// Retrieve the weather forecast for a given location.
    pub fn get_forecast(
        &self,
        latitude: f32,
        longitude: f32,
        lang: Option<Language>,
    ) -> Result<ForecastResponse, String> {
        let target = format!(
            "{}/forecast?token={}&lat={}&lon={}&lang={}",
            constants::METEOFRANCE_API_URL,
            self.token,
            latitude,
            longitude,
            match lang.unwrap_or_default() {
                Language::French => "fr",
                Language::English => "en",
            }
        );
        let response = ureq::get(&target)
            .timeout(Duration::from_secs(10))
            .call()
            .unwrap();
        if response.status() < 200 || response.status() > 299 {
            return Err(format!("Request failed: {}", response.status_text()));
        }
        let forecast = response.into_json::<ForecastResponse>();
        match forecast {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Retrieve the weather forecast for a given location (using V2 API).
    pub fn get_forecast_v2(
        &self,
        latitude: f32,
        longitude: f32,
        lang: Option<Language>,
    ) -> Result<ForecastResponseV2, String> {
        let target = format!(
            "{}/v2/forecast?token={}&lat={}&lon={}&lang={}",
            constants::METEOFRANCE_API_URL,
            self.token,
            latitude,
            longitude,
            match lang.unwrap_or_default() {
                Language::French => "fr",
                Language::English => "en",
            }
        );
        let response = ureq::get(&target)
            .timeout(Duration::from_secs(10))
            .call()
            .unwrap();
        if response.status() < 200 || response.status() > 299 {
            return Err(format!("Request failed: {}", response.status_text()));
        }
        let forecast = response.into_json::<ForecastResponseV2>();
        match forecast {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Retrieve the rain forecast for a given location, for the next hour.
    pub fn get_rain(
        &self,
        latitude: f32,
        longitude: f32,
        lang: Option<Language>,
    ) -> Result<RainResponse, String> {
        let target = format!(
            "{}/rain?token={}&lat={}&lon={}&lang={}",
            constants::METEOFRANCE_API_URL,
            self.token,
            latitude,
            longitude,
            match lang.unwrap_or_default() {
                Language::French => "fr",
                Language::English => "en",
            }
        );
        let response = ureq::get(&target)
            .timeout(Duration::from_secs(10))
            .call()
            .unwrap();
        if response.status() < 200 || response.status() > 299 {
            return Err(format!("Request failed: {}", response.status_text()));
        }
        let forecast = response.into_json::<RainResponse>();
        // let forecast = response.into_string();
        match forecast {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Search the places (cities) linked to a query by name
    /// Add GPS coordinates in parameter to search places around a given location.
    pub fn search_places(
        &self,
        query: &str,
        latitude: Option<f32>,
        longitude: Option<f32>,
    ) -> Result<Vec<Place>, String> {
        let mut target = format!(
            "{}/places?token={}&q={}",
            constants::METEOFRANCE_API_URL,
            self.token,
            query
        );
        match latitude {
            Some(lat) => target.push_str(&format!("&lat={}", lat)),
            None => (),
        };
        match longitude {
            Some(lon) => target.push_str(&format!("&lon={}", lon)),
            None => (),
        };
        let response = ureq::get(&target)
            .timeout(Duration::from_secs(10))
            .call()
            .unwrap();
        if response.status() < 200 || response.status() > 299 {
            return Err(format!("Request failed: {}", response.status_text()));
        }
        // let places = response.into_string();
        let places = response.into_json::<Vec<Place>>();
        // let forecast = response.into_string();
        match places {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }
}

pub enum Language {
    French,
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::French
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_forecast() {
        let client = MeteoFranceClient::new();
        let result = client.get_forecast(48.85, 2.35, None).unwrap();
        println!("Weather forecast\n---\n{:#?}\n---\n", result);
    }
    #[test]
    fn test_forecast_v2() {
        let client = MeteoFranceClient::new();
        let result = client.get_forecast_v2(48.85, 2.35, None).unwrap();
        println!("Weather forecast\n---\n{:#?}\n---\n", result);
    }
    #[test]
    fn test_rain() {
        let client = MeteoFranceClient::new();
        let result = client.get_rain(48.85, 2.35, None).unwrap();
        println!("Rain forecast\n---\n{:#?}\n---\n", result);
    }
    #[test]
    fn test_places_name() {
        let client = MeteoFranceClient::new();
        let result = client.search_places("Paris", None, None).unwrap();
        println!("Places search for \"Paris\"\n---\n{:#?}\n---\n", result);
    }
    #[test]
    fn test_places_lat_lon() {
        let client = MeteoFranceClient::new();
        let result = client
            .search_places("Paris", Some(48.85), Some(2.35))
            .unwrap();
        println!(
            "Places search for \"Paris\" near 48.85, 2.35\n---\n{:#?}\n---\n",
            result
        );
    }
}
