//use std::io::Result;
use simple_error::SimpleError;

#[derive(PartialEq)]
enum Weather {
    Clear,
    PartiallyCloudy,
    Overcast,
    Rain,
    HeavyRain,
    Snow,
}

impl Default for Weather {
    fn default() -> Self { Weather::Clear }
}

enum Wind {
    Calm,
    Light,
    Heavy,
}

impl Default for Wind {
    fn default() -> Self { Wind::Calm }
}

enum TimeOfDay {
    Morning,
    Daytime,
    Evening,
    Night,
}

impl Default for TimeOfDay {
    fn default() -> Self { TimeOfDay::Daytime }
}

enum Sex {
    Male,
    Female,
}

impl Default for Sex {
    fn default() -> Self { Sex::Male } // ¯\_(ツ)_/¯ 
}

enum Intensity {
    LongRun,
    Average,
    Workout,
    Race,
}

impl Default for Intensity {
    fn default() -> Self { Intensity::Average }
}

enum Feel {
    RunsWarm,
    Average,
    RunsCold,
}

impl Default for Feel {
    fn default() -> Self { Feel::Average }
}

#[derive(Default)]
struct Conditions {
    temperature: i16,
    weather: Weather,
    wind: Wind,
    time: TimeOfDay,
}

impl Conditions {
    fn validate(&self) -> Result<(), SimpleError> {
        Ok(())
    }
}

#[derive(Default)]
struct UserPreferences {
    sex: Sex,
    intensity: Intensity,
    feel: Feel,
}

#[derive(Default)]
struct RunParameters {
    conditions: Conditions,
    preferences: UserPreferences,
}

fn get_adjusted_temperature(params: RunParameters) -> i16 {
    let temperature = params.conditions.temperature;
    return temperature;
}

fn main() -> Result<(), SimpleError> {
    let mut params: RunParameters = Default::default();
    params.conditions.temperature = 50;
    params.conditions.validate()?;

    let temp = get_adjusted_temperature(params);
    println!("Temperature: {}", temp);
    Ok(())
}


