use crate::location;
use alexa_sdk::{Request, Response};
use log::{error, info};
use picker::{
    gear::Outfit,
    inputs::{RunParameters, UserPreferences},
    weather,
    weather::Conditions,
};
use std::fmt::Write;

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

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

fn get_outfit(conditions: &Conditions) -> Result<String, Error> {
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

// TODO: this should return an error type that gets mapped to a response at a higher level
pub fn handler(req: &Request) -> Result<Response, Error> {
    let owm_api_key = std::env::var("OWM_API_KEY").expect("No OpenWeatherMap API key provided");

    let loc = location::get_location(&req);
    if let Err(e) = loc {
        info!("Could not get location: {}", e);
        // TODO: return a AskForPermissionsConsent card
        return Ok(Response::simple("🌐 No location found", "I couldn't figure out where you are. Please enable location permissions for this skill in the Alexa app"));
    }
    let loc = loc.unwrap();

    let conditions = weather::get_current(&owm_api_key, &loc).or_else(|e| {
        error!("Could not get weather for {:?}", &loc);
        Err(e)
    })?;

    let speech = get_outfit(&conditions)
        .or_else::<Error, _>(|_| Ok("I had trouble finding you an outfit".to_string()))?;
    info!("Recommending outfit: {}", speech);
    Ok(Response::simple("Outfit", &speech))
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
