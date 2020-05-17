mod gear;
mod weather;

use simple_error::SimpleError;
use weather::Weather;

fn main() -> Result<(), SimpleError> {
    let mut params: weather::RunParameters = Default::default();
    params.conditions.weather = Weather::Rain;
    params.conditions.temperature = 30;
    params.conditions.validate()?;

    let temp = params.get_adjusted_temperature();
    println!("Real temperature {}", params.conditions.temperature);
    println!("Adjusted temperature: {}", temp);
    let outfit = gear::make_picks(params);
    println!("Found outfit: {:?}", outfit);
    Ok(())
}
