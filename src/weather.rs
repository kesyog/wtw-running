use anyhow::{anyhow, Context, Result};
use openweather::{Language, LocationSpecifier, Settings, Unit, WeatherReportCurrent};
use std::fmt;
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

// TODO: pretty print
impl fmt::Display for Conditions {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}°F @ {:?}\n{:?} with {:?} wind",
            self.temperature, self.time, self.weather, self.wind
        )
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
        match weather_item.id {
            600..=699 => return Weather::Snow,
            500 | 501 | 520 => return Weather::Rain,
            #[allow(overlapping_patterns)]
            500..=599 => return Weather::HeavyRain,
            _ => (),
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

pub fn get_current(owm_api_key: &str, loc: &LocationSpecifier) -> Result<Conditions> {
    let settings: Settings = Settings {
        unit: Some(Unit::Imperial),
        lang: Some(Language::English),
    };

    let weather = openweather::get_current_weather(loc, owm_api_key, &settings)
        .with_context(|| "Failed to fetch the weather")?;
    let forecast_time = UNIX_EPOCH + Duration::from_secs(weather.dt);
    let freshness_sec = SystemTime::now().duration_since(forecast_time)?.as_secs();

    // TODO: what loggers are out there?
    println!(
        "Fetched OpenWeatherMap data from {} minutes ago",
        freshness_sec / 60
    );

    let conditions: Conditions = Conditions {
        temperature: weather.main.temp.round() as i16,
        wind: resolve_wind(&weather),
        time: resolve_time_of_day(&weather),
        weather: resolve_weather(&weather),
    };
    conditions.validate()?;
    Ok(conditions)
}
