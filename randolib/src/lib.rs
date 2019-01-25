#![allow(non_snake_case)]

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate walkdir;

#[macro_use]
extern crate simple_error;

#[macro_use]
extern crate measure_time;

mod node;
mod region;
mod requirement;
mod sparking;
mod link;
mod connection;
mod world;
mod pathfinder;

pub fn randomize() -> Result<(), Box<std::error::Error>>
{
    print_time!("Execution");
    println!("Loading world...");
    let world = world::World::load_from("Super Metroid".to_owned(), &"C:/Users/Thomas/Documents/Rust/rando/sm-json-data".to_owned())?;
    let start_region = world.regions.iter().find(|r| r.id == 8).unwrap();
    let start_node = start_region.nodes.iter().find(|n| n.id == 5).unwrap();
    let mut locations = pathfinder::Location::generate(&world, &start_region, &start_node).unwrap();
    dbg!(locations.len());
    locations.pop();
    dbg!(locations.pop());
    

    Ok(())
}
