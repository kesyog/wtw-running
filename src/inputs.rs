use simple_error::SimpleError;

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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Debug)]
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

#[derive(Default, Debug)]
pub struct Conditions {
    // °F
    temperature: i16,
    // Temperature adjusted for conditions
    adjusted_temperature: i16,
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

#[derive(Default, Debug)]
pub struct UserPreferences {
    pub sex: Sex,
    pub intensity: Intensity,
    pub feel: Feel,
}

#[derive(Default, Debug)]
pub struct RunParameters {
    pub conditions: Conditions,
    pub preferences: UserPreferences,
}

impl RunParameters {
    fn adjust_temperature(&self, actual_temperature: i16) -> i16 {
        // Adjust for weather conditions
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

        actual_temperature + weather_adj + wind_adj + intensity_adj + user_adj
    }

    pub fn set_temperature(&mut self, temperature: i16) {
        self.conditions.temperature = temperature;
        self.conditions.adjusted_temperature = self.adjust_temperature(temperature);
    }

    pub fn get_adjusted_temperature(&self) -> i16 {
        self.conditions.adjusted_temperature
    }
}
