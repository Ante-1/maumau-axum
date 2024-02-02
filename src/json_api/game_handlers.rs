use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{
    app_state::AppState,
    auth::user::AuthSession,
    game::{
        game::{CreateGame, CreateGameResponse, PlayCardPayload},
        game_handler_helpers::{create_game, get_game_state},
    },
};

pub async fn create_game_handler(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateGame>,
) -> Response {
    let lobby_id = payload.lobby_id;

    let new_game_id = match create_game(state, lobby_id) {
        Ok(new_game_id) => new_game_id,
        Err(error_response) => return error_response,
    };
    (
        StatusCode::CREATED,
        Json(CreateGameResponse {
            game_id: new_game_id,
        }),
    )
        .into_response()
}

pub async fn get_game_state_handler(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    auth_session: AuthSession,
) -> Response {
    let game_state = match get_game_state(auth_session, state, game_id) {
        Ok(value) => value,
        Err(value) => return value,
    };

    Json(game_state).into_response()
}

pub async fn play_card(
    State(state): State<Arc<AppState>>,
    Path(game_id): Path<i64>,
    auth_session: AuthSession,
    Json(payload): Json<PlayCardPayload>,
) -> Response {
    let user_id = match auth_session.user {
        Some(user) => user.id,
        None => return (StatusCode::UNAUTHORIZED, "unauthorized").into_response(),
    };
    let card = match payload.card.to_card() {
        Ok(card) => card,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid card").into_response(),
    };
    {
        let mut games = state.get_games();
        let game = games.iter_mut().find(|game| game.id == game_id);
        let game = match game {
            Some(game) => game,
            None => return (StatusCode::NOT_FOUND, "game not found").into_response(),
        };
        if game.current_turn_player != user_id {
            return (StatusCode::BAD_REQUEST, "not your turn").into_response();
        }

        let player = game
            .players
            .iter_mut()
            .find(|player| player.lobby_player.user_id == user_id);
        let player = match player {
            Some(player) => player,
            None => return (StatusCode::BAD_REQUEST, "player not in game").into_response(),
        };

        match player.remove_card(&card) {
            Ok(_) => {}
            Err(_) => return (StatusCode::BAD_REQUEST, "card not in hand").into_response(),
        }
    }

    let mut games = state.get_games();
    let game = match games.iter_mut().find(|game| game.id == game_id) {
        Some(game) => game,
        None => return (StatusCode::NOT_FOUND, "game not found").into_response(),
    };

    match game.play_card(card) {
        Ok(_) => {}
        Err(_) => return (StatusCode::BAD_REQUEST, "cannot play card").into_response(),
    };

    let player = game
        .players
        .iter()
        .find(|player| player.lobby_player.user_id == user_id)
        .unwrap();

    if player.hand.is_empty() {
        game.winner = Some(user_id);
    }

    game.next_player();

    // // todo: do i want to return more?
    // // todo: do i want to make game copy and give easy acces methods to gamestate parts?
    // // todo: add play card test
    (StatusCode::OK, "card played").into_response()
}
