use anyhow::{anyhow, Result};
use openweather::{Language, LocationSpecifier, Settings, Unit, WeatherReportCurrent};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

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
    fn validate(&self) -> Result<()> {
        match self.weather {
            Weather::Rain | Weather::HeavyRain => {
                if self.temperature < 30 {
                    return Err(anyhow!("It's too cold for rain"));
                }
            }
            Weather::Snow => {
                if self.temperature > 45 {
                    return Err(anyhow!("It's too warm for snow"));
                }
            }
            _ => (),
        };
        Ok(())
    }
}

fn resolve_wind(weather: &WeatherReportCurrent) -> Wind {
    let wind_speed = weather.wind.speed;
    if wind_speed < 8.0 {
        Wind::Calm
    } else if wind_speed < 17.0 {
        Wind::Light
    } else {
        Wind::Heavy
    }
}

fn resolve_time_of_day(weather: &WeatherReportCurrent) -> TimeOfDay {
    // Rough heuristic that ignores higher latitudes
    // Morning lasts from an hour before sunrise to two hours after sunrise
    if weather.dt + 3600 > weather.sys.sunrise && weather.dt < weather.sys.sunrise + 2 * 3600 {
        return TimeOfDay::Morning;
    }
    // Daytime is from sunrise + 2H to sunset - 1H
    if weather.dt >= weather.sys.sunrise + 2 * 3600 && weather.dt + 3600 < weather.sys.sunset {
        return TimeOfDay::Daytime;
    }
    // Evening is within an hour of sunset
    if weather.dt + 3600 >= weather.sys.sunset && weather.dt < weather.sys.sunset + 3600 {
        return TimeOfDay::Evening;
    }

    TimeOfDay::Night
}

fn resolve_weather(weather: &WeatherReportCurrent) -> Weather {
    // TODO: replace magic numbers

    // Check for precipitation
    for weather_item in &weather.weather {
        if weather_item.id / 100 == 6 {
            return Weather::Snow;
        } else if weather_item.id / 100 == 5 {
            // Check if weather id matches OpenWeatherMap light rain types
            if [500, 501, 520].contains(&weather_item.id) {
                return Weather::Rain;
            } else {
                return Weather::HeavyRain;
            }
        }
    }
    // No precipitation. Can use % cloud cover to determine weather
    if weather.clouds.all > 75 {
        Weather::Overcast
    } else if weather.clouds.all > 25 {
        Weather::PartlyCloudy
    } else {
        Weather::Clear
    }
}

pub fn get_current_weather(owm_api_key: &str, loc: &LocationSpecifier) -> Result<Conditions> {
    let settings: Settings = Settings {
        unit: Some(Unit::Imperial),
        lang: Some(Language::English),
    };

    let weather = openweather::get_current_weather(loc, owm_api_key, &settings)
        .expect("Problem fetching current weather");
    let forecast_time = UNIX_EPOCH + Duration::from_secs(weather.dt);
    let freshness_sec = SystemTime::now()
        .duration_since(forecast_time)
        .unwrap()
        .as_secs();
    println!(
        "As of {} minutes ago, in Somerville, MA it is {:0.0}°F",
        freshness_sec / 60,
        weather.main.temp
    );
    for weather_item in &weather.weather {
        println!(
            "Weather: {}: {}",
            weather_item.main, weather_item.description
        );
    }

    let conditions: Conditions = Conditions {
        temperature: weather.main.temp as i16,
        adjusted_temperature: weather.main.temp as i16,
        wind: resolve_wind(&weather),
        time: resolve_time_of_day(&weather),
        weather: resolve_weather(&weather),
    };
    conditions.validate()?;
    Ok(conditions)
}
