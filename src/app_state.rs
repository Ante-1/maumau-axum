use std::sync::{Arc, Mutex, MutexGuard};

use crate::{game::Game, lobby::Lobby, player::Player};

pub struct AppState {
    pub games: Arc<Mutex<Vec<Game>>>,
    pub lobbies: Arc<Mutex<Vec<Lobby>>>,
    pub players: Arc<Mutex<Vec<Player>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            games: Arc::new(Mutex::new(vec![])),
            lobbies: Arc::new(Mutex::new(vec![])),
            players: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn get_games(&self) -> MutexGuard<'_, Vec<Game>> {
        self.games.lock().expect("mutex was poisoned")
    }

    pub fn get_lobbies(&self) -> MutexGuard<'_, Vec<Lobby>> {
        self.lobbies.lock().expect("mutex was poisoned")
    }

    pub fn get_players(&self) -> MutexGuard<'_, Vec<Player>> {
        self.players.lock().expect("mutex was poisoned")
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
