use crate::inputs::{Intensity, RunParameters, Sex};
use crate::weather::{TimeOfDay, Weather};
use crate::{Error, Result};
use std::fmt;

#[derive(Default, Clone, Copy)]
struct Gear {
    name: &'static str,
    max_temp: Option<i16>,
    min_temp: Option<i16>,
    // Function should return true if gear is acceptable
    other_checks: Option<fn(&Self, &RunParameters) -> bool>,
}

impl Gear {
    fn is_wearable(&self, params: &RunParameters) -> bool {
        if let (Some(max_temp), Some(min_temp)) = (self.max_temp, self.min_temp) {
            debug_assert!(max_temp >= min_temp);
        }
        // Check if current temperature is within acceptable range for this gear
        let effective_temperature = params.effective_temperature();
        if let Some(max_temp) = self.max_temp {
            if effective_temperature > max_temp {
                return false;
            }
        }
        if let Some(min_temp) = self.min_temp {
            if effective_temperature < min_temp {
                return false;
            }
        }
        // Apply optional additional checks
        if let Some(other_checks) = self.other_checks {
            return other_checks(self, params);
        }
        true
    }
}

// Gear options
const WINTER_CAP: Gear = Gear {
    name: "a winter cap",
    min_temp: None,
    max_temp: Some(38),
    other_checks: Some(disallow_heavy_rain),
};

const HAT: Gear = Gear {
    name: "a hat with visor",
    min_temp: None,
    max_temp: None,
    other_checks: Some(require_rain),
};

const SUNGLASSES: Gear = Gear {
    name: "sunglasses",
    min_temp: None,
    max_temp: None,
    other_checks: Some(require_sun),
};

const HEAVY_JACKET: Gear = Gear {
    name: "a heavy jacket",
    min_temp: None,
    max_temp: Some(20),
    other_checks: None,
};

const LIGHT_JACKET: Gear = Gear {
    name: "a light jacket",
    min_temp: Some(21),
    max_temp: Some(35),
    other_checks: None,
};

const VEST: Gear = Gear {
    name: "a vest",
    min_temp: Some(36),
    max_temp: Some(40),
    other_checks: None,
};

const LONG_SLEEVE: Gear = Gear {
    name: "a long-sleeved shirt",
    min_temp: None,
    max_temp: Some(54),
    other_checks: None,
};

const SHORT_SLEEVE: Gear = Gear {
    name: "a short-sleeved shirt",
    min_temp: Some(55),
    max_temp: Some(65),
    other_checks: None,
};

const SINGLET: Gear = Gear {
    name: "a sleeveless shirt",
    min_temp: Some(66),
    max_temp: Some(85),
    other_checks: Some(check_lower_heat_threshold_for_males),
};

const SPORTS_BRA: Gear = Gear {
    name: "a sports bra",
    min_temp: None,
    max_temp: None,
    other_checks: Some(require_female),
};

const TOPLESS: Gear = Gear {
    name: "no shirt",
    min_temp: Some(81),
    max_temp: None,
    other_checks: Some(require_male),
};

const TIGHTS: Gear = Gear {
    name: "tights",
    min_temp: None,
    max_temp: Some(40),
    other_checks: None,
};

const CAPRIS: Gear = Gear {
    name: "capri tights",
    min_temp: Some(41),
    max_temp: Some(50),
    other_checks: Some(require_female),
};

const SHORTS: Gear = Gear {
    name: "shorts",
    min_temp: Some(40),
    max_temp: None,
    other_checks: None,
};

const GLOVES: Gear = Gear {
    name: "gloves",
    min_temp: None,
    max_temp: Some(47),
    other_checks: Some(disallow_races),
};

const SHOES: Gear = Gear {
    name: "running shoes",
    min_temp: None,
    max_temp: None,
    other_checks: None,
};

const SUNBLOCK: Gear = Gear {
    name: "sunblock",
    min_temp: None,
    max_temp: None,
    other_checks: Some(require_bright_sun),
};

// Extra checks that can be used for Gear::other_checks field

