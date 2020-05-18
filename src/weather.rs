use simple_error::SimpleError;
use openweather::{Language, LocationSpecifier, Settings, Unit};

#[derive(Debug)]
pub enum Weather {
    Clear,
    PartlyCloudy,
    Overcast,
    Rain,
    HeavyRain,
    Snow,
}

impl Default for Weather {
    fn default() -> Self {
        Weather::Clear
    }
}

#[derive(Debug)]
pub enum Wind {
    Calm,
    Light,
    Heavy,
}

impl Default for Wind {
    fn default() -> Self {
        Wind::Calm
    }
}

#[derive(Debug)]
pub enum TimeOfDay {
    Morning,
    Daytime,
    Evening,
    Night,
}

impl Default for TimeOfDay {
    fn default() -> Self {
        TimeOfDay::Daytime
    }
}

#[derive(Default, Debug)]
pub struct Conditions {
    // Temperature in units (°F)
    pub temperature: i16,
    pub adjusted_temperature: i16,
    pub weather: Weather,
    pub wind: Wind,
    pub time: TimeOfDay,
}

impl Conditions {
    fn validate(&self) -> Result<(), SimpleError> {
        match self.weather {
            Weather::Rain | Weather::HeavyRain => {
                if self.temperature < 30 {
                    return Err(SimpleError::new("It's too cold for rain"));
                }
            }
            Weather::Snow => {
                if self.temperature > 45 {
                    return Err(SimpleError::new("It's too warm for snow"));
                }
            }
            _ => (),
        };
        Ok(())
    }
}

pub fn get_current_weather(owm_api_key: &str, loc: &LocationSpecifier) -> Result<Conditions, SimpleError> {
    let settings: Settings = Settings {
        unit: Some(Unit::Imperial),
        lang: Some(Language::English),
    };

    let weather = openweather::get_current_weather(loc, owm_api_key, &settings).unwrap();
    println!("Right now in Somerville, MA it is {}°F", weather.main.temp);

    let conditions: Conditions = Conditions {
        temperature: weather.main.temp as i16,
        //TODO: resolve other weather conditions beyond temperature
        ..Default::default()
    };
    conditions.validate()?;
    Ok(conditions)
}
