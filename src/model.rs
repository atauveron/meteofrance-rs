//! Structures to model the API responses
//!
//! Modeling is not comprehensive, some fields are be missing.
//! Moreover, there may exist some unhandled edge-cases which will lead to panics while deserializing.

pub mod forecast;
pub mod forecast_v2;
pub mod place;
pub mod rain;
