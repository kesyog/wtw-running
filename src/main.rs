mod gear;
mod weather;

use simple_error::SimpleError;
use weather::{Intensity, Sky};

fn main() -> Result<(), SimpleError> {
    let mut params: weather::RunParameters = Default::default();
    params.conditions.sky = Sky::Clear;
    params.preferences.intensity = Intensity::Race;

    println!("Parameters: {:?}", params);

    for i in (0..100).step_by(5) {
        params.conditions.temperature = i;
        params.conditions.validate()?;
        let adjusted_temp = params.get_adjusted_temperature();
        let outfit = gear::pick_outfit(&params);
        println!("{}°F Feels like {}°F {:?}", i, adjusted_temp, outfit);
    }
    Ok(())
}
