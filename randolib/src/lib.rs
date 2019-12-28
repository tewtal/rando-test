#![allow(non_snake_case)]

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate walkdir;

#[macro_use]
extern crate simple_error;

#[macro_use]
extern crate measure_time;

use std::collections::HashSet;

mod node;
mod region;
mod requirement;
mod sparking;
mod link;
mod connection;
mod world;
mod location;

pub fn randomize() -> Result<(), Box<std::error::Error>>
{
    println!("Loading world...");
    let world = world::World::load_from("Super Metroid".to_owned(), &"../sm-json-data".to_owned())?;
    let start_region = world.regions.iter().find(|r| r.id == 8).unwrap();
    let start_node = start_region.nodes.iter().find(|n| n.id == 5).unwrap();
    let items: HashSet<String> = ["canWalljump", "Missile", "Super", "Morph", "Bombs", "canOpenRedDoors", "canOpenGreenDoors", "canOpenYellowDoors", "canUsePowerBombs", "canMockball", "canDestroyBombWalls", "canPassBombPassages"].iter().map(|s| s.to_string()).collect();
    let mut locations: Vec<location::Location>;
    
    {
        print!("\nTraversing the world with: {:?}\n", &items);
        print_time!("Graph traversal");
        locations = location::Location::available(&items, &world, &start_region, &start_node).unwrap();    
    }

    print!("\nFound {} locations\n", &locations.len());
    print!("Visited Item Locations:\n");
    for location in &locations
    {
        print!("{}\n", &location.node.name);
    }

    Ok(())
}
