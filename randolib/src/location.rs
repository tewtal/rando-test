use crate::region::Region;
use crate::node::{Node, NodeType};
use crate::world::World;
use crate::link::{LinkTo, Strat};
use std::collections::{HashSet};

#[derive(Debug, Clone, PartialEq)]
pub struct State
{
    pub events: HashSet<String>,
    pub obstacles: HashSet<(i64, String)>,
    pub nodes: HashSet<(i64, i64)>,
    pub start: (i64, i64),
    pub backtracking: bool
}

impl State
{
    pub fn new(start_region: i64, start_node: i64) -> State
    {
        State
        {
            events: HashSet::new(),
            obstacles: HashSet::new(),
            nodes: HashSet::new(),
            start: (start_region, start_node),
            backtracking: false
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location<'a>
{
    pub name: String,
    pub region: &'a Region,
    pub node: &'a Node,
}

impl<'a> Location<'a>
{

    pub fn available(items: &HashSet<String>, world: &'a World, from_region: &'a Region, from_node: &'a Node) -> Option<Vec<Location<'a>>>
    {
        let mut state = State::new(from_region.id, from_node.id);
        let mut locations = Vec::new();
        let mut oldEvents = 0;
        let mut pass = 1;
        
        while pass == 1 || state.events.len() > oldEvents {
            oldEvents = state.events.len();
            pass += 1;
            locations = Location::visit_node(items, world, from_region, from_node, from_node, &mut state).unwrap_or_default();
            state.nodes = HashSet::new();
        }

        Some(locations)
    }

    pub fn available_with_state(items: &HashSet<String>, world: &'a World, from_region: &'a Region, from_node: &'a Node, state: &mut State) -> Option<Vec<Location<'a>>>
    {        
        Some(Location::visit_node(items, world, from_region, from_node, from_node, state).unwrap_or_default())
    }

    fn visit_node(items: &HashSet<String>, world: &'a World, region: &'a Region, node: &'a Node, _prevNode: &'a Node, state: &mut State) -> Option<Vec<Location<'a>>>
    {
        //print!("Visiting {}: {}\n", &region.name, &node.name);
        state.nodes.insert((region.id, node.id));
        if let Some(yields) = &node.yields
        {
            if Location::can_unlock(items, world, region, node, state) {
                state.events.extend(yields.iter().map(|y| y.to_string()));
            }
            //dbg!(&state.events);
        }

        let mut locations: Vec<Location> = Vec::new();

        if let Some(nodeType) = &node.nodeType
        {
            /* If it's an item, add it to our item location collection */
            if nodeType == &NodeType::Item && Location::can_unlock(items, world, region, node, state) {
                if !state.backtracking {
                    let (start_region, start_node) = state.start;
                    let mut backtrack_state = state.clone();
                    backtrack_state.nodes = HashSet::new();
                    backtrack_state.obstacles = state.obstacles.iter().filter(|(o, _)| o == &region.id).cloned().collect();
                    backtrack_state.start = (region.id, node.id);
                    backtrack_state.backtracking = true;
                    if let Some(backtrack_locations) = Location::available_with_state(items, world, region, node, &mut backtrack_state) {
                        if backtrack_locations.iter().any(|l| l.region.id == start_region && l.node.id == start_node) {
                            let location = Location
                            {
                                name: node.name.to_string(),
                                region,
                                node,
                            };
        
                            locations.push(location);    
                        }
                    }
                } else {
                    let location = Location
                    {
                        name: node.name.to_string(),
                        region,
                        node,
                    };

                    locations.push(location);    
                }
            }

            /* If it's a door or exit, find the connecting node and visit it */
            if (nodeType == &NodeType::Door || nodeType == &NodeType::Exit) && Location::can_unlock(items, world, region, node, state) {
                if let Some(connection) = world.connections.iter().find(|c| c.nodes.iter().any(|cn| cn.roomid == region.id && cn.nodeid == node.id)) {
                    if let Some(connection_node) = connection.nodes.iter().find(|cn| !(cn.roomid == region.id && cn.nodeid == node.id)) {
                        if let Some(target_region) = world.regions.iter().find(|r| r.id == connection_node.roomid) {
                            if let Some(target_node) = target_region.nodes.iter().find(|n| n.id == connection_node.nodeid) {
                                if !state.nodes.contains(&(target_region.id, target_node.id)) {
                                    if let Some(mut new_locations) = Location::visit_node(items, world, target_region, target_node, node, state) {
                                        locations.append(&mut new_locations);
                                    }
                                }
                            }
                        }
                    }
                }             
            }
        }

        /* Find the in-room links for this node */
        if let Some(links) = region.links.iter().find(|l| l.from == node.id) {
            for link in &links.to {
                if let Some(link_node) = region.nodes.iter().find(|n| n.id == link.id) {
                    if !state.nodes.contains(&(region.id, link_node.id)) && Location::can_traverse(items, world, region, &link, state) && Location::can_access(items, world, region, &link_node, state) {
                        if let Some(mut new_locations) = Location::visit_node(items, world, region, link_node, node, state) {
                            locations.append(&mut new_locations);
                        }
                    }                     
                }
            }
        }
        
        if !locations.is_empty() {
            Some(locations)
        } else {
            None
        }
    }

    fn can_unlock(items: &HashSet<String>, world: &World, region: &Region, node: &Node, state: &mut State) -> bool {
        match &node.locks {
            Some(locks) => locks.iter().all(|lock|
                match &lock.lock {
                    Some(_r) => true,
                    None => 
                        (match &lock.unlockStrats {
                            Some(us) => us.iter().any(|s| Location::can_do_strat(items, world, region, s, state)),
                            None => true
                        } ||
                        match &lock.bypassStrats {
                            Some(bs) => bs.iter().any(|s| Location::can_do_strat(items, world, region, s, state)),
                            None => false
                        })                    
                }
            ),
            None => true
        }
    }

    fn can_access(items: &HashSet<String>, world: &World, _region: &Region, node: &Node, state: &mut State) -> bool {
        match &node.interactionRequires {
            Some(r) => r.check(items, world, state),
            None => true
        }
    }

    fn can_traverse(items: &HashSet<String>, world: &World, region: &Region, link: &LinkTo, state: &mut State) -> bool {        
        match &link.strats {
            Some(strats) => strats.iter().any(|s| Location::can_do_strat(items, world, region, s, state)),
            None => true
        }
    }

    fn can_do_strat(items: &HashSet<String>, world: &World, region: &Region, strat: &Strat, state: &mut State) -> bool {
        let requires = match &strat.requires {
            Some(r) => r.check(items, world, state),
            None => true
        };

        let obstacles = match &strat.obstacles {
            Some(ob) => {
                ob.iter().all(|o| state.obstacles.contains(&(region.id, o.id.as_ref().unwrap().to_string())) ||
                    (match &o.requires {
                        Some(r) => {
                            if r.check(items, world, state) {
                                state.obstacles.insert((region.id, o.id.as_ref().unwrap().to_string())) && true
                            } else {
                                false
                            }
                        },
                        None => true
                    } || 
                    match &o.bypass {
                        Some(b) => b.check(items, world, state),
                        None => false
                    })
                )                                
            },
            None => true
        };

        requires && obstacles
    }
}