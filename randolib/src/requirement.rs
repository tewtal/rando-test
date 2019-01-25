use serde_derive::{Serialize,Deserialize};
use crate::sparking::{CanComeInCharged, CanShineCharge, AdjacentRunway};
use crate::node::CanVisitNode;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum Requirement
{
    Or { or: Vec<Requirement> },
    ExplicitAnd { and: Vec<Requirement> },
    Not { not: Vec<Requirement> },
    AdjacentRunway { adjacentRunway: AdjacentRunway },
    CanShineCharge { canShineCharge: CanShineCharge },
    CanComeInCharged { canComeInCharged: CanComeInCharged },
    CanVisitNode { canVisitNode: CanVisitNode },
    And(Vec<Requirement>),
    Req(String),
    None
}