use crate::gear::Outfit;
use crate::weather::Conditions;
use std::result;
use std::time::SystemTimeError;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("The given weather conditions are invalid")]
    InvalidWeather(Conditions),
    #[error("Failed to retrieve weather")]
    FetchWeather(#[from] openweather::Error),
    #[error("Problem with time calculation")]
    SystemTimeError(#[from] SystemTimeError),
    #[error("The generated outfit is invalid")]
    InvalidOutfit(Outfit),
}
