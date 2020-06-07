use alexa_sdk::Request;
use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OutfitHandlerError {
    #[error("Error picking an outfit")]
    OutfitPickerError(#[from] picker::Error),
    #[error("Formatting error")]
    FormattingError(#[from] fmt::Error),
    #[error("Request from Alexa was in an unexpected format")]
    InvalidAlexaRequest(Request),
    #[error("Don't have required permissions to access location data")]
    NoLocationPermissions,
    #[error("Failed to make request")]
    ConnectionError(#[from] reqwest::Error),
    #[error("Parsing failure")]
    ParseError(#[from] serde_json::Error),
}
