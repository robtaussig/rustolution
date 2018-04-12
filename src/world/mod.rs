use country::{Country, generate_villages, generate_environment, generate_country_name};
use linked_hash_map::{LinkedHashMap};

pub struct World {
    pub world_size: u32,
    pub countries: LinkedHashMap<u32, Country>
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
                    let village_location = person.village_location;
                    let country_option = self.countries.get(&person.country);
                    match country_option {
                        Some(e) => {
                            let village_option = e.villages.get(&person.village);
                            match village_option {
                                Some(f) => {
                                    println!("Geographical Location: {}, {}", f.name, e.name);
                                },
                                None => {
                                    println!("Can't find village");
                                }
                            }
                        },
                        None => {
                            println!("Can't find country");
                        }
                    }

                    // let village = matched_country.villages.get(&person.village);
                    // println!("Location: {}, {}, {}", village.name, matched_country.name, village_location);
                    println!("----------");
                }
            }
        }
    }
}

pub fn generate_countries(number_of_countries: u32, number_of_villages: u32, number_of_villagers: u32) -> LinkedHashMap<u32, Country> {
    let mut created_countries = LinkedHashMap::new();
    for i in 1..number_of_countries + 1 {
        let mut created_country = Country {
            villages: generate_villages(number_of_villages, number_of_villagers, i),
            environment: generate_environment(),
            name: generate_country_name(),
            world_location: i
        };
        created_countries.insert(i, created_country);
    }
    created_countries
}