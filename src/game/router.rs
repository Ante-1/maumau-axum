use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::app_state::AppState;

use super::{
    game_routes::{create_game, get_game_state, play_card},
    lobby_routes::{create_lobby, get_lobbies, join_lobby},
    player_routes::{create_player, get_players},
};
pub fn game_router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/players", post(create_player))
        .route("/players", get(get_players))
        .route("/lobbies", post(create_lobby))
        .route("/lobbies", get(get_lobbies))
        .route("/lobbies/join", post(join_lobby))
        .route("/games", post(create_game))
        .route("/games/:game_id", post(get_game_state))
        .route("/games/:game_id/play-card", post(play_card))
}
