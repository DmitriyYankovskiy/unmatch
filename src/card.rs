use std::{vec, fs, io, collections::{hash_set, hash_map, HashMap}};

use actix_web::web::Json;
use serde::{Serialize, Deserialize};
use serde_json::{json, map};
use io::Write; 

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
struct CardSet {
    count: usize,
    card: Card,
}

type Deck = HashMap<usize, CardSet>;

pub fn get_deck(name: String) {
    // let file = fs::read_to_string(format!("/data/decks/{name}.json")).unwrap();
    // let json_file: serde_json::Value = serde_json::from_str(&file)?;

    let mut deck = Deck::new();
    deck.insert(0, CardSet {
            count: 3,
            card: Card {
            name: "Helio".to_string(),

            class: Class::Combat(combat::Card {
                value: 2,
                immidietly: vec![],
                during_combat: vec![],
                after_combat: vec![],

                class: combat::Class::Versatile,
            }),
            fighter: Fighter::Any,
            boost: 3,
        }
    });

    deck.insert(1, CardSet {
        count: 5,
        card: Card {
            name: "Fireball".to_string(),

            class: Class::Sheme(scheme::Card {
                actions: vec![],
            }),
            fighter: Fighter::Any,
            boost: 2,
        }
    });
    
    let mut file = fs::File::create("example.json").expect("can'not create file");
    file.write_all(format!("{:#}", json!(deck).to_string()).as_bytes()).unwrap();

    // println!("{:#?}", json!(card));
}