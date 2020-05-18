mod gear;
mod inputs;
mod weather;

use openweather::LocationSpecifier;
use inputs::{Feel, Intensity, Sex};
use simple_error::SimpleError;
use weather::{Conditions, TimeOfDay, Weather, Wind};

fn main() -> Result<(), SimpleError> {
    dotenv::dotenv().ok();
    let owm_api_key = dotenv::var("OWM_API_KEY").expect("No OpenWeatherMap API key provided");

    let loc = LocationSpecifier::ZipCode {
        zip: "02144".to_string(),
        country: "US".to_string(),
    };

    let conditions = weather::get_current_weather(&owm_api_key, &loc).unwrap();

    let mut params: inputs::RunParameters = inputs::RunParameters {
        conditions,
        ..Default::default()
    };
    params.adjust_temperature();

    println!("Parameters: {:?}", params);
    let outfit = gear::pick_outfit(&params);
    println!(
        "{}°F Feels like {}°F {:?}",
        params.conditions.temperature,
        params.conditions.adjusted_temperature,
        outfit
    );
    Ok(())
}
