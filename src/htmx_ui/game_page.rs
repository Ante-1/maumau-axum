use askama::Template;

use crate::game::{card::CardDTO, player::PlayerDTO};

#[derive(Template)]
#[template(path = "game.html")]
pub struct GameTemplate {
    my_hand: Vec<CardDTO>,
    other_players: Vec<PlayerDTO>,
    current_turn_player: i64,
    is_my_turn: bool,
    winner: Option<PlayerDTO>,
    last_played_card: CardDTO,
}

pub mod get {
    use std::sync::Arc;

    use askama_axum::IntoResponse;
    use axum::extract::{Path, State};
    use axum::response::Response;

    use crate::{
        app_state::AppState,
        auth::user::AuthSession,
        game::{game_routes::get_game_state, player::PlayerDTO},
    };

    use super::GameTemplate;

    pub async fn game_handler(
        auth_session: AuthSession,
        Path(game_id): Path<i64>,
        State(state): State<Arc<AppState>>,
    ) -> Response {
        let current_player_game_state = match get_game_state(auth_session.clone(), state, game_id) {
            Ok(value) => value,
            Err(value) => return value,
        };
        let current_turn_player = current_player_game_state.current_player;
        let is_my_turn = current_turn_player == auth_session.user.unwrap().id;
        let winner: Option<PlayerDTO> = current_player_game_state.winner.map(|winner_id| {
            let winner = current_player_game_state
                .opponents
                .iter()
                .find(|player| player.user_id == winner_id)
                .unwrap();
            PlayerDTO {
                user_id: winner.user_id,
                username: winner.username.clone(),
                hand_size: 0,
            }
        });

        GameTemplate {
            my_hand: current_player_game_state.hand,
            other_players: current_player_game_state.opponents,
            current_turn_player,
            is_my_turn,
            winner,
            last_played_card: current_player_game_state
                .played_cards
                .last()
                .unwrap()
                .clone(),
        }
        .into_response()
    }
}

pub mod post {
    use std::sync::Arc;

    use axum::extract::{Path, State};
    use axum::http::StatusCode;
    use axum::response::IntoResponse;

    use crate::{app_state::AppState, game::game_routes::create_game};

    pub async fn create_game_handler(
        State(state): State<Arc<AppState>>,
        Path(lobby_id): Path<i64>,
    ) -> impl IntoResponse {
        let new_game_id = match create_game(state, lobby_id) {
            Ok(new_game_id) => new_game_id,
            Err(error_response) => return error_response,
        };
        (
            [("HX-Redirect", format!("/games/{}", new_game_id))],
            StatusCode::CREATED,
        )
            .into_response()
    }
}
