use std::fmt;
use rand::{thread_rng, Rng};

pub struct Land {
    pub land_size: u32,
    pub land_type: String
}

impl Land {
    
}

pub fn generate_size() -> u32 {
    let mut rng = thread_rng(); 
    rng.gen_range(1, 100)
}

pub fn generate_type() -> String {
    let mut rng = thread_rng(); 
    let land_type: u32 = rng.gen_range(1, 5);
    match land_type {
        1 => String::from("Desert"),
        2 => String::from("Jungle"),
        3 => String::from("Temperate"),
        4 => String::from("Oceanic"),
        5 => String::from("Icy"),
        _ => String::from("Other")
    }
}

impl fmt::Display for Land {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "Land Size = {}, Land Type = {}", self.land_size, self.land_type)
    }
}