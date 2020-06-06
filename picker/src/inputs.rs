use super::weather::{Conditions, TimeOfDay, Weather, Wind};
use std::fmt;

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Default, Debug, Clone)]
pub struct UserPreferences {
    pub sex: Sex,
    pub intensity: Intensity,
    pub feel: Feel,
}

impl fmt::Display for UserPreferences {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?} running @ {:?} intensity", self.sex, self.intensity)?;
        match &self.feel {
            Feel::Average => Ok(()),
            _ => write!(f, "{:?}", self.feel),
        }
    }
}

#[derive(Debug)]
pub struct RunParameters {
    pub conditions: Conditions,
    pub preferences: UserPreferences,
    effective_temperature: i16,
}

impl RunParameters {
    pub fn new(conditions: &Conditions, preferences: &UserPreferences) -> RunParameters {
        let effective_temperature = Self::calculate_effective_temperature(conditions, preferences);
        RunParameters {
            conditions: conditions.clone(),
            preferences: preferences.clone(),
            effective_temperature,
        }
    }

    pub fn effective_temperature(&self) -> i16 {
        self.effective_temperature
    }

    fn calculate_effective_temperature(
        conditions: &Conditions,
        preferences: &UserPreferences,
    ) -> i16 {
        // Adjust for weather conditions
        let weather_adj = match conditions.weather {
            Weather::Snow => -3,
            Weather::Rain => -4,
            Weather::HeavyRain => -10,
            Weather::Overcast => 0,
            Weather::PartlyCloudy => match conditions.time {
                TimeOfDay::Daytime => 5,
                TimeOfDay::Morning | TimeOfDay::Evening => 2,
                TimeOfDay::Night => 0,
            },
            Weather::Clear => match conditions.time {
                TimeOfDay::Daytime => 10,
                TimeOfDay::Morning | TimeOfDay::Evening => 5,
                TimeOfDay::Night => 0,
            },
        };

        // Adjust for wind
        let wind_adj = match conditions.wind {
            Wind::Light => -5,
            Wind::Heavy => -9,
            Wind::Calm => 0,
        };

        // Adjust for intensity
        let intensity_adj = match preferences.intensity {
            Intensity::Race => 15,
            Intensity::Workout => 8,
            Intensity::LongRun => -5,
            Intensity::Average => 0,
        };

        // Adjust for user preference
        let user_adj = match preferences.feel {
            Feel::RunsWarm => 10,
            Feel::RunsCold => -10,
            Feel::Average => 0,
        };

        conditions.temperature + weather_adj + wind_adj + intensity_adj + user_adj
    }
}

impl fmt::Display for RunParameters {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n{}", self.preferences, self.conditions)
    }
}
