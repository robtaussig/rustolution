use std::fmt;
use rand::{thread_rng, Rng};

pub struct Genetics {
    pub strength: u32,
    pub intelligence: u32
}

impl Genetics {
    pub fn describe(&self) {
        println!("Intelligence = {}, Strength = {}", self.intelligence, self.strength);
    }
}

pub fn generate_intelligence() -> u32 {
    let mut rng = thread_rng(); 
    rng.gen_range(1, 10)
}

pub fn generate_strength() -> u32 {
    let mut rng = thread_rng(); 
    rng.gen_range(1, 10)
}