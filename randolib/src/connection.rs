use serde_derive::{Serialize, Deserialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Connection {
    pub connectionType: String,
    pub description: Option<String>,
    pub nodes: Vec<ConnectionNode>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionFile {
    #[serde(rename="$schema")]
    pub schema: String,
    pub connections: Option<Vec<Connection>>
}

impl ConnectionFile {
    pub fn read(path: &str) -> Result<Vec<Connection>, Box<dyn std::error::Error>> {
        let data = fs::read_to_string(path)?;
        let connection_file: ConnectionFile = serde_json::from_str(&data)?;
        if let Some(connections) = connection_file.connections {
            Ok(connections)
        } else {
            bail!(format!("Could not find connections node in connection file: {:?}", path))
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConnectionNode {
    pub area: String,
    pub subarea: String,
    pub roomid: i64,
    pub nodeid: i64,
    pub position: String
}
