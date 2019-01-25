use serde_derive::{Serialize,Deserialize};
use crate::requirement::Requirement;

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
    pub requires: Option<Requirement>,
    pub unlock: Option<Requirement>,
    pub yields: Option<Vec<String>>,
    pub note: Option<String>
}