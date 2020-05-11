use serde_derive::{Serialize, Deserialize};
use crate::node::Node;
use crate::link::Link;
use crate::link::Obstacle;
use std::fs;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Region {
    pub id: i64,
    pub name: String,
    pub area: String,
    pub subarea: String,
    pub roomAddress: Option<String>,
    pub note: Option<Note>,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>,
    pub obstacles: Option<Vec<Obstacle>>
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum Note {
    Note(String),
    Notes(Vec<String>)
} 

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionFile {
    #[serde(rename="$schema")]
    pub schema: String,
    pub rooms: Option<Vec<Region>>
}

impl RegionFile {
    pub fn read(path: &str) -> Result<Vec<Region>, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let region_file: RegionFile = serde_json::from_str(&data)?;
        if let Some(regions) = region_file.rooms {
            Ok(regions)
        } else {
            bail!(format!("Could not find rooms node in region file: {:?}", path))
        }
    }
}