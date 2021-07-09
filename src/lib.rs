//! A rust client for MétéoFrance's private API
mod constants;
mod model;

use std::time::Duration;

use model::forecast::ForecastResponse;
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
    fn test_rain() {
        let client = MeteoFranceClient::new();
        let result = client.get_rain(48.85, 2.35, None).unwrap();
        println!("Rain forecast\n---\n{:#?}\n---\n", result);
    }
}
