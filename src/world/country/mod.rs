mod village;
mod environment;
use names::{Generator, Name};

pub struct Country {
    pub villages: Vec<village::Village>,
    pub environment: environment::Environment,
    pub name: String,
    pub world_location: u32
}

impl Country {
    pub fn generate_villages(number_of_villages: u32, number_of_villagers: u32) -> Vec<village::Village> {
        let mut created_villages = Vec::new();
        for i in 1..number_of_villages + 1 {
            let mut created_village = village::Village {
                people: village::Village::generate_persons(number_of_villagers),
                name: village::Village::generate_name(),
                country_location: i
            };
            created_villages.push(created_village);
        }
        created_villages
    }

    pub fn generate_environment() -> environment::Environment {
        let created_environment = environment::Environment {
            weather: environment::Environment::generate_weather(),
            land: environment::Environment::generate_land()
        };
        created_environment
    }

    pub fn generate_name() -> String {
        let mut generator = Generator::with_naming(Name::Plain);
        generator.next().unwrap()
    }
}