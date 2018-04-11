mod country;

pub struct World {
    pub size: u32,
    pub countries: Vec<country::Country>
}

impl World {
    pub fn generate_countries(&mut self, size: u32) {
        for _i in 1..size {
            let mut created_country = country::Country {
                villages: Vec::new()
            };
            created_country.generate_villages(5);
            self.countries.push(created_country);
        }
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