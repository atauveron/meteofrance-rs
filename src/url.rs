//! Functions for building URLs for various APIs
//!
//! These APIs appear to expect GET requests, all parameters are encoded in the URL itself.

use urlencoding::encode;

use crate::constants::*;

/// Build the URL for the weather forecast at a given location.
pub fn forecast_url(
    latitude: f32,
    longitude: f32,
    lang: Option<Language>,
    token: Option<&str>,
) -> String {
    format!(
        "{}/forecast?token={}&lat={}&lon={}&lang={}",
        METEOFRANCE_API_URL,
        match token {
            None => METEOFRANCE_API_TOKEN,
            Some(t) => t,
        },
        latitude,
        longitude,
        match lang.unwrap_or_default() {
            Language::French => "fr",
            Language::English => "en",
        }
    )
}

/// Build the URL for the weather forecast at a given location (using V2 API).
pub fn forecast_v2_url(
    latitude: f32,
    longitude: f32,
    lang: Option<Language>,
    token: Option<&str>,
) -> String {
    format!(
        "{}/v2/forecast?token={}&lat={}&lon={}&lang={}",
        METEOFRANCE_API_URL,
        match token {
            None => METEOFRANCE_API_TOKEN,
            Some(t) => t,
        },
        latitude,
        longitude,
        match lang.unwrap_or_default() {
            Language::French => "fr",
            Language::English => "en",
        }
    )
}

/// Build the URL for the rain forecast at a given location, for the next hour.
pub fn rain_url(
    latitude: f32,
    longitude: f32,
    lang: Option<Language>,
    token: Option<&str>,
) -> String {
    format!(
        "{}/rain?token={}&lat={}&lon={}&lang={}",
        METEOFRANCE_API_URL,
        match token {
            None => METEOFRANCE_API_TOKEN,
            Some(t) => t,
        },
        latitude,
        longitude,
        match lang.unwrap_or_default() {
            Language::French => "fr",
            Language::English => "en",
        }
    )
}

/// Build the URL for searching the places (cities) linked to a query by name.
///
/// Add GPS coordinates in parameter to search places around a given location.
pub fn search_places_url(
    query: &str,
    latitude: Option<f32>,
    longitude: Option<f32>,
    token: Option<&str>,
) -> String {
    let mut url = format!(
        "{}/places?token={}&q={}",
        METEOFRANCE_API_URL,
        match token {
            None => METEOFRANCE_API_TOKEN,
            Some(t) => t,
        },
        encode(query)
    );
    if let Some(lat) = latitude {
        url.push_str(&format!("&lat={}", lat))
    };
    if let Some(lon) = longitude {
        url.push_str(&format!("&lon={}", lon))
    };
    url
}

/// Language (defaults to French)
///
/// This parameter defines the language used for description fields.
#[derive(Default)]
pub enum Language {
    #[default]
    French,
    English,
}
