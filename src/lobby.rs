use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lobby {
    pub player_ids: Vec<u64>,
    pub name: String,
    pub id: u64,
}

#[derive(Deserialize)]
pub struct CreateLobby {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinLobby {
    pub player_id: u64,
    pub lobby_id: u64,
}
