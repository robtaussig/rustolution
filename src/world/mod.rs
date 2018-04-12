use country::{Country, generate_villages, generate_environment, generate_country_name};
use std::collections::HashMap;

pub struct World {
    pub world_size: u32,
    pub countries: HashMap<u32, Country>
}

impl World {    
    pub fn describe(&mut self) {
        for (_, ctry) in self.countries.iter() {
            ctry.describe();
            println!("----------");
            for (_, village) in ctry.villages.iter() {
                village.describe();
                println!("----------");
                for (_, person) in village.people.iter() {
                    person.describe();
                    println!("----------");
                }
            }
        }
    }
}

pub fn generate_countries(number_of_countries: u32, number_of_villages: u32, number_of_villagers: u32) -> HashMap<u32, Country> {
    let mut created_countries = HashMap::new();
    for i in 1..number_of_countries + 1 {
        let mut created_country = Country {
            villages: generate_villages(number_of_villages, number_of_villagers),
            environment: generate_environment(),
            name: generate_country_name(),
            world_location: i
        };
        created_countries.insert(i, created_country);
    }
    created_countries
}