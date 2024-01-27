use std::collections::HashMap;

use crate::{card::{self, CardStack}, character::{self, Character}, Readable, PlayerInfo};

pub enum GameState {
    Setup(Setup),
    On(Game),
    End,
}

pub struct Setup {
    player_count: usize,
    players: Vec<Player>,
    decks: Vec<card::Deck>,
}

pub struct Player {
    name: String,
    character: Character,
}

impl Player {
    fn new(name: String, character: Character) -> Player {
        Player {name, character}
    }
}

pub struct Game {
    player_count: usize,
    players: Vec<Player>,
    stacks: Vec<card::CardStack>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState::Setup(Setup {
            player_count: 0,
            players: Vec::<Player>::new(),
            decks: vec![],
        })
    }

    pub fn start(mut self) -> Result<(), &'static str> {
        if let GameState::Setup(setup) = self {
            let stacks = setup.decks.iter().map(card::CardStack::from).collect();

            self = GameState::On(Game {
                player_count: setup.player_count,
                players: setup.players,
                stacks,
            });

            Ok(())
        } else {
            Err("you are trying to start the game but it has already started")
        }
    }

    pub fn add_player(&mut self, player: PlayerInfo<String>) -> Result<usize, &str>{
        if let GameState::Setup(setup) = self {
            match Readable::from_path(format!("decks/{}.json", player.info)) {
                Ok(deck) => {
                    setup.decks.push(deck);
                    setup.player_count += 1;
                },
                Err(_) => {
                    return Err("you are trying to add new player in the game but his/her deck does not exists");
                }
            };

            match <Vec<Character> as Readable>::from_path(format!("characters.json")) {
                Ok(characters) => {
                    if setup.players.iter().any(|exist_player| exist_player.character.name == player.info) {
                        return Err("you are trying to add a player to the game but his character has already been selected");
                    }
                    setup.players.push(match characters.into_iter().find(|character| character.name == player.info) {
                        Some(character) => Player::new(player.name, character),
                        None => {
                            return Err("you are trying to add new player in the game but his/her character does not exists");
                        }
                    });
                },
                Err(_) => {
                    return Err("file characters.json does not exists");
                }
            };

            Ok(setup.player_count - 1)
        } else {
            Err("you are trying to add new player in the game but it has already started")
        }
    }
}
