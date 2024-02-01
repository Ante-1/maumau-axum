use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::app_state::AppState;

use super::{
    game_routes::{create_game_handler, get_game_state_handler, play_card},
    lobby_routes::{create_lobby_handler, get_lobbies, join_lobby},
};
pub fn game_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/lobbies", post(create_lobby_handler))
        .route("/lobbies", get(get_lobbies))
        .route("/lobbies/join", post(join_lobby))
        .route("/games", post(create_game_handler))
        .route("/games/:game_id", post(get_game_state_handler))
        .route("/games/:game_id/play-card", post(play_card))
}
