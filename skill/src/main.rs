use alexa_sdk::request::IntentType;
use alexa_sdk::{Request, Response};
use anyhow::{anyhow, Result};
use lambda::{error::HandlerError, lambda, Context};
use lambda_runtime as lambda;
use log::info;
use openweather::LocationSpecifier;
use picker::{
    gear::Outfit,
    inputs::{RunParameters, UserPreferences},
    weather,
};
use std::error::Error;
use std::fmt::Write;

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

fn get_outfit() -> Result<String> {
    let owm_api_key = std::env::var("OWM_API_KEY")
        .or_else(|_| Err(anyhow!("No OpenWeatherMap API key provided")))?;

    // TODO: get user's location
    let loc = LocationSpecifier::ZipCode {
        zip: "02144".to_string(),
        country: "US".to_string(),
    };

    let conditions = weather::get_current(&owm_api_key, &loc)?;
    let preferences = UserPreferences::default();
    let params = RunParameters::new(conditions, preferences);

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

fn outfit_intent_handler(_req: &Request) -> Result<Response, HandlerError> {
    let speech = get_outfit()
        .or_else::<HandlerError, _>(|_| Ok("I had trouble finding you an outfit".to_string()))?;
    info!("Recommending outfit: {}", speech);
    Ok(Response::simple("Outfit", &speech))
}

fn handle_help(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::simple(
        "Help",
        "Outfit Picker can help you pick a running outfit. Try saying \"find me an outfit\".",
    ))
}

fn handle_cancel(_req: &Request) -> Result<Response, HandlerError> {
    Ok(Response::end())
}

fn my_handler(req: Request, _ctx: Context) -> Result<Response, HandlerError> {
    match req.intent() {
        IntentType::Help => handle_help(&req),
        IntentType::Cancel => handle_cancel(&req),
        IntentType::User(_) => outfit_intent_handler(&req),
        // TODO; handle other Amazon built-in intents
        _ => handle_cancel(&req),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    lambda!(my_handler);

    Ok(())
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
