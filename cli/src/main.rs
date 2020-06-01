use anyhow::{anyhow, Result};
use openweather::LocationSpecifier;
use picker::{
    gear::Outfit,
    inputs::{RunParameters, UserPreferences},
    weather,
};

fn main() -> Result<()> {
    let owm_api_key = get_owm_key()?;

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

#[cfg(feature = "dotenv_key")]
// Get OpenWeatherMap
fn get_owm_key() -> Result<String> {
    dotenv::dotenv().ok();
    dotenv::var("OWM_API_KEY").or_else(|_| Err(anyhow!("No OpenWeatherMap API key provided")))
}

#[cfg(not(feature = "dotenv_key"))]
fn get_owm_key() -> Result<String> {
    std::env::var("OWM_API_KEY").or_else(|_| Err(anyhow!("No OpenWeatherMap API key provided")))
}
