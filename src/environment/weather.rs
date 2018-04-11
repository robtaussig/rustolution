use std::fmt;
use rand::{thread_rng, Rng};

pub struct Weather {
    pub precipitation: u32,
    pub temperature: u32
}

impl Weather {
    
}

pub fn generate_precipitation() -> u32 {
    let mut rng = thread_rng(); 
    rng.gen_range(1, 100)
}

pub fn generate_temperature() -> u32 {
    let mut rng = thread_rng(); 
    rng.gen_range(1, 100)
}

impl fmt::Display for Weather {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "Precipitation = {}, Temperature = {}", self.precipitation, self.temperature)
    }
}