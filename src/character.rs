use crate::{card, Readable};
use set::Set;
use std::{
    collections::HashMap,
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
pub struct CharacterState {
    health: usize,
    attack_type: AttackType,
    abilities: Vec<card::Actions>,
    sidekick: Option<Set<Sidekick>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Object<T> {
    position: i32,
    object: T,
}

impl<T> Deref for Object<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.object
    }
}

impl<T> DerefMut for Object<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.object
    }
}

pub type Characters = HashMap<String, CharacterState>;

impl Readable for Characters {
    fn from_path(name: String) -> Result<Self> {
        let file = fs::read_to_string(format!("data/{name}.json"))?;
        let characters: Characters = serde_json::from_str(file.as_str())?;
        return Ok(characters);
    }
}
