use std::sync::{Arc, Mutex, MutexGuard};

use sqlx::{Pool, Sqlite};

use crate::game::{game::Game, lobby::Lobby, player::Player};

pub struct AppState {
    pub games: Arc<Mutex<Vec<Game>>>,
    pub lobbies: Arc<Mutex<Vec<Lobby>>>,
    pub players: Arc<Mutex<Vec<Player>>>,
    pub pool: Pool<Sqlite>,
}

impl AppState {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            games: Arc::new(Mutex::new(vec![])),
            lobbies: Arc::new(Mutex::new(vec![])),
            players: Arc::new(Mutex::new(vec![])),
            pool,
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
