use serde_derive::{Serialize,Deserialize};
use crate::sparking::{CanComeInCharged, CanShineCharge, AdjacentRunway};
use crate::node::CanVisitNode;
use crate::link::{EnemyDamage, ResetRoom, Ammo, EnemyKill};
use std::collections::HashSet;

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
    EnemyDamage { enemyDamage: EnemyDamage },
    ResetRoom { resetRoom: ResetRoom },
    Ammo { ammo: Ammo },
    PreviousNode { previousNode: i64 },
    SpikeHits { spikeHits: i64 },
    EnemyKill { enemyKill: EnemyKill },
    HeatFrames { heatFrames: i64 },
    AcidFrames { acidFrames: i64 },
    LavaFrames { lavaFrames: i64 },
    HibashiHits { hibashiHits: i64 },
    EnergyAtMost { energyAtMost: i64 },
    PreviousStratProperty { previousStratProperty: String },
    And(Vec<Requirement>),
    Req(String),
    None
}

impl Requirement
{
    pub fn check(&self, items: &HashSet<String>) -> bool
    {
        match self {
            Requirement::Or { or: reqs } => reqs.iter().any(|r| r.check(items)),
            Requirement::ExplicitAnd { and: reqs} => reqs.iter().all(|r| r.check(items)),
            Requirement::Not { not: reqs } => !reqs.iter().any(|r| r.check(items)),
            Requirement::AdjacentRunway { adjacentRunway: _a} => true,
            Requirement::CanShineCharge { canShineCharge: _cs } => false,
            Requirement::CanComeInCharged { canComeInCharged: _c } => false,
            Requirement::CanVisitNode { canVisitNode: _cv } => true,
            Requirement::EnemyDamage { enemyDamage: _ed } => true,
            Requirement::ResetRoom { resetRoom: _rr } => true,
            Requirement::Ammo { ammo : _a } => true,
            Requirement::PreviousNode { previousNode: _p } => true,
            Requirement::SpikeHits { spikeHits: _s } => true,
            Requirement::EnemyKill { enemyKill: _e ] => true,
            Requirement::HeatFrames { heatFrames: _h } => true,
            Requirement::AcidFrames { acidFrames: _a } => true,
            Requirement::LavaFrames { lavaFrames: _l } => true,
            Requirement::EnergyAtMost { energyAtMost: _e } => true,
            Requirement::HibashiHits { hibashiHits: _h } => true,
            Requirement::PreviousStratProperty { previousStratProperty: _p } => true,
            Requirement::And(reqs) => { reqs.iter().all(|r| r.check(items)) },
            Requirement::Req(r) => { 
                // let ok = items.contains(r);
                // if !ok {
                //     print!("Unsatisified requirement: {}\n", r);
                // }
                // ok
                items.contains(r)
            },
            Requirement::None => true
        }
    }
}