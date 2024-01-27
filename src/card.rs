use rand::{seq::SliceRandom, thread_rng, Rng};
use serde::{Deserialize, Serialize};
use set::Set;
use std::{collections::HashMap, fs, io::Result};

use crate::Readable;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Fighter {
    Any,
    Hero,
    Support,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Actions {}

pub mod scheme {
    use super::Actions;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Card {
        actions: Vec<Actions>,
    }
}

pub mod combat {
    use serde::{Deserialize, Serialize};

    use super::Actions;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub enum Class {
        Attack,
        Defense,
        Versatile,
    }
    #[derive(Serialize, Deserialize, Debug, Clone)]
    #[serde(rename_all = "camelCase")]
    pub struct Card {
        value: u32,
        immidietly: Vec<Actions>,
        during_combat: Vec<Actions>,
        after_combat: Vec<Actions>,

        #[serde(rename = "type")]
        class: Class,
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Class {
    Combat(combat::Card),
    Sheme(scheme::Card),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Card {
    name: String,

    #[serde(rename = "type")]
    class: Class,
    
    fighter: Fighter,
    boost: u32,
}

pub type Deck = Vec<Set<Card>>;

impl Readable for Deck {
    fn from_path(path: String) -> Result<Self> {
        let file = fs::read_to_string(format!("data/{path}.json"))?;

        let deck: Deck = serde_json::from_str(file.as_str())?;
        return Ok(deck);
    }
}

#[derive(Clone)]
pub struct CardStack {
    stack: Vec<Card>,
}

impl CardStack {
    pub fn from(deck: &Deck) -> CardStack {
        let mut card_stack = CardStack { stack: vec![] };
        for card_set in deck {
            Vec::append(&mut card_stack.stack, &mut card_set.open());
        }
        card_stack
    }

    pub fn push(&mut self, card: Card) {
        self.stack.push(card);
    }

    pub fn shuffle(&mut self) {
        self.stack.shuffle(&mut thread_rng());
    }
}
