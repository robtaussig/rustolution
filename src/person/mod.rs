mod genetics;
use self::{genetics::Genetics, genetics::generate_intelligence, genetics::generate_strength};
use rand::{thread_rng, Rng};
use names::{Generator, Name};
use country::{Country};
use world::{World};
use std::error::Error;

pub struct Person {
    pub genetics: Genetics,
    pub name: String,
    pub age: u32,
    pub village_location: u32,
    pub country: u32,
    pub village: u32
}

impl Person {
    pub fn describe(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Village Location: {}", self.village_location);
        self.genetics.describe();
    }
}

pub fn generate_person_name() -> String {
    let mut generator = Generator::with_naming(Name::Plain);
    generator.next().unwrap()
}

pub fn generate_genetics() -> Genetics {
    Genetics {
        intelligence: generate_intelligence(),
        strength: generate_strength()
    }
}

pub fn generate_age() -> u32 {
    let mut rng = thread_rng();
    rng.gen_range(1, 60)
}