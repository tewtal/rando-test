use serde_derive::{Serialize, Deserialize};
use crate::region::{Region, RegionFile};
use crate::connection::{Connection, ConnectionFile};
use crate::enemy::{Enemy, EnemyFile};
use crate::weapon::{Weapon, WeaponFile};
use crate::helper::{Helper, HelperFile};
use crate::tech::TechFile;
use std::iter::FromIterator;
use walkdir::WalkDir;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct World {
    pub name: String,
    pub regions: Vec<Region>,
    pub connections: Vec<Connection>,
    pub enemies: Vec<Enemy>,
    pub weapons: Vec<Weapon>,
    pub helpers: Vec<Helper>,
    pub techs: Vec<Helper>
}

impl World {
    pub fn get_flags(&self, items: &[&str], techs: &[&str]) -> HashSet<String> {
        let hs_items = HashSet::from_iter(items.iter().map(|&s| s.to_owned()));
        let hs_techs = HashSet::from_iter(techs.iter().map(|&s| s.to_owned()));
        let active_helpers = self.resolve_helpers(&hs_items);
        let active_techs = self.resolve_techs(&hs_items, &active_helpers, &hs_techs);
        hs_items.into_iter().chain(active_helpers.into_iter()).chain(active_techs.into_iter()).collect()
    }

    pub fn get_item_locations(&self) -> Option<Vec<&str>> {
        let locations = self.regions.iter().flat_map(|r| &r.nodes).filter(|n| n.nodeType == Some(crate::node::NodeType::Item)).map(|n| n.name.as_ref()).collect::<Vec<&str>>();
        if !locations.is_empty() {
            Some(locations)
        } else {
            None
        }
    }

    fn resolve_helpers(&self, items: &HashSet<String>) -> HashSet<String> {
        let mut hs: HashSet<String> = HashSet::new();
        let state = crate::location::State::new(0, 0);
        let mut lastCount = 1;

        while lastCount != hs.len() {
            lastCount = hs.len();

            for helper in &self.helpers {
                if let Some(hr) = &helper.requires {
                    let checkItems: HashSet<String> = items.union(&hs).map(|i| i.to_owned()).collect();
                    if hr.check(&checkItems, self, &state) && !hs.contains(&helper.name) {
                        hs.insert(helper.name.to_owned());
                    }
                } else if !hs.contains(&helper.name) {
                    hs.insert(helper.name.to_owned());
                }
            }
        }
        hs
    }

    fn resolve_techs(&self, items: &HashSet<String>, helpers: &HashSet<String>, techs: &HashSet<String>) -> HashSet<String> {
        let mut hs: HashSet<String> = HashSet::new();
        let state = crate::location::State::new(0, 0);
        let mut lastCount = 1;

        /* First resolve all techs possible with the items we have */
        while lastCount != hs.len() {
            lastCount = hs.len();

            for helper in &self.techs {
                if let Some(hr) = &helper.requires {
                    let checkItems: HashSet<String> = items.union(&hs).cloned().collect::<HashSet<String>>().union(&helpers).cloned().collect();
                    if hr.check(&checkItems, self, &state) && !hs.contains(&helper.name) {
                        hs.insert(helper.name.to_owned());
                    }
                } else if !hs.contains(&helper.name) {
                    hs.insert(helper.name.to_owned());
                }
            }
        }

        /* Then match the input techs with the list of resolved techs and return the matching entries */
        hs.iter().filter(|&h| techs.contains(h)).cloned().collect()
    }

    pub fn load_from(name: &str, path: &str) -> Result<World, Box<dyn std::error::Error>> {
        let region_files = WalkDir::new(format!("{}/region/", path)).into_iter().filter_map(|e| e.ok()).filter(|f| f.file_name().to_string_lossy().ends_with(".json"));
        let connection_files = WalkDir::new(format!("{}/connection/", path)).into_iter().filter_map(|e| e.ok()).filter(|f| f.file_name().to_string_lossy().ends_with(".json"));

        let mut regions: Vec<Region> = Vec::new();
        let mut connections: Vec<Connection> = Vec::new();
        let mut enemies: Vec<Enemy> = Vec::new();
        let mut weapons: Vec<Weapon> = Vec::new();
        let mut helpers: Vec<Helper> = Vec::new();
        let mut techs: Vec<Helper> = Vec::new();

        for region_file in region_files {
            let mut rs = RegionFile::read(&region_file.path().to_string_lossy().into_owned())?;
            regions.append(&mut rs);
        }

        for connection_file in connection_files {
            let mut cs = ConnectionFile::read(&connection_file.path().to_string_lossy().into_owned())?;
            connections.append(&mut cs);
        }

        let mut es = EnemyFile::read(&format!("{}/enemies/main.json", path))?;
        enemies.append(&mut es);

        let mut ws = WeaponFile::read(&format!("{}/weapons/main.json", path))?;
        weapons.append(&mut ws);

        let mut hs = HelperFile::read(&format!("{}/helpers.json", path))?;
        helpers.append(&mut hs);

        let mut ts = TechFile::read(&format!("{}/tech.json", path))?;
        techs.append(&mut ts);

        Ok(World {
            name: name.to_string(),
            regions,
            connections,
            enemies,
            weapons,
            helpers,
            techs
        })
    }
}
