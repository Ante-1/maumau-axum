use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::sync::Arc;

use crate::{app_state::AppState, auth::user::AuthSession};

use super::{
    card::CardDTO,
    game::{CurrentPlayerGameState, Game},
    player::PlayerDTO,
};

pub fn create_game(state: Arc<AppState>, lobby_id: i64) -> Result<i64, Response> {
    let mut games = state.get_games();
    let mut lobbies = state.get_lobbies();

    let lobby = match lobbies.iter_mut().find(|lobby| lobby.id == lobby_id) {
        Some(value) => value,
        None => return Err((StatusCode::NOT_FOUND, "lobby not found").into_response()),
    };

    if lobby.players.len() < 2 {
        return Err((StatusCode::BAD_REQUEST, "not enough players").into_response());
    }

    let mut new_game_id: i64 = rand::random();

    while new_game_id < 0 || games.iter().any(|game| game.id == new_game_id) {
        new_game_id = rand::random();
    }

    let mut game = Game::new(lobby.players.clone(), lobby.id, new_game_id);
    lobby.running_game = Some(new_game_id);
    game.give_cards();
    game.turn_top_card();

    games.push(game);
    Ok(new_game_id)
}

pub fn get_game_state(
    auth_session: AuthSession,
    state: Arc<AppState>,
    game_id: i64,
) -> Result<CurrentPlayerGameState, Response> {
    if auth_session.user.is_none() {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized").into_response());
    }
    let user_id = auth_session.user.unwrap().id;
    {
        let mut games = state.get_games();
        let game = match games.iter_mut().find(|game| game.id == game_id) {
            Some(value) => value,
            None => return Err((StatusCode::NOT_FOUND, "game not found").into_response()),
        };

        if !game
            .players
            .iter()
            .any(|player| player.lobby_player.user_id == user_id)
        {
            return Err((StatusCode::BAD_REQUEST, "player not in game").into_response());
        }
    }
    let mut games = state.get_games();
    let game = match games.iter_mut().find(|game| game.id == game_id) {
        Some(value) => value,
        None => return Err((StatusCode::NOT_FOUND, "game not found").into_response()),
    };
    let player = match game
        .players
        .iter()
        .find(|player| player.lobby_player.user_id == user_id)
    {
        Some(value) => value,
        None => return Err((StatusCode::BAD_REQUEST, "player not found").into_response()),
    };
    let hand: Vec<CardDTO> = player.hand.iter().map(|card| card.to_dto()).collect();
    let played_cards: Vec<CardDTO> = game.played_cards.iter().map(|card| card.to_dto()).collect();
    let opponents = game
        .players
        .iter()
        .filter(|player| player.lobby_player.user_id != user_id)
        .map(|player| PlayerDTO {
            user_id: player.lobby_player.user_id,
            username: player.lobby_player.username.clone(),
            hand_size: player.hand.len(),
        })
        .collect::<Vec<_>>();
    let game_state = CurrentPlayerGameState {
        game_id,
        hand,
        current_player: game.current_turn_player,
        played_cards,
        opponents,
        winner: game.winner,
        deck_size: game.deck_size(),
    };
    Ok(game_state)
}
