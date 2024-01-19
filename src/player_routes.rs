use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state::AppState,
    player::{CreatePlayer, Player, PlayerDTO},
};

pub async fn create_player(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreatePlayer>,
) -> impl IntoResponse {
    let mut players = state.players.lock().expect("mutex was poisoned");
    let mut random_id: u64 = rand::random();
    while players.iter().any(|p| p.id == random_id) {
        random_id = rand::random();
    }

    let player = PlayerDTO {
        id: random_id,
        name: payload.name,
    };

    players.push(Player::new(player.id, player.name.clone()));

    (StatusCode::CREATED, Json(player))
}

pub async fn get_players(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let players: Vec<PlayerDTO> = state
        .players
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .map(|player| player.to_dto())
        .collect();

    Json(players)
}
