extern crate rand;
extern crate names;
mod world;

const WORLD_SIZE: u32 = 5;
const NUMBER_OF_COUNTRIES_PER_WORLD: u32 = 20;
const NUMBER_OF_VILLAGES_PER_COUNTRY: u32 = 20;
const NUMBER_OF_VILLAGERS_PER_VILLAGE: u32 = 20;

fn main() {
    let created_world = world::World {
        world_size: WORLD_SIZE,
        countries: world::World::generate_countries(NUMBER_OF_COUNTRIES_PER_WORLD, NUMBER_OF_VILLAGES_PER_COUNTRY, NUMBER_OF_VILLAGERS_PER_VILLAGE)
    };
    created_world.describe();
}
