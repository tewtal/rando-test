use serde_derive::{Serialize,Deserialize};
use crate::requirement::Requirement;
use crate::region::Note;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Link
{
    pub from: i64, 
    pub to: Vec<LinkTo>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct LinkTo
{
    pub id: i64,
    pub strats: Option<Vec<Strat>>,
    pub requires: Option<Requirement>,
    pub unlock: Option<Requirement>,
    pub yields: Option<Vec<String>>,
    pub note: Option<Note>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Strat
{
    pub name: String,
    pub notable: bool,
    pub requires: Option<Requirement>,
    pub obstacles: Option<Vec<Obstacle>>,
    pub note: Option<Note>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Obstacle
{
    pub id: Option<String>,
    pub requires: Option<Requirement>,
    pub bypass: Option<Requirement>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EnemyDamage
{
    pub enemy: String,
    pub r#type: String,
    pub hits: i64
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ResetRoom
{
    pub nodes: Vec<i64>,
    pub mustStayPut: Option<bool>,
    pub nodesToAvoid: Option<Vec<i64>>
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Ammo
{
    pub r#type: String,
    pub count: i64
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AmmoDrain
{
    pub r#type: String,
    pub count: i64
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct EnemyKill
{
    pub enemies: Vec<Vec<String>>,
}


