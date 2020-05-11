extern crate randolib;
use std::time::Instant;

fn main() {
    let items = 
        vec!["Morph", "Missile", "Bombs", "Super", "PowerBomb"];
    
    let techs =
        vec!["canWalljump", "canTrickyWalljump", "canMidAirMorph", "canCWJ"];

    let start_location = "Morphing Ball";

    println!("\nTraversing world from: {:?}\nUsing items: {:?}\nUsing techs: {:?}\n", &start_location, &items, &techs);

    println!("Finding suitable locations for item placement...");

    let mut now = Instant::now();
    let world = randolib::load_world("Super Metroid", "../sm-json-data");
    if world.is_err() {
        eprintln!("{:?}", world);
        return;
    }

    let world = world.unwrap();

    println!("Loaded world in: {:?}", now.elapsed());

    let possible_locations;
    {
        now = Instant::now();

        let start_node = world.regions.iter().flat_map(|r| &r.nodes).find(|n| n.name == start_location).unwrap();
        let start_region = world.regions.iter().find(|r| r.nodes.contains(&start_node)).unwrap();
        
        let game_flags = world.get_flags(&items, &techs);
        possible_locations = randolib::location::Location::available(&game_flags, &world, start_region, start_node).unwrap();

        println!("Traversed world graph in: {:?}", now.elapsed());
        println!("Found {:?} possible locations:\n{:?}\n", &possible_locations.len(), &possible_locations.iter().map(|p| p.name.as_ref()).collect::<Vec<&str>>());
    }
}
