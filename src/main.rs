extern crate rand;
mod world;

fn main() {
    let mut created_world = world::World {
        size: 5,
        countries: Vec::new()
    };
    created_world.generate_countries(5);
    created_world.describe();
}
