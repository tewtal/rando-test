use serde_derive::{Serialize, Deserialize};
use serde_json;
use crate::node::Node;
use crate::link::Link;
use std::fs;


#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Region
{
    pub id: i64,
    pub name: String,
    pub area: String,
    pub subarea: String,
    pub note: Option<String>,
    pub nodes: Vec<Node>,
    pub links: Vec<Link>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RegionFile
{
    #[serde(rename="$schema")]
    pub schema: String,
    pub rooms: Option<Vec<Region>>
}

impl RegionFile
{
    pub fn read(path: &String) -> Result<Vec<Region>, Box<std::error::Error>>
    {
        let data = fs::read_to_string(path)?;
        let region_file: RegionFile = serde_json::from_str(&data)?;
        if let Some(regions) = region_file.rooms
        {
            Ok(regions)
        }
        else
        {
            bail!(format!("Could not find rooms node in region file: {:?}", path))
        }
    }
}