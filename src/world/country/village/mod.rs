mod person;

pub struct Village {
    pub people: Vec<person::Person>
}

impl Village {
    pub fn generate_persons(number_of_villagers: u32) -> Vec<person::Person> {
        let mut created_people = Vec::new();
        for _i in 1..number_of_villagers {
            let mut created_person = person::Person {
                name: person::Person::generate_name(),
                genetics: person::Person::generate_genetics(),
                age: person::Person::generate_age()
            };

            created_people.push(created_person);
        }
        created_people
    }
}