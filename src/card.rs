use std::{fs, io, collections::HashMap};

use serde::{Serialize, Deserialize};
use io::Result; 

#[derive(Serialize, Deserialize, Debug)]
pub enum Fighter {
    Any,
    Hero,
    Support,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Actions {

}
pub mod scheme {
    use serde::{Serialize, Deserialize};
    use super::Actions;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Card {
        pub actions: Vec<Actions>,
    }
}

pub mod combat {
    use serde::{Serialize, Deserialize};

    use super::Actions;

    #[derive(Serialize, Deserialize, Debug)]
    pub enum Class {
        Attack,
        Defense,
        Versatile,
    }
    #[derive(Serialize, Deserialize, Debug)]
    pub struct Card {
        pub value: u32,
        pub immidietly: Vec<Actions>,
        pub during_combat: Vec<Actions>,
        pub after_combat: Vec<Actions>,

        pub class: Class,
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Class {
    Combat(combat::Card),
    Sheme(scheme::Card),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Card {
    pub name: String,

    pub class: Class,
    pub fighter: Fighter,
    pub boost: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CardSet {
    pub count: usize,
    pub card: Card,
}

type Deck = HashMap<usize, CardSet>;
 
pub fn get_deck(name: String) -> Result<Deck>{
    let file = fs::read_to_string(format!("data/decks/{name}.json"))?;

    let deck: Deck = serde_json::from_str(file.as_str())?;
    return Ok(deck);
}


struct CardStack {
    stack: Vec<Card>,
}

impl CardStack {
    fn push(&mut self, card: Card) {
        self.stack.push(card);
    }

    pub fn from_deck(deck: Deck) -> CardStack {
        let card_stack = CardStack { stack: vec![] };
        for (id, card_set) in deck {
            for i in 0..card_set.count {
                card_stack.push(card_set.card);
            }
        }
        card_stack
    }
}