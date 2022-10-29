//! A rust client for MétéoFrance's private API
#![forbid(unsafe_code)]

mod constants;

#[cfg(feature = "ureq")]
pub mod client;
#[cfg(feature = "ureq")]
pub use client::*;

pub mod model;

pub mod url;
pub use url::*;
