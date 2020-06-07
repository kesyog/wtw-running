use crate::error::OutfitHandlerError;
use crate::location;
use alexa_sdk::{
    request::IntentType,
    response::{Card, Speech},
    Request, Response,
};
use anyhow::anyhow;
use log::{error, info, warn};
use picker::{
    gear::Outfit,
    inputs::{Intensity, RunParameters, Sex, UserPreferences},
    weather,
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

fn outfit_to_speech(outfit: &Outfit) -> Result<String, OutfitHandlerError> {
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

    let sex = req
        .body
        .intent
        .as_ref()
        .and_then(|intent| intent.slots.as_ref())
        .and_then(|slots| slots.get("sex"))
        .and_then(|slot| slot.resolutions.as_ref())
        .and_then(|resolutions| resolutions.resolutions_per_authority.get(0))
        .and_then(|rpa| rpa.values.get(0))
        .map(|value_wrapper| &value_wrapper.value.id)
        .map(|id| {
            if id == "female" {
                Sex::Female
            } else {
                if id != "male" {
                    warn!("unknown slot id for sex: {}", id);
                }
                Sex::Male
            }
        })
        .unwrap_or(Sex::Male);

    let intensity = match req.intent() {
        IntentType::User(name) => match name.as_str() {
            "GetOutfitLongRun" => Intensity::LongRun,
            "GetOutfitRace" => Intensity::Race,
            "GetOutfitWorkout" => Intensity::Workout,
            _ => Intensity::Average,
        },
        _ => Intensity::Average,
    };

    let preferences = UserPreferences {
        sex,
        intensity,
        ..UserPreferences::default()
    };

    info!("{:?}", preferences);

    let speech: Result<String, OutfitHandlerError> = location::get(req)
        .and_then(|loc| weather::get_current(&owm_api_key, &loc).map_err(|e| e.into()))
        .and_then(|conditions| {
            Outfit::new(&RunParameters::new(&conditions, &preferences)).map_err(|e| e.into())
        })
        .and_then(|outfit| outfit_to_speech(&outfit));

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
