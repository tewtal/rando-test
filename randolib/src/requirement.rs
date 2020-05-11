use serde_derive::{Serialize,Deserialize};
use crate::sparking::{CanComeInCharged, CanShineCharge, AdjacentRunway};
use crate::node::CanVisitNode;
use crate::link::{EnemyDamage, ResetRoom, Ammo, EnemyKill, AmmoDrain};
use std::collections::HashSet;
use crate::location::State;
use crate::world::World;
use crate::weapon::Weapon;
use crate::enemy::Enemy;

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(untagged)]
pub enum Requirement {
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
    DraygonElectricityFrames { draygonElectricityFrames: i64 },
    HibashiHits { hibashiHits: i64 },
    EnergyAtMost { energyAtMost: i64 },
    PreviousStratProperty { previousStratProperty: String },
    AmmoDrain { ammoDrain: AmmoDrain },
    And(Vec<Requirement>),
    Req(String),
    None
}

impl Requirement {
    pub fn check(&self, items: &HashSet<String>, world: &World, state: &State) -> bool {
        match self {
            Requirement::Or { or: reqs } => reqs.iter().any(|r| r.check(items, world, state)),
            Requirement::ExplicitAnd { and: reqs} => reqs.iter().all(|r| r.check(items, world, state)),
            Requirement::Not { not: reqs } => !reqs.iter().any(|r| r.check(items, world, state)),
            Requirement::AdjacentRunway { adjacentRunway: _a} => true,
            Requirement::CanShineCharge { canShineCharge: _cs } => items.contains("SpeedBooster"),
            Requirement::CanComeInCharged { canComeInCharged: _c } => false,
            Requirement::CanVisitNode { canVisitNode: _cv } => true,
            Requirement::EnemyDamage { enemyDamage: _ed } => true,
            Requirement::ResetRoom { resetRoom: _rr } => true,
            Requirement::Ammo { ammo : _a } => true,
            Requirement::PreviousNode { previousNode: _p } => true,
            Requirement::SpikeHits { spikeHits: _s } => items.contains("EnergyTank"),
            Requirement::EnemyKill { enemyKill: e } => {
                let weapons = &world.weapons.iter().filter(|w| !w.situational && w.useRequires.check(items, world, state)).collect::<Vec<&Weapon>>();
                let weaponNames = weapons.iter().map(|w| &w.name).collect::<Vec<&String>>();

                if let Some(explicitWeapons) = &e.explicitWeapons {
                    if explicitWeapons.iter().any(|w| weaponNames.contains(&&w)) {
                        return true;
                    }
                }

                let enemies = &e.enemies.iter().flatten().filter(|e| world.enemies.iter().any(|we| we.name == **e)).map(|e| world.enemies.iter().find(|we| we.name == *e).unwrap()).collect::<Vec<&Enemy>>();
                let invulns = enemies.iter().flat_map(|e| e.invul.as_ref().unwrap_or(&vec![]).to_owned()).collect::<Vec<String>>();

                if invulns.is_empty() {
                    return true;
                }
                
                for weapon in weapons {
                    let weaponVuln = invulns.contains(&weapon.name);
                    let categoryVuln = weapon.categories.iter().any(|c| invulns.contains(c));
                    if !weaponVuln && !categoryVuln {
                        return true;
                    }
                }

                false
            },
            Requirement::HeatFrames { heatFrames: _h } => true,
            Requirement::AcidFrames { acidFrames: _a } => true,
            Requirement::LavaFrames { lavaFrames: _l } => true,
            Requirement::DraygonElectricityFrames { draygonElectricityFrames: _d } => true,
            Requirement::EnergyAtMost { energyAtMost: _e } => true,
            Requirement::HibashiHits { hibashiHits: _h } => true,
            Requirement::PreviousStratProperty { previousStratProperty: _p } => true,
            Requirement::AmmoDrain { ammoDrain: _a } => true,
            Requirement::And(reqs) => { reqs.iter().all(|r| r.check(items, world, state)) },
            Requirement::Req(r) => { items.contains(r) || state.events.contains(r) },
            Requirement::None => true
        }
    }
}