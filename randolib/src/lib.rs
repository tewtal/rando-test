#![allow(non_snake_case)]

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate walkdir;
extern crate ffi_utils;

#[macro_use]
extern crate simple_error;

pub mod node;
pub mod region;
pub mod requirement;
pub mod sparking;
pub mod link;
pub mod connection;
pub mod world;
pub mod location;
pub mod weapon;
pub mod enemy;
pub mod helper;
pub mod tech;

pub fn load_world(name: &str, path: &str) -> Result<world::World, Box<dyn std::error::Error>>
{
    let world = world::World::load_from(name, path)?;
    Ok(world)
}
