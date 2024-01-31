use askama::Template;

use crate::game::lobby::Lobby;

#[derive(Template)]
#[template(path = "lobby.html")]
pub struct LobbyTemplate {
    lobby: Lobby,
    players: Vec<String>,
}

pub mod get {
    use std::sync::Arc;

    use askama_axum::{IntoResponse, Response};
    use axum::{
        extract::{Path, State},
        http::StatusCode,
    };

    use crate::app_state::AppState;

    use super::*;

    pub async fn lobby(Path(lobby_id): Path<i64>, State(state): State<Arc<AppState>>) -> Response {
        let lobbies = state.get_lobbies();
        let lobby = match lobbies.iter().find(|l| l.id == lobby_id) {
            Some(value) => value.clone(),
            None => return (StatusCode::NOT_FOUND, "Not Found").into_response(),
        };

        LobbyTemplate {
            players: lobby.players.iter().map(|p| p.username.clone()).collect(),
            lobby,
        }
        .into_response()
    }
}

pub mod post {
    use std::sync::Arc;

    use axum::{
        extract::State,
        response::{IntoResponse, Redirect},
    };

    use crate::{
        app_state::AppState, auth::user::AuthSession, game::lobby_routes::create_new_lobby,
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
        Redirect::to(format!("/lobbies/{}", lobby.id).as_str()).into_response()
    }
}
