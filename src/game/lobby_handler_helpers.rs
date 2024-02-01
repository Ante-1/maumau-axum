use std::sync::Arc;

use crate::{app_state::AppState, auth::user::AuthSession};
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

use super::lobby::{Lobby, LobbyPlayer};

pub fn create_lobby(
    auth_session: AuthSession,
    state: Arc<AppState>,
    lobby_name: &str,
) -> Result<Lobby, Response> {
    if auth_session.user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized").into_response());
    }
    let user = auth_session.user.unwrap();
    let mut new_lobby_id: i64 = rand::random();
    let mut lobbies = state.lobbies.lock().expect("mutex was poisoned");
    while new_lobby_id < 0 || lobbies.iter().any(|l| l.id == new_lobby_id) {
        new_lobby_id = rand::random();
    }
    let lobby = Lobby {
        id: new_lobby_id,
        name: lobby_name.to_owned(),
        players: vec![LobbyPlayer {
            user_id: user.id,
            username: user.username,
        }],
        running_game: None,
    };
    lobbies.push(lobby.clone());
    Ok(lobby)
}

pub fn join_lobby_helper(
    state: Arc<AppState>,
    lobby_id: i64,
    auth_session: AuthSession,
) -> Result<i64, Response> {
    let mut lobbies = state.get_lobbies();
    let lobby = match lobbies.iter_mut().find(|l| l.id == lobby_id) {
        Some(value) => value,
        None => return Err((StatusCode::NOT_FOUND, "Not Found").into_response()),
    };
    let user = match auth_session.user {
        Some(value) => value,
        None => return Err((StatusCode::UNAUTHORIZED, "Unauthorized").into_response()),
    };
    lobby.players.push(LobbyPlayer {
        user_id: user.id,
        username: user.username,
    });
    let lobby_id = lobby.id;
    Ok(lobby_id)
}
