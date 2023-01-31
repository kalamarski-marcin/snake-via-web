use super::directions::Direction;
use super::game::Game;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct State {
    pub directions: Arc<Mutex<Vec<Direction>>>,
    pub game: Arc<Mutex<Game>>,
}

impl State {
    pub fn new() -> Self {
        Self {
            directions: Arc::new(Mutex::new(vec![])),
            game: Arc::new(Mutex::new(Game::new())),
        }
    }
}
