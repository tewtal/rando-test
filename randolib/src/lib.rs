#![allow(non_snake_case)]

extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate walkdir;
extern crate ffi_utils;

#[macro_use]
extern crate simple_error;

#[macro_use]
extern crate measure_time;

use std::collections::HashSet;
use std::os::raw::{c_char};
use std::ffi::{CStr, CString};
use std::mem::{transmute, forget};
use std::str;

mod node;
mod region;
mod requirement;
mod sparking;
mod link;
mod connection;
mod world;
mod location;
mod weapon;
mod enemy;

#[no_mangle]
pub extern "C" fn createWorld() -> *mut world::World
{
    let newWorld = world::World::load_from("Super Metroid".to_owned(), &"../sm-json-data".to_owned());
    Box::into_raw(Box::new(newWorld.unwrap()))
}

#[no_mangle]
pub extern "C" fn destroyWorld(world: *mut world::World)
{
    unsafe {
        Box::from_raw(world);
    }
}

#[no_mangle]
pub extern "C" fn getLocations(world: &mut world::World, item_str: *const c_char, node_str: *const c_char) -> *mut c_char
{   
    let c_node = unsafe { CStr::from_ptr(node_str).to_bytes() };
    let s_node = str::from_utf8(c_node).unwrap();

    let c_items = unsafe { CStr::from_ptr(item_str).to_bytes() };
    let s_items = str::from_utf8(c_items).unwrap();
    let items: HashSet<String> = s_items.split("|").map(|s| s.to_string()).collect();

    //let start_region = world.regions.iter().find(|r| r.id == region_id).unwrap();
    //let start_node = start_region.nodes.iter().find(|n| n.id == node_id).unwrap();
    
    let start_region: &region::Region;
    let start_node: &node::Node;

    if s_node.contains("|") 
    {
        let parts: Vec<&str> = s_node.split("|").collect();
        start_region = world.regions.iter().find(|r| r.name == parts[0]).unwrap();
        start_node = start_region.nodes.iter().find(|n| n.name == parts[1]).unwrap();
    } else {
        start_region = world.regions.iter().find(|r| r.nodes.iter().any(|n| n.name == s_node)).unwrap();
        start_node = start_region.nodes.iter().find(|n| n.name == s_node).unwrap();    
    }

    let mut locations: Vec<location::Location>;
    locations = location::Location::available(&items, &world, &start_region, &start_node).unwrap_or(Vec::new());
    let location_str = &locations.iter().map(|l| l.name.to_owned()).collect::<Vec<String>>().join("|");
    let location_cstr = CString::new(location_str.as_str()).unwrap();
    location_cstr.into_raw()
}

#[no_mangle]
pub extern "C" fn freeLocations(locations: *mut c_char)
{
    unsafe {
        CString::from_raw(locations);
    }
}

pub fn randomize() -> Result<(), Box<std::error::Error>>
{
    println!("Loading world...");
    let world = world::World::load_from("Super Metroid".to_owned(), &"../sm-json-data".to_owned())?;
    let start_region = world.regions.iter().find(|r| r.id == 8).unwrap();
    let start_node = start_region.nodes.iter().find(|n| n.id == 5).unwrap();
    let items: HashSet<String> = 
        ["Morph", "Missile", "Bombs", "Super", "canGateGlitch", "canOpenWrongFacingBlueGateFromRight", "canUseMorphBombs", "canPassBombPassages", "canDestroyBombWalls", "canBombThings", "canOpenEyeDoors", "canOpenGreenDoors", "canOpenRedDoors", "canBombAboveIBJ", "canIBJ", "canMockball", "canTrickyJump", "canWalljump", "canManipulateHitbox", "canUseEnemies", "canTrickyWalljump"]
        .iter().map(|s| s.to_string()).collect();
    let mut locations: Vec<location::Location>;
    
    {
        print!("\nTraversing the world with: {:?}\n", &items);
        print_time!("Graph traversal");
        locations = location::Location::available(&items, &world, &start_region, &start_node).unwrap_or(Vec::new());
    }

    print!("\nFound {} locations\n", &locations.len());
    print!("Visited Item Locations:\n");
    for location in &locations
    {
        print!("{}\n", &location.node.name);
    }

    Ok(())
}
