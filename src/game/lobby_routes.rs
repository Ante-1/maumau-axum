use std::sync::Arc;

use axum::{
    debug_handler,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    auth::user::AuthSession,
    game::lobby::{CreateLobby, JoinLobby, Lobby},
};

pub async fn create_lobby(
    State(state): State<Arc<AppState>>,
    auth_session: AuthSession,
    Json(playload): Json<CreateLobby>,
) -> Response {
    if auth_session.user.is_none() {
        return (StatusCode::UNAUTHORIZED, "unauthorized").into_response();
    }
    let user = auth_session.user.unwrap();
    let mut new_lobby_id: i64 = rand::random();
    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    while lobbies.iter().any(|l| l.id == new_lobby_id) {
        new_lobby_id = rand::random();
    }

    let lobby = Lobby {
        id: new_lobby_id,
        name: playload.name,
        user_ids: vec![user.id],
    };

    lobbies.push(lobby.clone());

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
    if auth_session.user.is_none() {
        return (StatusCode::UNAUTHORIZED, "unauthorized");
    }
    let user = auth_session.user.unwrap();

    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");

    let lobby = lobbies.iter_mut().find(|lobby| lobby.id == lobby_id);

    if lobby.is_none() {
        return (StatusCode::NOT_FOUND, "lobby not found");
    }

    lobby.unwrap().user_ids.push(user.id);
    (StatusCode::OK, "player joined lobby")
}
