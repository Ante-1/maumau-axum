use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lobby {
    pub id: i64,
    pub name: String,
    pub players: Vec<LobbyPlayer>,
}

#[derive(Deserialize)]
pub struct CreateLobby {
    pub name: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JoinLobby {
    pub lobby_id: i64,
}

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LobbyPlayer {
    pub user_id: i64,
    pub username: String,
}
