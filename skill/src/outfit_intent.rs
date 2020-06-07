use crate::error::OutfitHandlerError;
use crate::location;
use alexa_sdk::{
    response::{Card, Speech},
    Request, Response,
};
use anyhow::anyhow;
use log::{error, info};
use picker::{
    gear::Outfit,
    inputs::{RunParameters, UserPreferences},
    weather,
    weather::Conditions,
};
use std::fmt::Write;

const INSUFFICIENT_LOCATION_PERMISSION_TEXT: &str =
    "I couldn't figure out your current location. Please enable the location services permission \
    for this skill in the Alexa app";
const FETCH_WEATHER_ERROR_TEXT: &str =
    "I had an issue retrieving weather data for your location. Please try again later.";
const FETCH_WEATHER_ERROR_TITLE: &str = "No weather data";

// Flatten a collection of strings into a single string, adding in commas as necessary, and adding
// "and" before the last item.
fn join_english_list(items: &[&str]) -> String {
    match items.len() {
        0 => String::new(),
        1 => items[0].to_string(),
        2 => items.join(" and "),
        len => format!("{}, and {}", items[..(len - 1)].join(", "), items[len - 1]),
    }
}

fn get_outfit(conditions: &Conditions) -> Result<String, OutfitHandlerError> {
    let preferences = UserPreferences::default();
    let params = RunParameters::new(conditions, &preferences);

    let outfit = Outfit::new(&params)?;
    let mut speech = String::new();
    if !outfit.torso.is_empty() || !outfit.legs.is_empty() {
        write!(&mut speech, "You should wear ")?;
        write!(
            &mut speech,
            "{}",
            join_english_list(
                &outfit
                    .torso
                    .iter()
                    .chain(outfit.legs.iter())
                    .copied()
                    .collect::<Vec<&str>>()
            )
        )?;
        write!(&mut speech, ". ")?;
    }
    if !outfit.head.is_empty() {
        write!(&mut speech, "On your head, you should wear ")?;
        write!(&mut speech, "{}", join_english_list(&outfit.head))?;
        write!(&mut speech, ". ")?;
    }
    if !outfit.accessories.is_empty() {
        write!(&mut speech, "Don't forget ")?;
        write!(&mut speech, "{}", join_english_list(&outfit.accessories))?;
        write!(&mut speech, "!")?;
    }
    Ok(speech.trim().to_string())
}

pub fn handler(req: &Request) -> anyhow::Result<Response> {
    let owm_api_key =
        std::env::var("OWM_API_KEY").map_err(|_| anyhow!("No OpenWeatherMap API key provided"))?;

    let speech: Result<String, OutfitHandlerError> = location::get(req)
        .and_then(|loc| weather::get_current(&owm_api_key, &loc).map_err(|e| e.into()))
        .and_then(|conditions| get_outfit(&conditions));

    match speech {
        Ok(speech) => {
            info!("Recommending outfit: {}", speech);
            Ok(Response::simple("Outfit", &speech))
        }
        Err(OutfitHandlerError::NoLocationPermissions) => Ok(Response::end()
            .speech(Speech::plain(INSUFFICIENT_LOCATION_PERMISSION_TEXT))
            // Ask user for location permissions
            .card(Card::ask_for_permission(vec![
                "read::alexa:device:all:address:country_and_postal_code".to_string(),
                "alexa::devices:all:geolocation:read".to_string(),
            ]))),
        Err(OutfitHandlerError::OutfitPickerError(picker::Error::FetchWeather(e))) => {
            error!("{}", e);
            Ok(Response::simple(
                FETCH_WEATHER_ERROR_TITLE,
                FETCH_WEATHER_ERROR_TEXT,
            ))
        }
        Err(e) => {
            error!("{}", e);
            Err(e.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_join() {
        assert_eq!("", join_english_list(&[]));
        assert_eq!("apples", join_english_list(&["apples"]));
        assert_eq!(
            "apples and oranges",
            join_english_list(&["apples", "oranges"])
        );
        assert_eq!(
            "apples, oranges, and pears",
            join_english_list(&["apples", "oranges", "pears"])
        );
    }
}
