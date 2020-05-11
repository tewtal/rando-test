use serde_derive::{Serialize,Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Enemy {
    pub id: i64,
    pub name: String,
    pub invul: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnemyFile {
    #[serde(rename="$schema")]
    pub schema: String,
    pub enemies: Option<Vec<Enemy>>
}

impl EnemyFile {
    pub fn read(path: &str) -> Result<Vec<Enemy>, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let enemy_file: EnemyFile = serde_json::from_str(&data)?;
        if let Some(enemies) = enemy_file.enemies {
            Ok(enemies)
        } else {
            bail!(format!("Could not find enemies node in enemy file: {:?}", path))
        }
    }
}