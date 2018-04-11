mod genetics;
use rand::{thread_rng, Rng};

pub struct Person {
    pub genetics: genetics::Genetics,
    pub name: String,
    pub age: u32
}

impl Person {
    pub fn generate_name() -> String {
        let mut rng = thread_rng();        
        match rng.gen_range(1, 10) {
            0...2 => String::from("Rob"),
            3...5 => String::from("Andrea"),
            6...8 => String::from("Joey"),
            _ => String::from("Aaron")
        }
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