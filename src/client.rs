//! A simple wrapper for performing HTTP requests using `ureq`

use std::time::Duration;

use crate::constants::*;
use crate::model::forecast::ForecastResponse;
use crate::model::forecast_v2::ForecastResponse as ForecastResponseV2;
use crate::model::place::Place;
use crate::model::rain::RainResponse;
use crate::url::*;

pub struct MeteoFranceClient {
    token: String,
}

impl Default for MeteoFranceClient {
    fn default() -> Self {
        MeteoFranceClient {
            token: String::from(METEOFRANCE_API_TOKEN),
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

    /// Retrieve the weather forecast at a given location.
    pub fn get_forecast(
        &self,
        latitude: f32,
        longitude: f32,
        lang: Option<Language>,
    ) -> Result<ForecastResponse, String> {
        let target = forecast_url(latitude, longitude, lang, Some(&self.token));
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(10)))
            .build()
            .into();
        let response = agent.get(&target).call().unwrap();
        if !response.status().is_success() {
            return Err(format!("Request failed: {}", response.status().as_str()));
        }
        let forecast = response.into_body().read_json::<ForecastResponse>();
        match forecast {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Retrieve the weather forecast at a given location (using V2 API).
    pub fn get_forecast_v2(
        &self,
        latitude: f32,
        longitude: f32,
        lang: Option<Language>,
    ) -> Result<ForecastResponseV2, String> {
        let target = forecast_v2_url(latitude, longitude, lang, Some(&self.token));
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(10)))
            .build()
            .into();
        let response = agent.get(&target).call().unwrap();
        if !response.status().is_success() {
            return Err(format!("Request failed: {}", response.status().as_str()));
        }
        let forecast = response.into_body().read_json::<ForecastResponseV2>();
        match forecast {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Retrieve the rain forecast at a given location, for the next hour.
    pub fn get_rain(
        &self,
        latitude: f32,
        longitude: f32,
        lang: Option<Language>,
    ) -> Result<RainResponse, String> {
        let target = rain_url(latitude, longitude, lang, Some(&self.token));
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(10)))
            .build()
            .into();
        let response = agent.get(&target).call().unwrap();
        if !response.status().is_success() {
            return Err(format!("Request failed: {}", response.status().as_str()));
        }
        let forecast = response.into_body().read_json::<RainResponse>();
        // let forecast = response.into_string();
        match forecast {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }

    /// Search the places (cities) linked to a query by name.
    /// Add GPS coordinates in parameter to search places around a given location.
    pub fn search_places(
        &self,
        query: &str,
        latitude: Option<f32>,
        longitude: Option<f32>,
    ) -> Result<Vec<Place>, String> {
        let target = search_places_url(query, latitude, longitude, Some(&self.token));
        println!("Target is {}", target);
        let agent: ureq::Agent = ureq::Agent::config_builder()
            .timeout_global(Some(Duration::from_secs(10)))
            .build()
            .into();
        let response = agent.get(&target).call().unwrap();
        if !response.status().is_success() {
            return Err(format!("Request failed: {}", response.status().as_str()));
        }
        // let places = response.into_string();
        let places = response.into_body().read_json::<Vec<Place>>();
        // let forecast = response.into_string();
        match places {
            Ok(json) => Ok(json),
            Err(err) => Err(err.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_forecast() {
        let client = MeteoFranceClient::new();
        // Marseille
        let result = client.get_forecast(43.30, 5.37, None).unwrap();
        println!("Weather forecast for Marseille\n---\n{:#?}\n---\n", result);
        // Paris
        let result = client.get_forecast(48.85, 2.35, None).unwrap();
        println!("Weather forecast for Paris\n---\n{:#?}\n---\n", result);
        // Ouessant
        let result = client.get_forecast(48.47, -5.10, None).unwrap();
        println!("Weather forecast for Ouessant\n---\n{:#?}\n---\n", result);
        // Briançon
        let result = client.get_forecast(44.88, 6.63, None).unwrap();
        println!("Weather forecast for Briançon\n---\n{:#?}\n---\n", result);
        // Aurillac
        let result = client.get_forecast(44.93, 2.44, None).unwrap();
        println!("Weather forecast for Aurillac\n---\n{:#?}\n---\n", result);
    }

    #[test]
    fn test_forecast_v2() {
        let client = MeteoFranceClient::new();
        // Marseille
        let result = client.get_forecast_v2(43.30, 5.37, None).unwrap();
        println!("Weather forecast for Marseille\n---\n{:#?}\n---\n", result);
        // Paris
        let result = client.get_forecast_v2(48.85, 2.35, None).unwrap();
        println!("Weather forecast for Paris\n---\n{:#?}\n---\n", result);
        // Ouessant
        let result = client.get_forecast_v2(48.47, -5.10, None).unwrap();
        println!("Weather forecast for Ouessant\n---\n{:#?}\n---\n", result);
        // Briançon
        let result = client.get_forecast_v2(44.88, 6.63, None).unwrap();
        println!("Weather forecast for Briançon\n---\n{:#?}\n---\n", result);
        // Aurillac
        let result = client.get_forecast_v2(44.93, 2.44, None).unwrap();
        println!("Weather forecast for Aurillac\n---\n{:#?}\n---\n", result);
    }

    #[test]
    fn test_rain() {
        let client = MeteoFranceClient::new();
        // Marseille
        let result = client.get_rain(43.30, 5.37, None).unwrap();
        println!("Rain forecast for Marseille\n---\n{:#?}\n---\n", result);
        // Paris
        let result = client.get_rain(48.85, 2.35, None).unwrap();
        println!("Rain forecast for Paris\n---\n{:#?}\n---\n", result);
        // Ouessant
        let result = client.get_rain(48.47, -5.10, None).unwrap();
        println!("Rain forecast for Ouessant\n---\n{:#?}\n---\n", result);
        // Briançon
        let result = client.get_rain(44.88, 6.63, None).unwrap();
        println!("Rain forecast for Briançon\n---\n{:#?}\n---\n", result);
        // Aurillac
        let result = client.get_rain(44.93, 2.44, None).unwrap();
        println!("Rain forecast for Aurillac\n---\n{:#?}\n---\n", result);
    }

    #[test]
    fn test_places_name() {
        let client = MeteoFranceClient::new();
        // Paris
        let result = client.search_places("Paris", None, None).unwrap();
        println!("Places search for \"Paris\"\n---\n{:#?}\n---\n", result);
        // Ouessant
        let result = client.search_places("Ouessant", None, None).unwrap();
        println!("Places search for \"Ouessant\"\n---\n{:#?}\n---\n", result);
        // Briançon
        let result = client.search_places("Briançon", None, None).unwrap();
        println!("Places search for \"Briançon\"\n---\n{:#?}\n---\n", result);

        // Saint-Denis (dash in name)
        let result = client.search_places("Saint-Denis", None, None).unwrap();
        println!(
            "Places search for \"Saint-Denis\"\n---\n{:#?}\n---\n",
            result
        );
        // Le Bourget (space in name)
        let result = client.search_places("Le Bourget", None, None).unwrap();
        println!(
            "Places search for \"Le Bourget\"\n---\n{:#?}\n---\n",
            result
        );
    }

    #[test]
    fn test_places_lat_lon() {
        let client = MeteoFranceClient::new();
        // Paris
        let result = client
            .search_places("Paris", Some(48.85), Some(2.35))
            .unwrap();
        println!(
            "Places search for \"Paris\" near 48.85, 2.35\n---\n{:#?}\n---\n",
            result
        );
        // Ouessant
        let result = client
            .search_places("Ouessant", Some(48.47), Some(-5.10))
            .unwrap();
        println!(
            "Places search for \"Ouessant\" near 48.47, -5.10\n---\n{:#?}\n---\n",
            result
        );
        // Briançon
        let result = client
            .search_places("Briançon", Some(44.88), Some(6.63))
            .unwrap();
        println!(
            "Places search for \"Briançon\" near 48.47, -5.10\n---\n{:#?}\n---\n",
            result
        );
    }

    #[test]
    fn test_places_error() {
        let client = MeteoFranceClient::new();
        // Unknown name
        let result = client.search_places("not-a-town-name", None, None).unwrap();
        assert!(result.is_empty());
        println!(
            "Places search for \"not-a-town-name\"\n---\n{:#?}\n---\n",
            result
        );
        // Unknown name
        let result = client.search_places("Not a town name", None, None).unwrap();
        assert!(result.is_empty());
        println!(
            "Places search for \"Not a town name\"\n---\n{:#?}\n---\n",
            result
        );
        // Search for Paris in the Atlantic Ocean
        let result = client
            .search_places("Paris", Some(50.00), Some(-30.00))
            .unwrap();
        println!(
            "Places search for \"Paris\" near 50.00, -30.00\n---\n{:#?}\n---\n",
            result
        );
    }
}
