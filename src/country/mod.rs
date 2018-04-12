use village::{Village, generate_persons, generate_village_name};
use environment::{Environment, generate_weather, generate_land};
use names::{Generator, Name};
use std::collections::HashMap;

pub struct Country {
    pub villages: HashMap<u32, Village>,
    pub environment: Environment,
    pub name: String,
    pub world_location: u32
}

impl Country {
    pub fn describe(&self) {
        println!("Country: {}", self.name);
        println!("World Location: {}", self.world_location);
        println!("Environment: [{}]", self.environment);
    }
}

pub fn generate_villages(number_of_villages: u32, number_of_villagers: u32) -> HashMap<u32, Village> {
    let mut created_villages = HashMap::new();
    for i in 1..number_of_villages + 1 {
        let mut created_village = Village {
            people: generate_persons(number_of_villagers),
            name: generate_village_name(),
            country_location: i
        };
        created_villages.insert(i, created_village);
    }
    created_villages
}

pub fn generate_environment() -> Environment {
    let created_environment = Environment {
        weather: generate_weather(),
        land: generate_land()
    };
    created_environment
}

pub fn generate_country_name() -> String {
    let mut generator = Generator::with_naming(Name::Plain);
    generator.next().unwrap()
}