use crate::region::Region;
use crate::node::{Node, NodeType};
use crate::world::World;
use crate::link::{LinkTo, Strat};
use std::collections::HashSet;

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
        
        let mut visited_nodes: Vec<(i64, i64)> = Vec::new();
        let locations = Location::visit_node(items, world, from_region, from_node, &mut visited_nodes).unwrap();


        // let mut unvisited_nodes: Vec<String> = Vec::new();
        // for region in &world.regions {
        //     for node in &region.nodes {
        //         if !visited_nodes.contains(&(region.id, node.id)) {
        //             unvisited_nodes.push(format!("{} - {}\n", region.name, node.name));
        //         }
        //     }
        // }

        // print!("Unvisited nodes:\n");
        // for uv in unvisited_nodes {
        //     print!("{}", uv);
        // }

        Some(locations)
    }

    fn visit_node(items: &HashSet<String>, world: &'a World, region: &'a Region, node: &'a Node, visited_nodes: &mut Vec<(i64, i64)>) -> Option<Vec<Location<'a>>>
    {
        //print!("Visisting {}: {}\n", &region.name, &node.name);
        visited_nodes.push((region.id, node.id));

        let mut locations: Vec<Location> = Vec::new();

        if let Some(nodeType) = &node.nodeType
        {
            /* If it's an item, add it to our item location collection */
            if nodeType == &NodeType::Item
            {
                let location = Location
                {
                    name: format!("{} - {}", region.name, node.name),
                    region: region,
                    node: node,
                };

                locations.push(location);
            }

            /* If it's a door or exit, find the connecting node and visit it */
            if nodeType == &NodeType::Door || nodeType == &NodeType::Exit
            {
                if let Some(connection) = world.connections.iter().find(|c| c.nodes.iter().any(|cn| cn.roomid == region.id && cn.nodeid == node.id))
                {
                    if let Some(connection_node) = connection.nodes.iter().find(|cn| cn.roomid != region.id)
                    {
                        if let Some(target_region) = world.regions.iter().find(|r| r.id == connection_node.roomid)
                        {
                            if let Some(target_node) = target_region.nodes.iter().find(|n| n.id == connection_node.nodeid)
                            {
                                if !visited_nodes.contains(&(target_region.id, target_node.id))
                                {
                                    if let Some(mut new_locations) = Location::visit_node(items, world, target_region, target_node, visited_nodes)
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

        /* Find the in-room links for this node */
        if let Some(links) = region.links.iter().find(|l| l.from == node.id)
        {
            for link in &links.to
            {
                if let Some(link_node) = region.nodes.iter().find(|n| n.id == link.id)
                {
                    if !visited_nodes.contains(&(region.id, link_node.id))
                    {
                        if Location::can_traverse(items, &link) && Location::can_access(items, &link_node)
                        {
                            if let Some(mut new_locations) = Location::visit_node(items, world, region, link_node, visited_nodes)
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

    fn can_access(items: &HashSet<String>, node: &Node) -> bool
    {
        (match &node.interactionRequires {
            Some(r) => r.check(items),
            None => true
        } &&
        match &node.locks {
            Some(locks) => locks.iter().all(|lock|
                match &lock.lock
                {
                    Some(_r) => true,
                    None => {
                        (match &lock.unlockStrats {
                            Some(us) => us.iter().any(|s| Location::can_do_strat(items, s)),
                            None => true
                        } ||
                        match &lock.bypassStrats {
                            Some(bs) => bs.iter().any(|s| Location::can_do_strat(items, s)),
                            None => false
                        })
                    }
                }
            ),
            None => true
        })
    }

    fn can_traverse(items: &HashSet<String>, link: &LinkTo) -> bool
    {        
        match &link.strats {
            Some(strats) => strats.iter().any(|s| Location::can_do_strat(items, s)),
            None => true
        }
    }

    fn can_do_strat(items: &HashSet<String>, strat: &Strat) -> bool
    {
        (match &strat.requires {
            Some(r) => r.check(items),
            None => true
        } && match &strat.obstacles {
            Some(ob) => {
                ob.iter().any(|o| 
                    match &o.requires {
                        Some(r) => r.check(items),
                        None => true
                    } || 
                    match &o.bypass {
                        Some(b) => b.check(items),
                        None => false
                    }
                )                                
            },
            None => true
        })
    }
}