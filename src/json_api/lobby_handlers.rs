use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    auth::user::AuthSession,
    game::{
        lobby::{CreateLobby, JoinLobby, Lobby},
        lobby_handler_helpers::{create_lobby, join_lobby_helper},
    },
};

pub async fn create_lobby_handler(
    State(state): State<Arc<AppState>>,
    auth_session: AuthSession,
    Json(playload): Json<CreateLobby>,
) -> Response {
    let lobby_name = &playload.name;
    let lobby = match create_lobby(auth_session, state, lobby_name) {
        Ok(value) => value,
        Err(value) => return value,
    };

    (StatusCode::CREATED, Json(lobby)).into_response()
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
    auth_session: AuthSession,
    Json(payload): Json<JoinLobby>,
) -> impl IntoResponse {
    let lobby_id = payload.lobby_id;

    match join_lobby_helper(state, lobby_id, auth_session) {
        Ok(value) => value,
        Err(value) => return value,
    };

    (StatusCode::OK, "player joined lobby").into_response()
}
