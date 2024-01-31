use crate::game::lobby::Lobby;
use askama::Template;

use std::sync::Arc;

use axum::{extract::State, response::IntoResponse};

use crate::app_state::AppState;
#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    is_logged_in: bool,
    lobbies: Vec<Lobby>,
}

pub mod get {
    use crate::auth::user::AuthSession;

    use super::*;

    pub async fn index(
        State(state): State<Arc<AppState>>,
        auth_session: AuthSession,
    ) -> impl IntoResponse {
        let is_logged_in = auth_session.user.is_some();
        let lobbies = state.get_lobbies();
        let lobbies: Vec<Lobby> = lobbies.iter().cloned().collect();

        IndexTemplate {
            is_logged_in,
            lobbies,
        }
    }
}
