use std::fmt;
mod weather;
mod land;
use self::{weather::Weather, weather::generate_precipitation, weather::generate_temperature};
use self::{land::Land, land::generate_size, land::generate_type};

pub struct Environment {
    pub weather: Weather,
    pub land: Land
}

impl Environment {
    
}

pub fn generate_weather() -> Weather {
    let created_weather = Weather {
        precipitation: generate_precipitation(),
        temperature: generate_temperature()
    };
    created_weather
}

pub fn generate_land() -> Land {
    let created_land = Land {
        land_size: generate_size(),
        land_type: generate_type()
    };
    created_land
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "Weather: [{}], Land: [{}]", self.weather, self.land)
    }
}