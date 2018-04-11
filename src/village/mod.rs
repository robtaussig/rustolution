use person::{Person, generate_person_name, generate_age, generate_genetics};
use names::{Generator, Name};

pub struct Village {
    pub people: Vec<Person>,
    pub name: String,
    pub country_location: u32
}

impl Village {
    pub fn describe(&self) {
        println!("Village: {}", self.name);
        println!("Country Location: {}", self.country_location);
        println!("----------");
    }
}

pub fn generate_persons(number_of_villagers: u32) -> Vec<Person> {
    let mut created_people = Vec::new();
    for i in 1..number_of_villagers + 1 {
        let mut created_person = Person {
            name: generate_person_name(),
            genetics: generate_genetics(),
            age: generate_age(),
            village_location: i
        };

        created_people.push(created_person);
    }
    created_people
}

pub fn generate_village_name() -> String {
    let mut generator = Generator::with_naming(Name::Plain);
    generator.next().unwrap()
}