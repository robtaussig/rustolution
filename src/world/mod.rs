mod country;

pub struct World {
    pub size: u32,
    pub countries: Vec<country::Country>
}

impl World {
    pub fn generate_countries(number_of_countries: u32, number_of_villages: u32, number_of_villagers: u32) -> Vec<country::Country> {
        let mut created_countries = Vec::new();
        for _i in 1..number_of_countries {
            let mut created_country = country::Country {
                villages: country::Country::generate_villages(number_of_villages, number_of_villagers)
            };
            created_countries.push(created_country);
        }
        created_countries
    }

    pub fn describe(&self) {
        for ctry in &self.countries {
            for village in &ctry.villages {
                for villager in &village.people {
                    println!("Name: {}", villager.name);
                    println!("Age: {}", villager.age);
                    println!("Genetics: {}", villager.genetics);
                }
            }
        }
    }
}