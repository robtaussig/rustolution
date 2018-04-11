mod country;

pub struct World {
    pub world_size: u32,
    pub countries: Vec<country::Country>
}

impl World {
    pub fn generate_countries(number_of_countries: u32, number_of_villages: u32, number_of_villagers: u32) -> Vec<country::Country> {
        let mut created_countries = Vec::new();
        for i in 1..number_of_countries + 1 {
            let mut created_country = country::Country {
                villages: country::Country::generate_villages(number_of_villages, number_of_villagers),
                environment: country::Country::generate_environment(),
                name: country::Country::generate_name(),
                world_location: i
            };
            created_countries.push(created_country);
        }
        created_countries
    }

    pub fn describe(&self) {
        for ctry in &self.countries {
            println!("Country: {}", ctry.name);
            println!("World Location: {}", ctry.world_location);
            println!("Environment: [{}]", ctry.environment);
            println!("----------");
            for village in &ctry.villages {
                println!("Village: {}", village.name);
                println!("Country Location: {}", village.country_location);
                println!("----------");
                for villager in &village.people {
                    println!("Name: {}", villager.name);
                    println!("Age: {}", villager.age);
                    println!("Genetics: [{}]", villager.genetics);
                    println!("Home: {}, {}", village.name, ctry.name);
                    println!("Village Location: {}", villager.village_location);
                    println!("");
                }
            }
        }
    }
}