mod gear;
mod inputs;
mod weather;

use anyhow::Result;
use inputs::{RunParameters, UserPreferences};
use openweather::LocationSpecifier;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let owm_api_key = dotenv::var("OWM_API_KEY").expect("No OpenWeatherMap API key provided");

    let loc = LocationSpecifier::ZipCode {
        zip: "02144".to_string(),
        country: "US".to_string(),
    };

    let conditions = weather::get_current_weather(&owm_api_key, &loc).unwrap();
    let preferences = UserPreferences::default();
    let params = RunParameters::new(conditions, preferences);

    let outfit = gear::pick_outfit(&params);
    println!("\nParameters:\n{}\n\nOutfit:\n{}", params, outfit);
    Ok(())
}
