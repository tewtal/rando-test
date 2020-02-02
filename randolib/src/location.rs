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
            start: (start_region, start_node)
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
            locations = Location::visit_node(items, world, from_region, from_node, from_node, &mut state).unwrap_or(Vec::new());
            state.nodes = HashSet::new();
        }

        Some(locations)
    }

    fn visit_node(items: &HashSet<String>, world: &'a World, region: &'a Region, node: &'a Node, prevNode: &'a Node, state: &mut State) -> Option<Vec<Location<'a>>>
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
            if nodeType == &NodeType::Item
            {
                if Location::can_unlock(items, world, region, node, state)
                {
                    //print!("Visiting {}: {}\n", &region.name, &node.name);
                    /* try to backtrack to the starting node from here */
                    // let (start_region, start_node) = state.start;                    
                    // let mut backtrackState: State = State::new(start_region, start_node);
                    // backtrackState.events = state.events.iter().cloned().collect();
                    // backtrackState.obstacles = state.obstacles.iter().cloned().collect();

                    // if Location::backtrack(items, world, region, node, &mut backtrackState)
                    // {
                        let location = Location
                        {
                            name: format!("{}", node.name),
                            region: region,
                            node: node,
                        };

                        locations.push(location);
                    // }
                }
            }

            /* If it's a door or exit, find the connecting node and visit it */
            if nodeType == &NodeType::Door || nodeType == &NodeType::Exit
            {
                if Location::can_unlock(items, world, region, node, state)
                {
                    if let Some(connection) = world.connections.iter().find(|c| c.nodes.iter().any(|cn| cn.roomid == region.id && cn.nodeid == node.id))
                    {
                        if let Some(connection_node) = connection.nodes.iter().find(|cn| cn.roomid != region.id)
                        {
                            if let Some(target_region) = world.regions.iter().find(|r| r.id == connection_node.roomid)
                            {
                                if let Some(target_node) = target_region.nodes.iter().find(|n| n.id == connection_node.nodeid)
                                {
                                    if !state.nodes.contains(&(target_region.id, target_node.id))
                                    {
                                        if let Some(mut new_locations) = Location::visit_node(items, world, target_region, target_node, node, state)
                                        {
                                            locations.append(&mut new_locations);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }             
            }
        }

        /* Find the in-room links for this node */
        if let Some(links) = region.links.iter().find(|l| l.from == node.id)
        {
            for link in &links.to
            {
                if let Some(link_node) = region.nodes.iter().find(|n| n.id == link.id)
                {
                    if !state.nodes.contains(&(region.id, link_node.id))
                    {
                        if Location::can_traverse(items, world, region, &link, state) && Location::can_access(items, world, region, &link_node, state)
                        {
                            if let Some(mut new_locations) = Location::visit_node(items, world, region, link_node, node, state)
                            {
                                locations.append(&mut new_locations);
                            }
                        }
                    }                     
                }
            }
        }
        
        if locations.len() > 0
        {
            Some(locations)
        }
        else
        {
            None
        }
    }

    fn can_unlock(items: &HashSet<String>, world: &World, region: &Region, node: &Node, state: &mut State) -> bool
    {
        match &node.locks {
            Some(locks) => locks.iter().all(|lock|
                match &lock.lock
                {
                    Some(_r) => true,
                    None => {
                        (match &lock.unlockStrats {
                            Some(us) => us.iter().any(|s| Location::can_do_strat(items, world, region, s, state)),
                            None => true
                        } ||
                        match &lock.bypassStrats {
                            Some(bs) => bs.iter().any(|s| Location::can_do_strat(items, world, region, s, state)),
                            None => false
                        })
                    }
                }
            ),
            None => true
        }
    }

    fn can_access(items: &HashSet<String>, world: &World, region: &Region, node: &Node, state: &mut State) -> bool
    {
        match &node.interactionRequires {
            Some(r) => r.check(items, world, state),
            None => true
        }
    }

    fn can_traverse(items: &HashSet<String>, world: &World, region: &Region, link: &LinkTo, state: &mut State) -> bool
    {        
        match &link.strats {
            Some(strats) => strats.iter().any(|s| Location::can_do_strat(items, world, region, s, state)),
            None => true
        }
    }

    fn can_do_strat(items: &HashSet<String>, world: &World, region: &Region, strat: &Strat, state: &mut State) -> bool
    {
        (match &strat.requires {
            Some(r) => r.check(items, world, state),
            None => true
        } && match &strat.obstacles {
            Some(ob) => {
                ob.iter().all(|o| state.obstacles.contains(&(region.id, o.id.as_ref().unwrap().to_string())) ||
                    (match &o.requires {
                        Some(r) => {
                            match r.check(items, world, state)
                            {
                                true => state.obstacles.insert((region.id, o.id.as_ref().unwrap().to_string())) && true,
                                false => false
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
        })
    }
}