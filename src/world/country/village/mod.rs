mod person;

pub struct Village {
    pub people: Vec<person::Person>
}

impl Village {
    pub fn generate_persons(&mut self, size: u32) {
        for _i in 1..size {
            let mut created_person = person::Person {
                name: person::Person::generate_name(),
                genetics: person::Person::generate_genetics(),
                age: person::Person::generate_age()
            };

            self.people.push(created_person);
        }
    }
}