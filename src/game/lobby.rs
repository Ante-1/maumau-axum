use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Lobby {
    pub id: i64,
    pub name: String,
    pub user_ids: Vec<i64>,
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
