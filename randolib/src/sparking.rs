use serde_derive::{Serialize, Deserialize};
use crate::link::Strat;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Sparking
{
    pub runways: Option<Vec<Runway>>,
    pub canLeaveCharged: Option<Vec<CanLeaveCharged>>
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Runway
{
    pub length: i64,
    pub strats: Option<Vec<Strat>>,
    pub usableComingIn: Option<bool>,
    pub openEnd: i64
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct CanLeaveCharged
{
    pub usedTiles: i64,
    pub framesRemaining: i64,
    pub strats: Option<Vec<Strat>>,
    pub openEnd: i64
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CanShineCharge
{
    pub usedTiles: i64
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct CanComeInCharged
{
    pub fromNode: i64,
    pub framesRemaining: i64
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct AdjacentRunway
{
    pub fromNode: i64,
    pub usedTiles: i64
}
