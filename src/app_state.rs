use std::sync::Mutex;

use crate::{game::Game, lobby::Lobby, player::Player};

pub struct AppState {
    pub games: Mutex<Vec<Game>>,
    pub lobbies: Mutex<Vec<Lobby>>,
    pub players: Mutex<Vec<Player>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Mutex::new(vec![]),
            lobbies: Mutex::new(vec![]),
            players: Mutex::new(vec![]),
        }
    }
}
impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
