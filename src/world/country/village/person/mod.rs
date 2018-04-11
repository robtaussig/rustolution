mod genetics;
use rand::{thread_rng, Rng};
use names::{Generator, Name};

pub struct Person {
    pub genetics: genetics::Genetics,
    pub name: String,
    pub age: u32,
    pub village_location: u32
}

impl Person {
    pub fn generate_name() -> String {
        let mut generator = Generator::with_naming(Name::Plain);
        generator.next().unwrap()
    }

    pub fn generate_genetics() -> genetics::Genetics {
        genetics::Genetics {
            intelligence: genetics::Genetics::generate_intelligence(),
            strength: genetics::Genetics::generate_strength()
        }
    }

    pub fn generate_age() -> u32 {
        let mut rng = thread_rng();
        rng.gen_range(1, 60)
    }
}