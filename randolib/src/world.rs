use serde_derive::{Serialize, Deserialize};
use crate::region::{Region, RegionFile};
use crate::connection::{Connection, ConnectionFile};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize, Debug)]
pub struct World
{
    pub name: String,
    pub regions: Vec<Region>,
    pub connections: Vec<Connection>
}

impl World
{
    pub fn load_from(name: String, path: &String) -> Result<World, Box<std::error::Error>>
    {
        let region_files = WalkDir::new(path.clone() + "/region/").into_iter().filter_map(|e| e.ok()).filter(|f| f.file_name().to_string_lossy().ends_with(".json"));
        let connection_files = WalkDir::new(path.clone() + "/connection/").into_iter().filter_map(|e| e.ok()).filter(|f| f.file_name().to_string_lossy().ends_with(".json"));

        let mut regions: Vec<Region> = Vec::new();
        let mut connections: Vec<Connection> = Vec::new();

        for region_file in region_files
        {
            let mut rs = RegionFile::read(&region_file.into_path().to_string_lossy().into_owned())?;
            regions.append(&mut rs);
        }

        for connection_file in connection_files
        {
            let mut cs = ConnectionFile::read(&connection_file.into_path().to_string_lossy().into_owned())?;
            connections.append(&mut cs);
        }

        Ok(World
        {
            name: name,
            regions: regions,
            connections: connections
        })
    }
}
