use crate::region::Region;
use crate::connection::Connection;
use crate::requirement::Requirement;
use crate::node::{Node, NodeType};
use crate::world::World;

#[derive(Debug, Clone, PartialEq)]
pub struct Location<'a>
{
    pub name: String,
    pub region: &'a Region,
    pub node: &'a Node,
    pub requires: Vec<&'a Requirement>,
}

impl<'a> Location<'a>
{
    pub fn generate(world: &'a World, from_region: &'a Region, from_node: &'a Node) -> Option<Vec<Location<'a>>>
    {
        let visited_regions: Vec<&Region> = Vec::new();     
        Location::traverse_region(world, from_region, from_node, &visited_regions, Vec::new())
    }

    fn merge_locations(locations: Vec<Location<'a>>) -> Vec<Location<'a>>
    {
        let mut new_locations: Vec<Location> = Vec::new();
        for mut loc in locations
        {
            if new_locations.iter().any(|n| n.region == loc.region && n.node == loc.node)
            {
                continue;
            }
            new_locations.push(loc.clone());
        }

        new_locations
    }

    fn find_region_locations(world: &'a World, region: &'a Region, node: &'a Node, requirements: Vec<&'a Requirement>) -> Option<Vec<Location<'a>>>
    {
        let visited_nodes: Vec<&Node> = Vec::new();
        let requirement = requirements.clone();
        match Location::traverse_node(world, region, node, &visited_nodes, requirements)
        {
            Some(l) => Some(Location::merge_locations(l)),
            None => None
        }
    }

    fn traverse_node(world: &'a World, region: &'a Region, node: &'a Node, visited_nodes: &Vec<&Node>, mut requirements: Vec<&'a Requirement>) -> Option<Vec<Location<'a>>>
    {
        if visited_nodes.contains(&node)
        {
            return None;
        }

        let mut locations: Vec<Location> = Vec::new();

        if let Some(nodeType) = &node.nodeType
        {
            /* If it's an item or a door, add it to the location collection */
            if nodeType == &NodeType::Item || nodeType == &NodeType::Door
            {
                let location = Location
                {
                    name: format!("{} - {}", region.name, node.name),
                    region: region,
                    node: node,
                    requires: requirements.clone()
                };

                locations.push(location);
            }
        }

        /* Find the in-room links for this node */
        if let Some(links) = region.links.iter().find(|l| l.from == node.id)
        {
            for link in &links.to
            {
                if let Some(link_node) = region.nodes.iter().find(|n| n.id == link.id)
                {
                    /* Found a link to another node, traverse it */
                    let mut link_requirements: Vec<&Requirement> = Vec::new();
                    link_requirements.append(&mut requirements);

                    if let Some(requires) = &link.requires
                    {
                        link_requirements.push(requires);
                    }

                    if let Some(unlock) = &link.unlock
                    {
                        link_requirements.push(unlock);
                    }

                    if let Some(node_requires) = &node.requires
                    {
                        link_requirements.push(node_requires);
                    }
                    
                    if let Some(node_unlock) = &node.unlock
                    {
                        link_requirements.push(node_unlock);
                    }
                    
                    let mut new_visited_nodes = visited_nodes.clone();
                    new_visited_nodes.push(node);

                    if let Some(mut new_locations) = Location::traverse_node(world, region, link_node, &new_visited_nodes, link_requirements)
                    {
                        locations.append(&mut new_locations);
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

    fn traverse_region(world: &'a World, region: &'a Region, node: &'a Node, visited_regions: &Vec<&Region>, mut requirements: Vec<&'a Requirement>) -> Option<Vec<Location<'a>>>
    {
        if visited_regions.contains(&region)
        {
            return None;
        }

        dbg!(&region.name);

        let regionnames: Vec<String> = visited_regions.iter().map(|vr| vr.name.to_string()).collect();
        //println!("Visited regions: {:?}", regionnames);
        //println!("Region: {:?}", region.name);

        let mut locations: Vec<Location> = Vec::new();
        if let Some(region_locations) = Location::find_region_locations(world, region, node, requirements.clone())
        {
            for item_location in region_locations.iter().filter(|rl| rl.node.nodeType == Some(NodeType::Item))
            {
                //dbg!(&item_location.name);
                locations.push(item_location.clone());
            }

            for door_location in region_locations.iter().filter(|rl| rl.node.nodeType == Some(NodeType::Door))
            {
                if let Some(connection) = world.connections.iter().find(|c| c.nodes.iter().any(|cn| cn.roomid == door_location.region.id && cn.nodeid == door_location.node.id))
                {
                    if let Some(connection_node) = connection.nodes.iter().find(|cn| cn.roomid != door_location.region.id)
                    {
                        if let Some(target_region) = world.regions.iter().find(|r| r.id == connection_node.roomid)
                        {
                            if let Some(target_node) = target_region.nodes.iter().find(|n| n.id == connection_node.nodeid)
                            {
                                let mut new_visited_regions = visited_regions.clone();
                                new_visited_regions.push(region);

                                let mut new_requirements: Vec<&Requirement> = Vec::new();
                                new_requirements.append(&mut door_location.requires.clone());
                                new_requirements.append(&mut requirements);
                                if let Some(r) = &door_location.node.requires
                                {
                                    new_requirements.push(r);
                                }

                                if let Some(u) = &door_location.node.unlock
                                {
                                    new_requirements.push(u);
                                }

                                if let Some(mut new_locations) = Location::traverse_region(world, target_region, target_node, &new_visited_regions, new_requirements)
                                {
                                    locations.append(&mut new_locations);
                                }
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
}
