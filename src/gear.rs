use super::weather::RunParameters;

#[derive(Default, Clone, Copy)]
struct Gear {
    pub name: &'static str,
    max_temp: Option<i16>,
    min_temp: Option<i16>,
}

impl Gear {
    pub fn is_valid_range(&self, temperature: i16) -> bool {
        if let (Some(max_temp), Some(min_temp)) = (self.max_temp, self.min_temp) {
            assert!(max_temp >= min_temp);
        }
        if let Some(max_temp) = self.max_temp {
            if temperature > max_temp {
                return false;
            }
        }
        if let Some(min_temp) = self.min_temp {
            if temperature < min_temp {
                return false;
            }
        }
        true
    }
}

const WINTER_CAP: Gear = Gear {
    name: "Winter cap",
    min_temp: None,
    max_temp: Some(38),
};

const HAT: Gear = Gear {
    name: "Hat with visor",
    min_temp: None,
    max_temp: None,
};

const SUNGLASSES: Gear = Gear {
    name: "Sunglasses",
    min_temp: None,
    max_temp: None,
};

const HEAVY_JACKET: Gear = Gear {
    name: "Heavy jacket",
    min_temp: None,
    max_temp: Some(20),
};

const LIGHT_JACKET: Gear = Gear {
    name: "Light jacket",
    min_temp: Some(20),
    max_temp: Some(40),
};

const VEST: Gear = Gear {
    name: "Vest",
    min_temp: Some(35),
    max_temp: Some(40),
};

const LONG_SLEEVE: Gear = Gear {
    name: "Long-sleeve shirt",
    min_temp: None,
    max_temp: Some(54),
};

const SHORT_SLEEVE: Gear = Gear {
    name: "Short-sleeve shirt",
    min_temp: Some(54),
    max_temp: Some(65),
};

const SINGLET: Gear = Gear {
    name: "Singlet",
    min_temp: Some(65),
    max_temp: Some(85),
};

const SPORTS_BRA: Gear = Gear {
    name: "Sports bra",
    min_temp: None,
    max_temp: None,
};

const TOPLESS: Gear = Gear {
    name: "Topless",
    min_temp: Some(85),
    max_temp: None,
};

const TIGHTS: Gear = Gear {
    name: "Tights",
    min_temp: None,
    max_temp: Some(40),
};

const CAPRIS: Gear = Gear {
    name: "Capri tights",
    min_temp: Some(35),
    max_temp: Some(50),
};

const SHORTS: Gear = Gear {
    name: "Shorts",
    min_temp: Some(40),
    max_temp: None,
};

const GLOVES: Gear = Gear {
    name: "Gloves",
    min_temp: None,
    max_temp: Some(77),
};

const SHOES: Gear = Gear {
    name: "Running shoes",
    min_temp: None,
    max_temp: None,
};

const SUNBLOCK: Gear = Gear {
    name: "Sunblock",
    min_temp: None,
    max_temp: None,
};

fn pick_from_list<'a>(choices: Vec<&'a Gear>, params: &RunParameters) -> Vec<&'a str> {
    choices
        .iter()
        .filter(|&x| x.is_valid_range(params.conditions.temperature))
        .map(|&x| x.name)
        .collect()
}

#[derive(Debug)]
pub struct Outfit {
    pub head: String,
    pub torso: String,
    pub legs: String,
    pub feet: String,
    pub accessories: String,
}

pub fn make_picks(params: RunParameters) -> Outfit {
    let head = vec![&WINTER_CAP, &HAT];
    let torso = vec![
        &HEAVY_JACKET,
        &LIGHT_JACKET,
        &VEST,
        &LONG_SLEEVE,
        &SHORT_SLEEVE,
        &SINGLET,
        &SPORTS_BRA,
        &TOPLESS,
    ];
    let legs = vec![
        &TIGHTS,
        &CAPRIS,
        &SHORTS,
    ];
    let feet = vec![
        &SHOES,
    ];
    let accessories = vec![
        &GLOVES,
        &SUNGLASSES,
        &SUNBLOCK,
    ];

    Outfit {
        head: pick_from_list(head, &params).join(", "),
        torso: pick_from_list(torso, &params).join(", "),
        legs: pick_from_list(legs, &params).join(", "),
        feet: pick_from_list(feet, &params).join(", "),
        accessories: pick_from_list(accessories, &params).join(", "),
    }
}
