use std::collections::HashMap;

use crate::{card::{self, CardStack}, character::Characters, Readable};

pub enum GameState {
    Setup(Setup),
    On(Game),
    End,
}

pub struct Setup {
    player_count: usize,
    characters: Characters,
    decks: Vec<card::Deck>,
}
pub struct Game {
    player_count: usize,
    characters: Characters,
    stacks: Vec<card::CardStack>,
    
}

impl GameState {
    pub fn new() -> GameState {
        GameState::Setup(Setup {
            player_count: 0,
            characters: Characters::new(),
            decks: vec![],
        })
    }

    pub fn start(mut self) -> Result<(), &'static str> {
        if let GameState::Setup(setup) = self {
            let stacks = setup.decks.iter().map(card::CardStack::from).collect();

            self = GameState::On(Game {
                player_count: setup.player_count,
                characters: setup.characters,
                stacks,
            });

            Ok(())
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }

    pub fn add_player(&mut self, name: &String) -> Result<usize, &str>{
        if let GameState::Setup(setup) = self {
            match Readable::from_path(format!("decks/{}.json", name.clone())) {
                Ok(deck) => {
                    setup.decks.push(deck);
                    setup.player_count += 1;
                },
                Err(_) => {
                    return Err("you are trying to add new player in the game but his/her character does not exists");
                }
            };

            match <Characters as Readable>::from_path(format!("characters.json")) {
                Ok(characters) => {
                    if setup.characters.contains_key(name) {
                        return Err("you are trying to add a player to the game but his character has already been selected");
                    }
                    setup.characters.insert((*name).clone(), characters[&format!("{}", name).to_string()].clone());
                },
                Err(_) => {
                    return Err("you are trying to add new player in the game but his/her character does not exists");
                }
            };

            Ok(setup.player_count - 1)
        } else {
            Err("you are trying to add new player in the game but it has already started")
        }
    }
}
