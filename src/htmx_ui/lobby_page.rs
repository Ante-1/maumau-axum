use askama::Template;

use crate::game::lobby::Lobby;

#[derive(Template)]
#[template(path = "lobby.html")]
pub struct LobbyTemplate {
    lobby: Lobby,
    players_route: String,
    is_lobby_owner: bool,
    not_joined: bool,
}

#[derive(Template)]
#[template(path = "player-list.html")]
pub struct PlayersTemplate {
    players: Vec<String>,
}

pub mod get {
    use std::sync::Arc;

    use askama_axum::{IntoResponse, Response};
    use axum::{
        extract::{Path, State},
        http::StatusCode,
    };

    use crate::{app_state::AppState, auth::user::AuthSession};

    use super::*;

    pub async fn lobby(
        Path(lobby_id): Path<i64>,
        State(state): State<Arc<AppState>>,
        auth_session: AuthSession,
    ) -> Response {
        let lobbies = state.get_lobbies();
        let lobby = match lobbies.iter().find(|l| l.id == lobby_id) {
            Some(value) => value.clone(),
            None => return (StatusCode::NOT_FOUND, "Not Found").into_response(),
        };
        let user = match auth_session.user {
            Some(value) => value,
            None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
        };
        let is_lobby_owner = lobby.players[0].user_id == user.id;
        let not_joined = !lobby.players.iter().any(|p| p.user_id == user.id);

        LobbyTemplate {
            players_route: format!("/lobbies/{}/players", lobby_id),
            lobby,
            is_lobby_owner,
            not_joined,
        }
        .into_response()
    }

    pub async fn lobby_players(
        Path(lobby_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> Response {
        let lobbies = state.get_lobbies();
        let lobby = match lobbies.iter().find(|l| l.id == lobby_id) {
            Some(value) => value.clone(),
            None => return (StatusCode::NOT_FOUND, "Not Found").into_response(),
        };

        let players = lobby.players;

        let players: Vec<String> = players.iter().map(|p| p.username.clone()).collect();

        PlayersTemplate { players }.into_response()
    }
}

pub mod post {
    use std::sync::Arc;

    use axum::{
        extract::{Path, State},
        http::StatusCode,
        response::{IntoResponse, Redirect},
    };

    use crate::{
        app_state::AppState,
        auth::user::AuthSession,
        game::{lobby::LobbyPlayer, lobby_routes::create_new_lobby},
    };

    pub async fn create_lobby(
        State(state): State<Arc<AppState>>,
        auth_session: AuthSession,
    ) -> impl IntoResponse {
        let username = match auth_session.clone().user {
            Some(value) => value.username,
            None => return Redirect::to("/login").into_response(),
        };
        let lobby = match create_new_lobby(
            auth_session,
            state,
            format!("{}'s lobby", username).as_str(),
        ) {
            Ok(value) => value,
            Err(value) => return value,
        };
        ([("HX-Redirect", format!("/lobbies/{}", lobby.id))]).into_response()
    }

    pub async fn join_lobby(
        Path(lobby_id): Path<i64>,
        State(state): State<Arc<AppState>>,
        auth_session: AuthSession,
    ) -> impl IntoResponse {
        let mut lobbies = state.get_lobbies();
        let lobby = match lobbies.iter_mut().find(|l| l.id == lobby_id) {
            Some(value) => value,
            None => return (StatusCode::NOT_FOUND, "Not Found").into_response(),
        };
        let user = match auth_session.user {
            Some(value) => value,
            None => return (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
        };
        lobby.players.push(LobbyPlayer {
            user_id: user.id,
            username: user.username,
        });
        ([("HX-Redirect", format!("/lobbies/{}", lobby.id))]).into_response()
    }
}
