use std::fmt;
mod weather;
mod land;

pub struct Environment {
    pub weather: weather::Weather,
    pub land: land::Land
}

impl Environment {
    pub fn generate_weather() -> weather::Weather {
        let created_weather = weather::Weather {
            precipitation: weather::Weather::generate_precipitation(),
            temperature: weather::Weather::generate_temperature()
        };
        created_weather
    }

    pub fn generate_land() -> land::Land {
        let created_land = land::Land {
            land_size: land::Land::generate_size(),
            land_type: land::Land::generate_type()
        };
        created_land
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "Weather: [{}], Land: [{}]", self.weather, self.land)
    }
}