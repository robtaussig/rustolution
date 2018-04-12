use person::{Person, generate_person_name, generate_age, generate_genetics};
use names::{Generator, Name};
use linked_hash_map::{LinkedHashMap};

pub struct Village {
    pub people: LinkedHashMap<u32, Person>,
    pub name: String,
    pub country: u32,
    pub country_location: u32
}

impl Village {
    pub fn describe(&self) {
        println!("Village: {}", self.name);
        println!("Country Location: {}", self.country_location);
        println!("----------");
    }
}

pub fn generate_persons(number_of_villagers: u32, country: u32, village: u32 ) -> LinkedHashMap<u32, Person> {
    let mut created_people = LinkedHashMap::new();
    for i in 1..number_of_villagers + 1 {
        let mut created_person = Person {
            name: generate_person_name(),
            genetics: generate_genetics(),
            age: generate_age(),
            village_location: i,
            country: country,
            village: village
        };

        created_people.insert(i, created_person);
    }
    created_people
}

pub fn generate_village_name() -> String {
    let mut generator = Generator::with_naming(Name::Plain);
    generator.next().unwrap()
}