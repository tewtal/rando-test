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
    let items: HashSet<String> = ["DefeatGoldenTorizo", "canTrickyJump", "canCrumbleJump", "canMultiviolaClip", "canXRayClimb", "canSpringBallJumpMidAir", "canXRayClimb", "DefeatSporeSpawn", "DefeatCrocomire", "UsedAcidChozoStatue", "canGGG", "ShaktoolDoneDigging", "Bomb", "canUseFrozenEnemies", "canGravityJump", "canSnailClip", "HiJump", "Space Jump", "canUseSpringBall","SpringBall","Wave", "canSuitlessMaridia", "Plasma", "GravitY", "Charge", "Grapple", "DefeatBotwoon", "DefeatDraygon", "DefeatKraid", "DefeatRidley", "ScrewAttack", "canWalljump", "canHeatRun", "heatProof", "MaridiaTubeBroken", "canNavigateUnderwater", "canOpenEyeDoors", "DefeatPhantoon", "canNavigateHeatRooms", "SpaceJump", "canUsePowerBombs", "SpeedBooster", "Ice", "Gravity", "Varia", "PowerBomb", "canOpenYellowDoors", "Super", "canOpenGreenDoors", "canDamageBoost", "Morph", "Missile", "canOpenRedDoors", "ZebesAwake", "Bombs", "canUseMorphBombs", "canPassBombPassages", "canDestroyBombWalls", "canMockball", "canTrickyWalljump", "canFly", "canIBJ"].iter().map(|s| s.to_string()).collect();
    let mut locations: Vec<location::Location>;
    
    {
        print!("\nTraversing the world with: {:?}\n", &items);
        print_time!("Graph traversal");
        locations = location::Location::available(&items, &world, &start_region, &start_node).unwrap();    
    }

    print!("\nFound {} locations\n", &locations.len());
    // print!("Visited Item Locations:\n");
    // for location in &locations
    // {
    //     print!("{}\n", &location.node.name);
    // }

    Ok(())
}