fn check_lower_heat_threshold_for_males(_gear: &Gear, params: &RunParameters) -> bool {
    match params.preferences.sex {
        Sex::Male => params.effective_temperature() <= 80,
        Sex::Female => true,
    }
}

fn disallow_heavy_rain(_gear: &Gear, params: &RunParameters) -> bool {
    match params.conditions.weather {
        Weather::HeavyRain => false,
        _ => true,
    }
}

fn require_rain(_gear: &Gear, params: &RunParameters) -> bool {
    match params.conditions.weather {
        Weather::HeavyRain | Weather::Rain => true,
        _ => false,
    }
}

fn require_sun(_gear: &Gear, params: &RunParameters) -> bool {
    if let TimeOfDay::Night = params.conditions.time {
        return false;
    }
    match params.conditions.weather {
        Weather::Clear | Weather::PartlyCloudy => true,
        _ => false,
    }
}

fn require_male(_gear: &Gear, params: &RunParameters) -> bool {
    match params.preferences.sex {
        Sex::Male => true,
        Sex::Female => false,
    }
}

fn require_female(_gear: &Gear, params: &RunParameters) -> bool {
    match params.preferences.sex {
        Sex::Male => false,
        Sex::Female => true,
    }
}

fn disallow_races(_gear: &Gear, params: &RunParameters) -> bool {
    match params.preferences.intensity {
        Intensity::Race => false,
        _ => true,
    }
}

fn require_bright_sun(_gear: &Gear, params: &RunParameters) -> bool {
    if let TimeOfDay::Night = params.conditions.time {
        return false;
    }
    match params.conditions.weather {
        Weather::Clear | Weather::PartlyCloudy => true,
        _ => false,
    }
}

fn filter_wearable<'a>(choices: &[&'a Gear], params: &RunParameters) -> Vec<&'a str> {
    choices
        .iter()
        .filter(|x| x.is_wearable(params))
        .map(|x| x.name)
        .collect()
}

#[derive(Debug)]
pub struct Outfit {
    pub head: Vec<&'static str>,
    pub torso: Vec<&'static str>,
    pub legs: Vec<&'static str>,
    pub feet: Vec<&'static str>,
    pub accessories: Vec<&'static str>,
}

impl fmt::Display for Outfit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if !self.head.is_empty() {
            writeln!(f, "{}", self.head.join(", "))?;
        }
        if !self.torso.is_empty() {
            writeln!(f, "{}", self.torso.join(", "))?;
        }
        if !self.legs.is_empty() {
            writeln!(f, "{}", self.legs.join(", "))?;
        }
        if !self.feet.is_empty() {
            writeln!(f, "{}", self.feet.join(", "))?;
        }
        if !self.accessories.is_empty() {
            writeln!(f, "{}", self.accessories.join(", "))?;
        }
        Ok(())
    }
}

impl Outfit {
    pub fn new(params: &RunParameters) -> Result<Self> {
        let head_options = vec![&WINTER_CAP, &HAT];
        let torso_options = vec![
            &HEAVY_JACKET,
            &LIGHT_JACKET,
            &VEST,
            &LONG_SLEEVE,
            &SHORT_SLEEVE,
            &SINGLET,
            &SPORTS_BRA,
            &TOPLESS,
        ];
        let legs_options = vec![&TIGHTS, &CAPRIS, &SHORTS];
        let feet_options = vec![&SHOES];
        let accessories_options = vec![&GLOVES, &SUNGLASSES, &SUNBLOCK];

        let mut outfit = Self {
            head: filter_wearable(&head_options, params),
            torso: filter_wearable(&torso_options, params),
            legs: filter_wearable(&legs_options, params),
            feet: filter_wearable(&feet_options, params),
            accessories: filter_wearable(&accessories_options, params),
        };

        // Special override for males running races
        if let Sex::Male = params.preferences.sex {
            if let Intensity::Race = params.preferences.intensity {
                if params.effective_temperature() > 35 {
                    outfit.torso = vec![SINGLET.name];
                }
            }
        }

        if outfit.torso.is_empty() || outfit.legs.is_empty() || outfit.feet.is_empty() {
            Err(Error::InvalidOutfit(outfit))
        } else {
            Ok(outfit)
        }
    }
}
