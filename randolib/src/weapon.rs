use serde_derive::{Serialize,Deserialize};
use std::fs;
use crate::requirement::Requirement;

#[derive(Serialize, Deserialize, Debug)]
pub struct Weapon
{
    pub id: i64,
    pub name: String,
    pub situational: bool,
    pub useRequires: Requirement,
    pub categories: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WeaponFile
{
    #[serde(rename="$schema")]
    pub schema: String,
    pub weapons: Option<Vec<Weapon>>
}
impl WeaponFile
{
    pub fn read(path: &String) -> Result<Vec<Weapon>, Box<std::error::Error>>
    {
        let data = fs::read_to_string(path)?;
        let weapon_file: WeaponFile = serde_json::from_str(&data)?;
        if let Some(weapons) = weapon_file.weapons
        {
            Ok(weapons)
        }
        else
        {
            bail!(format!("Could not find weapons node in weapon file: {:?}", path))
        }
    }
}