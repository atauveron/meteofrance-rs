//! A rust client for MétéoFrance's private API
mod constants;
mod model;

use std::time::Duration;

use model::ForecastResponse;

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

    pub fn get_whatever(&self, latitude: f32, longitude: f32) -> Result<ForecastResponse, String> {
        let target = format!(
            "{}/forecast?token={}&lat={}&lon={}",
            constants::METEOFRANCE_API_URL,
            self.token,
            latitude,
            longitude
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
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_whatever() {
        let client = MeteoFranceClient::new();
        let result = client.get_whatever(48.85, 2.35).unwrap();
        println!("Weather forecast\n---\n{:#?}\n---\n", result);
    }
}
