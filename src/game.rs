use crate::card;

pub enum GameState {
    Setup(Setup),
    On(Game),
    End,
}

pub struct Setup {
    first_deck: Option<card::CardStack>,
    second_deck: Option<card::CardStack>,
}
pub struct Game {
    first_stack: card::CardStack,
    second_stack: card::CardStack,
}

impl GameState {
    fn start(& mut self) -> Result<(), &str> {
        if let GameState::Setup(setup) = self {
            Ok(())
        } else {
            Err("Game already started")
        }
    }
}

impl Game {
    fn new() {
         
    }
}