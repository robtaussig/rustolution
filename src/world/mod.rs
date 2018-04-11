use country::{Country, generate_villages, generate_environment, generate_country_name};

pub struct World {
    pub world_size: u32,
    pub countries: Vec<Country>
}

impl World {    
    pub fn describe(&self) {
        for ctry in &self.countries {
            ctry.describe();
            println!("----------");
            for village in &ctry.villages {
                village.describe();
                println!("----------");
                for villager in &village.people {
                    villager.describe();
                    println!("");
                }
            }
        }
    }
}

pub fn generate_countries(number_of_countries: u32, number_of_villages: u32, number_of_villagers: u32) -> Vec<Country> {
    let mut created_countries = Vec::new();
    for i in 1..number_of_countries + 1 {
        let mut created_country = Country {
            villages: generate_villages(number_of_villages, number_of_villagers),
            environment: generate_environment(),
            name: generate_country_name(),
            world_location: i
        };
        created_countries.push(created_country);
    }
    created_countries
}