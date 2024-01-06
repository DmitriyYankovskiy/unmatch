use crate::card::{self, CardStack};

pub enum GameState {
    Setup(Setup),
    On(Game),
    End,
}

pub struct Setup {
    player_count: usize,
    decks: Vec<Option<card::Deck>>,
}
pub struct Game {
    player_count: usize,
    stacks: Vec<card::CardStack>,
}

impl GameState {
    fn start(&mut self) -> Result<(), &str> {
        if let GameState::Setup(setup) = self {
            let mut decks = vec![card::Deck::new(); setup.player_count];

            for i in 0..setup.player_count {
                let deck = setup.decks[i].to_owned();
                match deck {
                    Some(deck) => {
                        decks[i] = deck;
                    }
                    None => return Err("first deck is not selected"),
                }
            }

            let stacks = decks.into_iter().map(card::CardStack::from).collect();

            *self = GameState::On(Game {
                player_count: setup.player_count,
                stacks,
            });

            Ok(())
        } else {
            Err("game already started")
        }
    }
}

impl GameState {
    pub fn new() -> GameState {
        GameState::Setup(Setup {
            player_count: 0,
            decks: vec![],
        })
    }
}
