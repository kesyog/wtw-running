mod gear;
mod inputs;

use simple_error::SimpleError;
use inputs::{Intensity, Weather, Wind, Sex};

fn main() -> Result<(), SimpleError> {
    let mut params: inputs::RunParameters = Default::default();

    println!("Parameters: {:?}", params);

    for i in (0..100).step_by(5) {
        params.set_temperature(i);
        params.conditions.validate()?;
        let outfit = gear::pick_outfit(&params);
        println!(
            "{}°F Feels like {}°F {:?}",
            i,
            params.get_adjusted_temperature(),
            outfit
        );
    }
    Ok(())
}
