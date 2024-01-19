use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    pub name: String,
    pub id: u64,
}

#[derive(Deserialize)]
pub struct CreatePlayer {
    pub name: String,
}
