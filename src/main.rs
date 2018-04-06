use std::fmt;
extern crate rand;

use rand::Rng;

enum Gender {
    Male,
    Female
}

impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let printable = match *self {
            Gender::Male => "Male",
            Gender::Female => "Female"
        };
        write!(f, "{}", printable)
    }
}

struct PersonTraits {
    intelligence: u32,
    strength: u32
}

impl fmt::Display for PersonTraits {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {        
        write!(f, "Intelligence = {}, Strength = {}", self.intelligence, self.strength)
    }
}

impl PersonTraits {
    pub fn generate_intelligence() -> u32 {
        rand::thread_rng().gen_range(1, 10)
    }

    pub fn generate_strength() -> u32 {
        rand::thread_rng().gen_range(1, 10)
    }
}

struct Person {
    name: String,
    gender: Gender,
    traits: PersonTraits,
    age: u32
}

impl Person {
    pub fn generate_name() -> String {
        let num = rand::thread_rng().gen_range(1, 10);
        let name = match num {
            0...2 => String::from("Rob"),
            3...5 => String::from("Andrea"),
            6...8 => String::from("Joey"),
            _ => String::from("Aaron")
        };
        name
    }

    pub fn generate_gender() -> Gender {
        let mut rng = rand::thread_rng();
        if rng.gen() {
            Gender::Male
        } else {
            Gender::Female
        }
    }

    pub fn generate_traits() -> PersonTraits {
        let intelligence = PersonTraits::generate_intelligence();
        let strength = PersonTraits::generate_strength();
        let person_trait = PersonTraits {
            intelligence: intelligence,
            strength: strength    
        };
        person_trait
    }

    pub fn generate_age() -> u32 {
        rand::thread_rng().gen_range(1, 60)
    }
}

struct Village {
    villagers: Vec<Person>
}

impl Village {
    pub fn generate_persons(&mut self, size: u32) {
        for _i in 1..size {
            
            let name = Person::generate_name();
            let gender = Person::generate_gender();
            let traits = Person::generate_traits();
            let age = Person::generate_age();

            let mut person = Person {
                name: name,
                gender: gender,
                traits: traits,
                age: age
            };

            self.villagers.push(person);
        }
    }
}

enum Weather {
    Hot,
    Warm,
    Cold,
    Freezing
}

struct Country {
    villages: Vec<Village>,
    weather: Weather
}

impl Country {
    pub fn generate_villages(&mut self, size: u32) {
        for _i in 1..size {
            let mut village = Village {
                villagers: Vec::new()
            };
            village.generate_persons(5);
            self.villages.push(village);
        }
    }
}

struct World {
    countries: Vec<Country>,
    size: u32
}

impl World {
    fn generate_countries(&mut self, size: u32) {
        for _i in 1..size {
            let mut country = Country {
                villages: Vec::new(),
                weather: Weather::Hot
            };
            country.generate_villages(5);
            self.countries.push(country);
        }
    }

    fn describe(&self) {
        for country in &self.countries {
            for village in &country.villages {
                for villager in &village.villagers {
                    println!("Name: {}", villager.name);
                    println!("Age: {}", villager.age);
                    println!("Gender: {}", villager.gender);
                    println!("Traits: {}", villager.traits);
                }
            }
        }
    }
}

fn main() {
    let mut world = World { 
        countries: Vec::new(),
        size: 5
    };
    world.generate_countries(5);
    world.describe();
}
