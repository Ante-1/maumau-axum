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
    game::lobby::{CreateLobby, JoinLobby, Lobby},
};

use super::lobby::LobbyPlayer;

pub async fn create_lobby(
    State(state): State<Arc<AppState>>,
    auth_session: AuthSession,
    Json(playload): Json<CreateLobby>,
) -> Response {
    let lobby_name = &playload.name;
    let lobby = match create_new_lobby(auth_session, state, lobby_name) {
        Ok(value) => value,
        Err(value) => return value,
    };

    (StatusCode::CREATED, Json(lobby)).into_response()
}

pub fn create_new_lobby(
    auth_session: AuthSession,
    state: Arc<AppState>,
    lobby_name: &str,
) -> Result<Lobby, axum::http::Response<axum::body::Body>> {
    if auth_session.user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized").into_response());
    }
    let user = auth_session.user.unwrap();
    let mut new_lobby_id: i64 = rand::random();
    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    while lobbies.iter().any(|l| l.id == new_lobby_id) {
        new_lobby_id = rand::random();
    }
    let lobby = Lobby {
        id: new_lobby_id,
        name: lobby_name.to_owned(),
        players: vec![LobbyPlayer {
            user_id: user.id,
            username: user.username,
        }],
    };
    lobbies.push(lobby.clone());
    Ok(lobby)
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
    // let lobby_id = payload.lobby_id;
    // if auth_session.user.is_none() {
    //     return (StatusCode::UNAUTHORIZED, "unauthorized");
    // }
    // let user = auth_session.user.unwrap();

    // let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");

    // let lobby = lobbies.iter_mut().find(|lobby| lobby.id == lobby_id);

    // if lobby.is_none() {
    //     return (StatusCode::NOT_FOUND, "lobby not found");
    // }

    // lobby.unwrap().user_ids.push(user.id);
    // (StatusCode::OK, "player joined lobby")
    todo!()
}
