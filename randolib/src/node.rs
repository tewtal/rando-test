use serde_derive::{Serialize,Deserialize};
use crate::sparking::{Sparking, Runway};
use crate::requirement::Requirement;
use crate::region::Note;
use crate::link::Strat;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Node
{
    pub id: i64,
    pub name: String,
    pub nodeType: Option<NodeType>,
    pub nodeSubType: Option<NodeSubType>,
    pub nodeItem: Option<String>,
    pub nodeAddress: Option<String>,
    pub interactionRequires: Option<Requirement>,
    pub runways: Option<Vec<Runway>>,
    pub sparking: Option<Sparking>,
    pub spawnAt: Option<i64>,
    pub locks: Option<Vec<Lock>>,
    pub utility: Option<Vec<Utility>>,
    pub view: Option<Requirement>,
    pub yields: Option<Vec<String>>,
    pub note: Option<Note>
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Lock
{
    pub name: Option<String>,
    pub lock: Option<Requirement>,
    pub unlockStrats: Option<Vec<Strat>>,
    pub bypassStrats: Option<Vec<Strat>>,
    pub note: Option<Note>
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CanVisitNode
{
    pub number: i64,
    pub scope: String
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NodeType
{
    Door,
    Entrance,
    Exit,
    Event,
    Item,
    Junction
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum NodeSubType
{
    Boss,
    Chozo,
    Flag,
    Hidden,
    Visible,
    Blue,
    Doorway,
    Elevator,
    Eye,
    Gray,
    Grey,
    Green,
    #[serde(rename = "one-way")]
    OneWay,
    Passage,
    Red,
    Sandpit,
    Vertical,
    Yellow,
    Junction,
    Chest,
    Dash,
    Dig,
    Standing,
    Tablet,
    #[serde(rename = "closedWall")]
    ClosedWall
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Utility {
    Save,
    Missile,
    Super,
    PowerBomb,
    Energy,
    Reserve,
    Map
}
