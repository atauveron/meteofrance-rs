use serde::Deserialize;

/// The `ForecastResponse` struct describes the result of a forecast call.
/// It contains the hourly and daily weather forecast.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ForecastResponse {
    pub update_time: String,
    #[serde(rename = "type")]
    pub location_type: String,
    pub geometry: Geometry,
    pub properties: Properties,
}

/// The `Geometry` struct contains information about the location
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Geometry {
    #[serde(rename = "type")]
    pub geometry_type: String,
    pub coordinates: Vec<f32>,
}

/// The `Properties` struct contains information about the location, and the forecast
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Properties {
    pub altitude: Option<i16>,
    pub name: String,
    pub country: String,
    pub french_department: Option<String>,
    pub rain_product_available: Option<u8>,
    pub timezone: Option<String>,
    pub insee: Option<String>,
    pub bulletin_cote: Option<u8>,
    pub daily_forecast: Vec<DailyForecast>,
    pub forecast: Vec<Forecast>,
    pub probability_forecast: Vec<ProbabilityForecast>,
}

/// The `DailyForecast` struct contains forecast information for a given day.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct DailyForecast {
    pub time: String,
    #[serde(rename = "T_min")]
    pub t_min: Option<f32>,
    #[serde(rename = "T_max")]
    pub t_max: Option<f32>,
    #[serde(rename = "T_sea")]
    pub t_sea: Option<f32>,
    pub relative_humidity_min: Option<u8>,
    pub relative_humidity_max: Option<u8>,
    pub total_precipitation_24h: Option<f32>,
    pub uv_index: Option<u8>,
    // TODO Icons
    pub sunrise_time: String,
    pub sunset_time: String,
}

/// The `Forecast` struct contains forecast information.
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Forecast {
    pub time: String,
    #[serde(rename = "T")]
    pub t: Option<f32>,
    #[serde(rename = "T_windchill")]
    pub t_windchill: Option<f32>,
    pub relative_humidity: Option<u8>,
    #[serde(rename = "P_sea")]
    pub p_sea: Option<f32>,
    pub wind_speed: Option<f32>,
    pub wind_speed_gust: Option<f32>,
    pub wind_direction: Option<i32>,
    wind_icon: Option<String>,
    pub rain_1h: Option<f32>,
    pub rain_3h: Option<f32>,
    pub rain_6h: Option<f32>,
    pub rain_12h: Option<f32>,
    pub rain_24h: Option<f32>,
    pub snow_1h: Option<f32>,
    pub snow_3h: Option<f32>,
    pub snow_6h: Option<f32>,
    pub snow_12h: Option<f32>,
    pub snow_24h: Option<f32>,
    pub iso0: Option<u16>,
    // FIXME This field is either an altitude (u16) or string "Non pertinent"
    // #[serde(rename = "rain snow limit")]
    // pub rain_snow_limit: u16,
    pub total_cloud_cover: Option<u8>,
    weather_icon: Option<String>,
    pub weather_description: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ProbabilityForecast {
    pub time: String,
    pub rain_hazard_3h: Option<f32>,
    pub rain_hazard_6h: Option<f32>,
    pub snow_hazard_3h: Option<f32>,
    pub snown_hazard_6h: Option<f32>,
    pub freezing_hazard: Option<f32>,
    pub storm_hazard: Option<f32>,
}
