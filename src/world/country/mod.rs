mod village;

pub struct Country {
    pub villages: Vec<village::Village>
}

impl Country {
    pub fn generate_villages(&mut self, size: u32) {
        for _i in 1..size {
            let mut created_village = village::Village {
                people: Vec::new()
            };
            created_village.generate_persons(5);
            self.villages.push(created_village);
        }
    }
}