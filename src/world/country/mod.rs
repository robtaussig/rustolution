mod village;

pub struct Country {
    pub villages: Vec<village::Village>
}

impl Country {
    pub fn generate_villages(number_of_villages: u32, number_of_villagers: u32) -> Vec<village::Village> {
        let mut created_villages = Vec::new();
        for _i in 1..number_of_villages {
            let mut created_village = village::Village {
                people: village::Village::generate_persons(number_of_villagers)
            };
            created_villages.push(created_village);
        }
        created_villages
    }
}