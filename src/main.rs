mod gear;
mod inputs;
mod weather;

use anyhow::{anyhow, Result};
use gear::Outfit;
use inputs::{RunParameters, UserPreferences};
use openweather::LocationSpecifier;

fn main() -> Result<()> {
    dotenv::dotenv().ok();
    let owm_api_key = dotenv::var("OWM_API_KEY")
        .or_else(|_| Err(anyhow!("No OpenWeatherMap API key provided")))?;

    let loc = LocationSpecifier::ZipCode {
        zip: "02144".to_string(),
        country: "US".to_string(),
    };

    let conditions = weather::get_current(&owm_api_key, &loc)?;
    let preferences = UserPreferences::default();
    let params = RunParameters::new(conditions, preferences);

    let outfit = Outfit::new(&params)?;
    println!("\nParameters:\n{}\n\nOutfit:\n{}", params, outfit);
    Ok(())
}
