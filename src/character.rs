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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttackType {
    Melee,
    Ranged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sidekick {
    name: String,
    health: usize,
    attack_type: AttackType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CharacterState {
    health: usize,
    attack_type: AttackType,
    abilities: Vec<card::Actions>,
    sidekick: Option<Set<Sidekick>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

pub type Character = HashMap<String, CharacterState>;

impl Readable for Character {
    fn from_name(name: String) -> Result<Self> {
        //let file = fs::read_to_string(format!("data/{name}.json"))?;

        //let character: Character = serde_json::from_str(file.as_str())?;

        let mut y = fs::File::create("data/characters.json").unwrap();
        let mut ch = Character::new();

        ch.insert(
            "name".to_string(),
            CharacterState {
                health: 10,
                attack_type: AttackType::Ranged,
                abilities: vec![],
                sidekick: Option::Some(Set::new(
                    Sidekick {
                        name: "side name".to_string(),
                        health: 5,
                        attack_type: AttackType::Melee,
                    },
                    2,
                )),
            },
        );
        y.write_all(json!(ch).to_string().as_bytes()).unwrap();

        return Ok(ch);
    }
}
