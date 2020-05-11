use serde_derive::{Serialize,Deserialize};
use crate::requirement::Requirement;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Helper {
    pub name: String,
    pub requires: Option<Requirement>,
    pub note: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelperFile {
    #[serde(rename="$schema")]
    pub schema: String,
    pub helpers: Option<Vec<Helper>>    
}

impl HelperFile {
    pub fn read(path: &str) -> Result<Vec<Helper>, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let helper_file: HelperFile = serde_json::from_str(&data)?;
        if let Some(helpers) = helper_file.helpers {
            Ok(helpers)
        } else {
            bail!(format!("Could not find helpers node in helper file: {:?}", path))
        }
    }
}