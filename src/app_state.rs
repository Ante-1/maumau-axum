use std::sync::{Arc, Mutex, MutexGuard};

use sqlx::{Pool, Sqlite};

use crate::game::{game::Game, lobby::Lobby};

pub struct AppState {
    pub games: Arc<Mutex<Vec<Game>>>,
    pub lobbies: Arc<Mutex<Vec<Lobby>>>,
    pub db_conn_pool: Pool<Sqlite>,
}

impl AppState {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self {
            games: Arc::new(Mutex::new(vec![])),
            lobbies: Arc::new(Mutex::new(vec![])),
            db_conn_pool: pool,
        }
    }

    pub fn get_games(&self) -> MutexGuard<'_, Vec<Game>> {
        self.games.lock().expect("mutex was poisoned")
    }

    pub fn get_lobbies(&self) -> MutexGuard<'_, Vec<Lobby>> {
        self.lobbies.lock().expect("mutex was poisoned")
    }
}
