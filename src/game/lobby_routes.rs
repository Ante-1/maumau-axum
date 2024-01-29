use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state::AppState,
    game::lobby::{CreateLobby, JoinLobby, Lobby},
};

pub async fn create_lobby(
    State(state): State<Arc<AppState>>,
    Json(playload): Json<CreateLobby>,
) -> impl IntoResponse {
    let mut random_id: u64 = rand::random();
    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    while lobbies.iter().any(|l| l.id == random_id) {
        random_id = rand::random();
    }

    let lobby = Lobby {
        id: random_id,
        name: playload.name,
        player_ids: vec![],
    };

    lobbies.push(lobby.clone());

    (StatusCode::CREATED, Json(lobby))
}

pub async fn get_lobbies(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let lobbies: Vec<Lobby> = state
        .lobbies
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .cloned()
        .collect();

    Json(lobbies)
}

pub async fn join_lobby(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<JoinLobby>,
) -> impl IntoResponse {
    let lobby_id = payload.lobby_id;
    let player_id = payload.player_id;
    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    let players = state.players.lock().expect("mutex was poisoned");

    let lobby = lobbies.iter_mut().find(|lobby| lobby.id == lobby_id);

    if lobby.is_none() {
        return (StatusCode::NOT_FOUND, "lobby not found");
    }

    let player = players.iter().find(|player| player.id == player_id);

    if player.is_none() {
        return (StatusCode::NOT_FOUND, "player not found");
    }

    lobby.unwrap().player_ids.push(player_id);
    (StatusCode::OK, "player joined lobby")
}
