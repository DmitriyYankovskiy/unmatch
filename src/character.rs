use crate::{card, Readable, map};
use set::Set;
use std::{
    fs,
    io::{Result, Write},
    ops::{Deref, DerefMut},
};

use serde::{Deserialize, Serialize};
use serde_json::{self, json};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AttackType {
    Melee,
    Ranged,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sidekick {
    name: String,
    health: usize,
    attack_type: AttackType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Character {
    pub name: String,
    pub health: usize,
    pub attack_type: AttackType,
    pub abilities: Vec<card::Actions>,
    pub sidekick: Option<Set<Sidekick>>,
}

impl Readable for Vec<Character> {
    fn from_path(name: String) -> Result<Self> {
        let file = fs::read_to_string(format!("data/{name}.json"))?;
        let characters: Vec<Character> = serde_json::from_str(file.as_str())?;
        return Ok(characters);
    }
}
