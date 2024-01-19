use std::sync::Arc;

use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    app_state::AppState,
    deck::Deck,
    game::{CreateGame, Game, GameResponse},
};

pub async fn create_game(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateGame>,
) -> impl IntoResponse {
    let lobby_id = payload.lobby_id;
    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    let mut games = state.games.lock().expect("mutex was poisoned");

    let lobby = lobbies.iter_mut().find(|lobby| lobby.id == lobby_id);

    if lobby.is_none() {
        return (StatusCode::NOT_FOUND, "lobby not found");
    }

    let lobby = lobby.unwrap();

    if lobby.player_ids.len() < 2 {
        return (StatusCode::BAD_REQUEST, "not enough players");
    }

    let mut random_id: u64 = rand::random();

    while games.iter().any(|game| game.id == random_id) {
        random_id = rand::random();
    }

    let game = Game::new(
        lobby.player_ids.clone(),
        lobby.id,
        random_id,
        Deck::new(),
        vec![],
    );

    games.push(game);

    (StatusCode::CREATED, "game created")
}

pub async fn get_games(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    let games: Vec<GameResponse> = state
        .games
        .lock()
        .expect("mutex was poisoned")
        .iter()
        .map(|game| game.to_dto())
        .collect();

    Json(games)
}
