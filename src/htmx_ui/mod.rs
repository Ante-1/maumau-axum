use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::app_state::AppState;

pub mod index_page;
pub mod lobby_page;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(index_page::get::index))
        .route("/lobbies/:id", get(lobby_page::get::lobby))
        .route("/lobbies", post(lobby_page::post::create_lobby))
        .route("/lobbies/:id/players", get(lobby_page::get::lobby_players))
        .route("/lobbies/:id/players", post(lobby_page::post::join_lobby))
}
