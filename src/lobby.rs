use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Lobby {
    pub player_ids: Vec<u64>,
    pub name: String,
    pub id: u64,
}

#[derive(Deserialize)]
pub struct CreateLobby {
    pub name: String,
}
