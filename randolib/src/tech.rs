use serde_derive::{Serialize,Deserialize};
use crate::helper::Helper;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct TechFile {
    #[serde(rename="$schema")]
    pub schema: String,
    pub techs: Option<Vec<Helper>>    
}

impl TechFile {
    pub fn read(path: &str) -> Result<Vec<Helper>, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let tech_file: TechFile = serde_json::from_str(&data)?;
        if let Some(techs) = tech_file.techs {
            Ok(techs)
        } else {
            bail!(format!("Could not find techs node in tech file: {:?}", path))
        }
    }
}