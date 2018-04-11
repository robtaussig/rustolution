mod person;
use names::{Generator, Name};

pub struct Village {
    pub people: Vec<person::Person>,
    pub name: String,
    pub country_location: u32
}

impl Village {
    pub fn generate_persons(number_of_villagers: u32) -> Vec<person::Person> {
        let mut created_people = Vec::new();
        for i in 1..number_of_villagers + 1 {
            let mut created_person = person::Person {
                name: person::Person::generate_name(),
                genetics: person::Person::generate_genetics(),
                age: person::Person::generate_age(),
                village_location: i
            };

            created_people.push(created_person);
        }
        created_people
    }

    pub fn generate_name() -> String {
        let mut generator = Generator::with_naming(Name::Plain);
        generator.next().unwrap()
    }
}