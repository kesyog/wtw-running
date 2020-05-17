use simple_error::SimpleError;

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

pub enum Sex {
    Male,
    Female,
}

impl Default for Sex {
    fn default() -> Self {
        // ¯\_(ツ)_/¯
        Sex::Male
    }
}

pub enum Intensity {
    LongRun,
    Average,
    Workout,
    Race,
}

impl Default for Intensity {
    fn default() -> Self {
        Intensity::Average
    }
}

pub enum Feel {
    RunsWarm,
    Average,
    RunsCold,
}

impl Default for Feel {
    fn default() -> Self {
        Feel::Average
    }
}

#[derive(Default)]
pub struct Conditions {
    pub temperature: i16, // degrees F
    pub weather: Weather,
    pub wind: Wind,
    pub time: TimeOfDay,
}

impl Conditions {
    pub fn validate(&self) -> Result<(), SimpleError> {
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

#[derive(Default)]
pub struct UserPreferences {
    pub sex: Sex,
    pub intensity: Intensity,
    pub feel: Feel,
}

#[derive(Default)]
pub struct RunParameters {
    pub conditions: Conditions,
    pub preferences: UserPreferences,
}

impl RunParameters {
    pub fn get_adjusted_temperature(&self) -> i16 {
        // Adjust for weather
        let weather_adj = match self.conditions.weather {
            Weather::Snow => -3,
            Weather::Rain => -4,
            Weather::HeavyRain => -10,
            Weather::Overcast => 0,
            Weather::PartlyCloudy => match self.conditions.time {
                TimeOfDay::Daytime => 5,
                TimeOfDay::Morning | TimeOfDay::Evening => 2,
                TimeOfDay::Night => 0,
            },
            Weather::Clear => match self.conditions.time {
                TimeOfDay::Daytime => 10,
                TimeOfDay::Morning | TimeOfDay::Evening => 5,
                TimeOfDay::Night => 0,
            },
        };

        // Adjust for wind
        let wind_adj = match self.conditions.wind {
            Wind::Light => -5,
            Wind::Heavy => -9,
            Wind::Calm => 0,
        };

        // Adjust for intensity
        let intensity_adj = match self.preferences.intensity {
            Intensity::Race => 15,
            Intensity::Workout => 8,
            Intensity::LongRun => -5,
            Intensity::Average => 0,
        };

        // Adjust for user preference
        let user_adj = match self.preferences.feel {
            Feel::RunsWarm => 10,
            Feel::RunsCold => -10,
            Feel::Average => 0,
        };

        self.conditions.temperature + weather_adj + wind_adj + intensity_adj + user_adj
    }
}
